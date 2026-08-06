use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpResponse,
};
use kcm_interface::middleware::rate_limit::RateLimiter;
use std::future::{ready, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

pub struct RateLimitGuard;

impl RateLimitGuard {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RateLimitGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = RateLimitGuardService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitGuardService {
            service: Rc::new(service),
        }))
    }
}

pub struct RateLimitGuardService<S> {
    service: Rc<S>,
}

fn extract_client_id(req: &ServiceRequest) -> String {
    // Check request extensions for user identity set by auth middleware
    if let Some(user_id) = req.extensions().get::<String>().filter(|s| !s.is_empty()) {
        return user_id.clone();
    }

    // Fall back to IP-based identification
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(val) = forwarded.to_str() {
            if let Some(first) = val.split(',').next() {
                let ip = first.trim();
                if !ip.is_empty() {
                    return format!("ip:{}", ip);
                }
            }
        }
    }

    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(val) = real_ip.to_str() {
            let ip = val.trim();
            if !ip.is_empty() {
                return format!("ip:{}", ip);
            }
        }
    }

    match req.peer_addr() {
        Some(addr) => format!("ip:{}", addr.ip()),
        None => "unknown".to_string(),
    }
}

impl<S, B> Service<ServiceRequest> for RateLimitGuardService<S>
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

        Box::pin(async move {
            let rate_limiter = match req.app_data::<web::Data<Arc<RateLimiter>>>() {
                Some(rl) => rl,
                None => {
                    log::error!("RateLimiter not found in app_data");
                    let response = HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(r#"{"error":"Rate limiter not configured","status":500}"#);
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            let client_id = extract_client_id(&req);

            if !rate_limiter.allow(&client_id) {
                log::warn!(
                    "Rate limit exceeded: client_id={} method={} path={}",
                    client_id,
                    req.method(),
                    req.path()
                );
                let response = HttpResponse::TooManyRequests()
                    .content_type("application/json")
                    .body(r#"{"error":"Rate limit exceeded","status":429}"#);
                return Ok(req.into_response(response).map_into_right_body());
            }

            let res = svc.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
