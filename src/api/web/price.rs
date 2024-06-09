use crate::{
  infra::error::{AppError, AppResult},
  module::{self, Frugal},
};
use askama::Template;
use axum::{response::Html, routing::post, Form, Router};
use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize)]
struct Run {
  cmd: String,
}

#[derive(Template)]
#[template(path = "comps/output.html")]
struct OutputTemplate {
  result: String,
}

async fn handle_run(Form(run): Form<Run>) -> AppResult<Html<String>> {
  let result = Frugal::try_parse_from(run.cmd.split(' '))
    .map_or_else(|e| Err(AppError::Parse(e)), module::run)
    .unwrap_or_else(|e| e.to_string());
  let template = OutputTemplate { result };
  let content = template.render().map_err(AppError::Render)?;

  Ok(Html(content))
}

pub fn route() -> Router {
  Router::new().route("/cmd.run", post(handle_run))
}