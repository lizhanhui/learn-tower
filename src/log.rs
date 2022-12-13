use std::task::{Context, Poll};

use futures::future::BoxFuture;
use hyper::Request;
use log::trace;
use tower::Service;

#[derive(Debug, Clone)]
pub struct LogService<T> {
    inner: T,
}

impl<T> LogService<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<B, T> Service<Request<B>> for LogService<T>
where
    T: Service<Request<B>> + Send + Clone + 'static,
    T::Future: Send,
    B: Send + 'static,
{
    type Response = T::Response;
    type Error = T::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let method = req.method().clone();
        let path = req.uri().path().to_owned();
        let mut inner = self.inner.clone();
        Box::pin(async move {
            trace!("Accepted a request: {} {}", method, path);
            let response = inner.call(req).await;
            trace!("Finished processing request: {} {}", method, path);
            response
        })
    }
}
