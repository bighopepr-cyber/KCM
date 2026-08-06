use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use kcm_core::types::ContextID;
use kcm_interface::middleware::auth::CredentialStore;
use kcm_security::rbac::{ACLManager, Permission};
use std::future::{ready, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

const DEFAULT_CONTEXT: ContextID = ContextID(0);

pub struct AuthConfig {
    pub credential_store: Arc<CredentialStore>,
    pub acl_manager: Arc<ACLManager>,
    pub enabled: bool,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        let enabled = std::env::var("KCM_AUTH_ENABLED")
            .map(|v| v != "false" && v != "0")
            .unwrap_or(true);

        let store = CredentialStore::from_env();
        let acl = Self::init_acl_manager(&store);

        if enabled && store.is_empty() {
            log::warn!(
                "Auth middleware enabled but no tokens configured. \
                 Set KCM_AUTH_TOKENS or KCM_AUTH_ENABLED=false for development."
            );
        }

        Self {
            credential_store: Arc::new(store),
            acl_manager: Arc::new(acl),
            enabled,
        }
    }

    fn init_acl_manager(store: &CredentialStore) -> ACLManager {
        let acl = ACLManager::new();

        let _ = acl.create_role("reader");
        let _ = acl.add_permission_to_role("reader", Permission::Read);

        let _ = acl.create_role("writer");
        let _ = acl.add_permission_to_role("writer", Permission::Read);
        let _ = acl.add_permission_to_role("writer", Permission::Write);

        let _ = acl.create_role("admin");
        let _ = acl.add_permission_to_role("admin", Permission::Read);
        let _ = acl.add_permission_to_role("admin", Permission::Write);
        let _ = acl.add_permission_to_role("admin", Permission::Delete);
        let _ = acl.add_permission_to_role("admin", Permission::Execute);
        let _ = acl.add_permission_to_role("admin", Permission::Admin);

        for entry in store.entries().values() {
            let _ = acl.create_user(&entry.user_id);
            for role in &entry.roles {
                let _ = acl.assign_role(&entry.user_id, role);
            }
        }

        acl
    }
}

pub struct AuthGuard {
    config: Arc<AuthConfig>,
}

impl AuthGuard {
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self { config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthGuardService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthGuardService {
            service: Rc::new(service),
            config: self.config.clone(),
        }))
    }
}

pub struct AuthGuardService<S> {
    service: Rc<S>,
    config: Arc<AuthConfig>,
}

fn requires_auth(path: &str) -> bool {
    path.starts_with("/api/v1/facts") || path.starts_with("/facts")
}

fn extract_token(req: &ServiceRequest) -> Option<String> {
    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let token = token.trim();
                if !token.is_empty() {
                    return Some(token.to_string());
                }
            }
        }
    }

    if let Some(api_key) = req.headers().get("x-api-key") {
        if let Ok(key) = api_key.to_str() {
            let key = key.trim();
            if !key.is_empty() {
                return Some(key.to_string());
            }
        }
    }

    None
}

fn required_permission(method: &actix_web::http::Method) -> Permission {
    match *method {
        actix_web::http::Method::GET
        | actix_web::http::Method::HEAD
        | actix_web::http::Method::OPTIONS => Permission::Read,
        _ => Permission::Write,
    }
}

impl<S, B> Service<ServiceRequest> for AuthGuardService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let config = self.config.clone();

        Box::pin(async move {
            if !config.enabled {
                let res = svc.call(req).await?;
                return Ok(res.map_into_left_body());
            }

            let path = req.path().to_string();
            if !requires_auth(&path) {
                let res = svc.call(req).await?;
                return Ok(res.map_into_left_body());
            }

            let token = match extract_token(&req) {
                Some(t) => t,
                None => {
                    let response = HttpResponse::Unauthorized()
                        .content_type("application/json")
                        .body(r#"{"error":"Missing authentication credentials","status":401}"#);
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            let entry = match config.credential_store.lookup(&token) {
                Some(e) => e,
                None => {
                    log::warn!("Auth denied: invalid token path={}", path);
                    let response = HttpResponse::Unauthorized()
                        .content_type("application/json")
                        .body(r#"{"error":"Invalid API key","status":401}"#);
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            let user_id = entry.user_id.clone();

            let required = required_permission(req.method());
            let allowed =
                config
                    .acl_manager
                    .check_permission_level(&user_id, DEFAULT_CONTEXT, required);

            if !allowed {
                log::warn!(
                    "RBAC denied: user={} required={} method={} path={}",
                    user_id,
                    required.name(),
                    req.method(),
                    path
                );
                let response = HttpResponse::Forbidden()
                    .content_type("application/json")
                    .body(format!(
                        r#"{{"error":"Insufficient permissions: requires {} access","status":403}}"#,
                        required.name()
                    ));
                return Ok(req.into_response(response).map_into_right_body());
            }

            log::debug!(
                "Auth OK: user={} method={} path={}",
                user_id,
                req.method(),
                path
            );

            let res = svc.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
