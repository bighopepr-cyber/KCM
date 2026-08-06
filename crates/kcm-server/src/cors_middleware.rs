use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error, HttpResponse,
};
use std::future::{ready, Ready};
use std::pin::Pin;
use std::rc::Rc;

const DEFAULT_METHODS: &str = "GET, POST, PUT, DELETE, OPTIONS";
const DEFAULT_HEADERS: &str = "Content-Type, Authorization, X-Request-ID";
const DEFAULT_MAX_AGE: &str = "3600";

pub struct CorsMiddleware {
    allowed_origins: Vec<String>,
}

impl CorsMiddleware {
    pub fn new() -> Self {
        let allowed_origins = std::env::var("KCM_CORS_ORIGINS")
            .map(|val| {
                val.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        Self { allowed_origins }
    }
}

impl Default for CorsMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for CorsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = CorsMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CorsMiddlewareService {
            service: Rc::new(service),
            allowed_origins: self.allowed_origins.clone(),
        }))
    }
}

pub struct CorsMiddlewareService<S> {
    service: Rc<S>,
    allowed_origins: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for CorsMiddlewareService<S>
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
        let allowed_origins = self.allowed_origins.clone();

        Box::pin(async move {
            let origin = req
                .headers()
                .get("origin")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            // No Origin header — not a CORS request, pass through
            let origin = match origin {
                Some(o) => o,
                None => {
                    let res = svc.call(req).await?;
                    return Ok(res.map_into_left_body());
                }
            };

            // Check if origin is allowed
            if !allowed_origins.is_empty() && !allowed_origins.iter().any(|o| o == origin) {
                let response = HttpResponse::Forbidden()
                    .content_type("application/json")
                    .body(r#"{"error":"Origin not allowed","status":403}"#);
                return Ok(req.into_response(response).map_into_right_body());
            }

            // Handle preflight OPTIONS request
            if req.method() == actix_web::http::Method::OPTIONS {
                let mut response = HttpResponse::Ok();
                response.content_type("application/json");

                if let Ok(val) = HeaderValue::from_str(origin.as_str()) {
                    response.insert_header((
                        HeaderName::from_static("access-control-allow-origin"),
                        val,
                    ));
                }
                if let Ok(val) = HeaderValue::from_static(DEFAULT_METHODS) {
                    response.insert_header((
                        HeaderName::from_static("access-control-allow-methods"),
                        val,
                    ));
                }
                if let Ok(val) = HeaderValue::from_static(DEFAULT_HEADERS) {
                    response.insert_header((
                        HeaderName::from_static("access-control-allow-headers"),
                        val,
                    ));
                }
                if let Ok(val) = HeaderValue::from_static(DEFAULT_MAX_AGE) {
                    response.insert_header((
                        HeaderName::from_static("access-control-max-age"),
                        val,
                    ));
                }
                response.insert_header((
                    HeaderName::from_static("access-control-allow-headers"),
                    "Content-Type, Authorization, X-Request-ID",
                ));

                return Ok(req.into_response(response).map_into_right_body());
            }

            // Non-preflight: add CORS headers to actual response
            let res = svc.call(req).await?;

            let mut response = res.into_response();
            let headers = response.headers_mut();

            if let Ok(val) = HeaderValue::from_str(origin.as_str()) {
                headers.insert(
                    HeaderName::from_static("access-control-allow-origin"),
                    val,
                );
            }
            if let Ok(val) = HeaderValue::from_static(DEFAULT_METHODS) {
                headers.insert(
                    HeaderName::from_static("access-control-allow-methods"),
                    val,
                );
            }
            if let Ok(val) = HeaderValue::from_static(DEFAULT_HEADERS) {
                headers.insert(
                    HeaderName::from_static("access-control-allow-headers"),
                    val,
                );
            }
            if let Ok(val) = HeaderValue::from_static(DEFAULT_MAX_AGE) {
                headers.insert(
                    HeaderName::from_static("access-control-max-age"),
                    val,
                );
            }

            Ok(response.map_into_left_body())
        })
    }
}