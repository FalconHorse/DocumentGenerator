use std::collections::HashMap;
use std::fmt::Error;
use axum::{Json, Router};
use axum::extract::connect_info::ResponseFuture;
use axum::routing::get;
use axum::routing::post;
use axum::extract::multipart::Multipart;
use axum::http::*;
use axum::response::IntoResponse;
use http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use crate::generator::identify_type_and_transform;

pub fn transform() -> Router {
    Router::new()
        .route("/test", get("Online and working"))
        .route("/transform", post(identify_type_and_transform_handler))
}

async fn identify_type_and_transform_handler(mut multipart: Multipart) -> impl IntoResponse {
    let headers = [
        (CONTENT_TYPE, HeaderValue::from_static("application/zip")),
        (CONTENT_DISPOSITION, HeaderValue::from_static("attachment; filename=\"yourfiles.zip\"")),
    ];
    let result = identify_type_and_transform(multipart).await;
    (StatusCode::OK, headers, result)
}
