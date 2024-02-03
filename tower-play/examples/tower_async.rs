use hyper::body::Incoming;

#[derive(Debug, Clone)]
pub struct Logger<S> {
    inner: S,
}
impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}
type Req = hyper::Request<Incoming>;
trait ServiceAsync<Request> {
    type Response;
    type Error;
    async fn call(&mut self, req: Request) -> Result<Self::Response, Self::Error>;
}

impl<S> ServiceAsync<Req> for Logger<S>
where
    S: ServiceAsync<Req> + Clone,
{
    type Response = S::Response;
    type Error = S::Error;
    async fn call(&mut self, req: Req) -> Result<Self::Response, Self::Error> {
        println!("processing request: {} {}", req.method(), req.uri().path());
        self.inner.call(req).await
    }
    /* Before 1.75.0, the following error was reported:
     error[E0706]: functions in traits cannot be declared `async`
    = note: `async` trait functions are not currently supported
    = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
      */
}
fn main() {}
