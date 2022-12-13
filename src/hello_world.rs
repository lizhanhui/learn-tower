use std::task::{Context, Poll};

use futures::future::{ready, Ready};

use hyper::{http, Body, Request, Response};
use log::trace;
use tower::Service;

pub struct HelloWorld {}

impl Service<Request<Body>> for HelloWorld {
    type Response = Response<Body>;
    type Error = http::Error;

    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let method = req.method().clone();
        let path = req.uri().path();
        trace!("Received a request: {} - {}", method, path);
        // create the body
        let body = Body::from("foo, bar, baz!\n");
        let response = Response::new(body);
        ready(Ok(response))
    }
}
