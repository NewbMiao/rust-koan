use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    body::{Bytes, Incoming},
    server::conn::http1,
    Request, Response,
};

use http_body_util::Full;
use hyper_util::{rt::TokioIo, service::TowerToHyperService};
use tokio::net::TcpListener;
use tower::{timeout::Timeout, Layer, ServiceBuilder};

/// a simple tower Logger middleware
use tower::Service;

#[derive(Debug, Clone)]
pub struct Logger<S> {
    inner: S,
}
impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}

struct LoggerLayer;
impl<S> Layer<S> for LoggerLayer {
    type Service = Logger<S>;
    fn layer(&self, inner: S) -> Self::Service {
        Logger { inner }
    }
}
type Req = hyper::Request<Incoming>;
impl<S> Service<Req> for Logger<S>
where
    S: Service<Req> + Clone,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        println!("processing request: {} {}", req.method(), req.uri().path());
        self.inner.call(req)
    }
}

async fn handler(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::spawn(async move {
            let svc = tower::service_fn(handler);
            let svc = ServiceBuilder::new()
                .layer(LoggerLayer)
                .layer_fn(|s| Timeout::new(s, std::time::Duration::from_secs(5)))
                .service(svc);
            let svc = TowerToHyperService::new(svc);
            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                eprintln!("server error: {}", err);
            }
        });
    }
}
