use std::net::SocketAddr;
use tower::make::Shared;

use hyper::{service::service_fn, Body, Client, Request, Response, Server};

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    client.request(req).await
}

#[tokio::main]
async fn main() {
    let make_service = Shared::new(service_fn(handle));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("Error: {}", e);
    }
} 
