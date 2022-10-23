use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, http::header::HeaderName,
};
use futures_util::future::LocalBoxFuture;
use crate::utilities::verify_jwt;

pub struct PermissionCheck;

impl<S, B> Transform<S, ServiceRequest> for PermissionCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let token = req.headers().get(HeaderName::from_static("authorization"));
        if token.is_none() {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
            });
        }

        let token = token.unwrap().to_str().unwrap().to_string();
        let token = token.replace("Bearer ", "");
        let user_id = verify_jwt(token);
        if user_id.to_lowercase() == "invalid token" {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
            });
        }

        req.headers_mut().insert(HeaderName::from_lowercase(b"userid").unwrap(), user_id.parse().unwrap());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}