use askama::Template;
use lambda_http::{run, service_fn, Body, Error, Request, Response};

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let resp;
    let mut content_type = "text/html";

    match event.uri().path() {
        "/public/main.css" => {
            content_type = "text/css";
            resp = include_str!("../public/main.css").into();
        }
        "/support" => {
            let support = SupportTemplate {};
            resp = support.render().unwrap();
        }
        "/about" => {
            let about = AboutTemplate {};
            resp = about.render().unwrap();
        }
        _ => {
            let home = HomeTemplate {};
            resp = home.render().unwrap();
        }
    }

    let response = Response::builder()
        .status(200)
        .header("content-type", content_type)
        .body(resp.into())
        .map_err(Box::new)?;
    Ok(response)
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

#[derive(Template)]
#[template(path = "support.html")]
struct SupportTemplate {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(handler)).await
}
