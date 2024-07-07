use axum::body::Body;
use axum::extract::Request;
use axum::response::Response;
use axum::{routing::post, routing::get, Router};
use hyper::Method;
use tower_http::cors::Any;
use crate::handlers::hello_user;
use crate::handlers::get_projects;
use crate::handlers::add_project_to_db;


pub fn router() -> Router {

    let cors = tower_http::cors::CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
    .allow_methods([Method::GET, Method::POST])
    // allow requests from any origin
    .allow_origin(Any)
    .allow_headers(vec![
        "Content-Type".parse().unwrap(),
        // Добавьте другие необходимые заголовки, если таковые имеются
    ]);;

    Router::new()
        .route("/hello", get(hello_user)) 
        .route("/add", post(add_project_to_db)) //
        .route("/projects", get(get_projects))
        .layer(tower::ServiceBuilder::new().layer(cors)) 
}

