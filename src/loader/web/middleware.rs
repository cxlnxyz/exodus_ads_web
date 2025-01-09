use actix_service::{Service, Transform};
use actix_session::UserSession;
use actix_web::body::EitherBody;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use futures::future::{ok, Ready};
use futures::Future;
use std::fs::read_to_string;
use std::pin::Pin;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let is_logged_in = session.get::<String>("user").unwrap_or(None).is_some();

        if is_logged_in {
            if req.path() == "/login" {
                let response = HttpResponse::Found().header("Location", "/dashboard").finish();
                Box::pin(async move { Ok(req.into_response(response.map_into_right_body())) })
            } else {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            }
        } else {
            if req.path() == "/login" {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            } else {
                Box::pin(async move {
                    let (http_request, _payload) = req.into_parts();
                    let content = read_to_string("src/sites/unauthorized.html").unwrap();
                    let response = HttpResponse::Unauthorized()
                        .content_type("text/html")
                        .body(content)
                        .map_into_right_body();
                    Ok(ServiceResponse::new(http_request, response))
                })
            }
        }
    }
}