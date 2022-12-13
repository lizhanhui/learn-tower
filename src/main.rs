use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

use log::{debug, info};

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World")))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Log example");

    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async {
        let svc = learn_rust::hello_world::HelloWorld {};
        let svc = learn_rust::log::LogService::new(svc);
        let svc = learn_rust::log::LogService::new(svc);

        let svc = learn_rust::log::LogService::new(svc);
        Ok::<_, Infallible>(/* service_fn(handle) */ svc)
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
