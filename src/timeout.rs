use std::{
    error::Error,
    fmt::Display,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use pin_project::pin_project;
use tokio::time::sleep;
use tower::{Layer, Service};

#[derive(Debug, Clone)]
#[pin_project]
struct Timeout<T> {
    #[pin]
    inner: T,
    timeout: Duration,
}

impl<T> Timeout<T> {
    pub fn new(inner: T, timeout: Duration) -> Self {
        Self { inner, timeout }
    }
}

#[derive(Debug)]
pub struct Expired {}

impl Display for Expired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request is expired")
    }
}

impl Error for Expired {}

impl<T, Request> Service<Request> for Timeout<T>
where
    T: Service<Request>,
    T::Future: 'static,
    T::Error: Into<Box<dyn Error>> + 'static,
{
    type Response = T::Response;
    type Error = Box<dyn Error>;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let sleep = sleep(self.timeout);
        let fut = self.inner.call(req);
        let f = async move {
            tokio::select! {
                res = fut => {
                    res.map_err(|err|err.into())
                }
                _ = sleep => {
                    Err(Box::new(Expired{}) as Box<dyn Error>)
                }
            }
        };

        Box::pin(f)
    }
}

struct TimeoutLayer(Duration);

impl TimeoutLayer {
    pub fn new(delay: Duration) -> Self {
        Self(delay)
    }
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = Timeout<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Timeout::new(inner, self.0)
    }
}
