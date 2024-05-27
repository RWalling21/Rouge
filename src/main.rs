use std::{env, net::SocketAddr};
use hyper::{service::{make_service_fn, service_fn}, Body, Client, Request, Response, Server};
use hyper::server::conn::AddrStream;

async fn log(req: Request<Body>, remote_addr: SocketAddr) -> Result<Response<Body>, hyper::Error> {
    println!("Client IP: {}", remote_addr.ip());

    handle(req).await
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    client.request(req).await
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <IP> <PORT>", args[0]);
        return;
    }

    let ip: std::net::IpAddr = args[1].parse().expect("Invalid IP address");
    let port: u16 = args[2].parse().expect("Invalid port number");

    let addr = SocketAddr::from((ip, port));

    let make_service = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| log(req, remote_addr)))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("Error: {}", e);
    }
}
