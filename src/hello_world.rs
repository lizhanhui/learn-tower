use std::task::{Context, Poll};

use futures::future::{ready, BoxFuture, Ready};

use hyper::{http, Body, Request, Response};
use log::trace;
use tower::Service;

pub struct HelloWorld {}

impl Service<Request<Body>> for HelloWorld {
    type Response = Response<Body>;
    type Error = http::Error;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let method = req.method().clone();
        let path = req.uri().path().to_owned();
        trace!("Received a request: {} - {}", method, path);
        // Caveat: memory allocation for each request. It's too expensive.
        Box::pin(async move {
            // create the body
            let body = Body::from("foo, bar, baz!\n");
            let response = Response::new(body);
            let resp = ready(Ok(response)).await;
            // In Rust, Futures are lazy!!
            trace!("Finished processing requset: {} - {}", method, path);
            resp
        })
    }
}
