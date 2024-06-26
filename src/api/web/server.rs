use super::home;
use crate::{
  infra::config::Config,
  module::{cmd, money, price},
};
use axum::{
  http::{header::CONTENT_TYPE, Method},
  serve, Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
  classify::{ServerErrorsAsFailures, SharedClassifier},
  cors::{Any, CorsLayer},
  services::ServeDir,
  trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{info, Level};

pub async fn start(config: &Config) {
  info!("Starting the web server!");

  let service_builder = ServiceBuilder::new()
    .layer(trace_layer())
    .layer(cors_layer());
  let router = build_router().layer(service_builder);
  let address = SocketAddr::from(([0, 0, 0, 0], config.server.port));

  tracing::info!("Server is going to listen on address={}", address);

  let listener = tokio::net::TcpListener::bind(address)
    .await
    .expect("Error while binding the address!");

  serve(listener, router)
    .await
    .expect("Failed to start the web server");
}

fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
  TraceLayer::new_for_http()
    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
    .on_request(DefaultOnRequest::new().level(Level::INFO))
    .on_response(DefaultOnResponse::new().level(Level::INFO))
}

pub fn cors_layer() -> CorsLayer {
  CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([CONTENT_TYPE])
    .allow_origin(Any)
}

fn build_router() -> Router {
  Router::new()
    .nest("/", home::route())
    .nest("/", cmd::web::route())
    .nest("/", price::web::route())
    .nest("/", money::web::route())
    .nest_service("/assets", ServeDir::new("assets"))
}
