use std::task::{Context, Poll};

use futures::future::{ready, Ready};

use hyper::{http, Body, Request, Response};
use tower::Service;

#[derive(Debug, Clone)]
pub struct HelloWorld {}

impl Service<Request<Body>> for HelloWorld {
    type Response = Response<Body>;
    type Error = http::Error;

    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        ready(Ok(Response::new(Body::from("Foo, Bar, Baz"))))
    }
}
