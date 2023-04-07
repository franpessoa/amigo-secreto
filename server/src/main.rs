use clap::Parser;
use axum::{
    routing::get,
    response::Html,
    Router
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Parser)]
#[clap(name = "server")]
struct Args {
    #[clap(short = 'a', long = "addr", default_value = "0.0.0.0:8080")]
    addr: String
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let addr = Args::parse().addr;
    let router = Router::new()
        .route("/", get(hello_handler))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));
        
    let server = axum::Server::bind(&addr.parse().unwrap())
        .serve(router.into_make_service());
    
    log::info!("Listening at {addr}");
    
    server
        .await
        .unwrap();
}

async fn hello_handler() -> Html<&'static str> {
    Html("hi")
}