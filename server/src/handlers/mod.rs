use axum::body::Body;
use axum::http;
use axum::{response::IntoResponse, Json};
use serde_json::json;
use tower::layer::util::Identity;
use tower_http::cors::{Any, CorsLayer};
use hyper::{Method, StatusCode};


use http::{Request, Response, header};
//use http_body_util::Full;
//use bytes::Bytes;
use tower::{ServiceBuilder, ServiceExt, Service};
use std::convert::Infallible;

use crate::db::select_projects;
use crate::db::add_project;
use crate::models::User;
use crate::models::Project;


pub async fn hello_user(Json(user): Json<User>) -> impl IntoResponse {
    format!("hello {}", user.name)
}

pub async fn get_projects() -> impl IntoResponse {

    //let projects = vec!["project1", "project2", "project3"]; // Пример данных о проектах

    //let connection = sqlite::open(":memory:").unwrap();


    let projects: Vec<Project> = select_projects().unwrap();

    // Serialize projects to JSON
    //let projects_json = serde_json::to_string(&projects).unwrap();
    //println!("{}", projects_json);

    Json(projects) // Возвращает ответ в формате JSON

}



pub async fn add_project_to_db(req: Request<Body>) -> impl IntoResponse  {


    //println!("we had request: \n {:?}", req);

    // Ваша логика для обработки POST-запроса с данными JSON
    let whole_body = axum::body::to_bytes(req.into_body(), 1000).await.unwrap();
    let body = whole_body.to_vec();

    // Парсинг JSON-данных в структуру Project
    let project: Project = match serde_json::from_slice(&body) {
        Ok(project) => project,
        Err(_) => {
            // Возвращаем ошибку, если не удалось разобрать JSON
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid JSON format"))
                .unwrap();
        }
    };
    println!("calling a func....{:?}", project);
    add_project(project);

    //
    // Возвращаем ответ, например, сообщение об успешном добавлении проекта
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("ok"))
        .unwrap()
}




/*
pub async fn get_projects_with_cors() -> Response {

let cors = CorsLayer::new()
// allow `GET` and `POST` when accessing the resource
.allow_methods([Method::GET, Method::POST])
// allow requests from any origin
.allow_origin(Any);
//let mut service = 
ServiceBuilder::new()
.layer(cors)
.service_fn(get_projects)
}
*/
/*
pub async fn get_projects_with_cors() -> Response<Json<Vec<Project>>> {
    

    let projects: Vec<Project> = select_projects().unwrap();
    Json(projects); // Возвращает ответ в формате JSON

    let mut response = Response::new(Json(projects));

    // Добавляем заголовки CORS к ответу
    let headers = response.headers_mut();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, header::HeaderValue::from_static("*"));
    
    Ok(response)
}*/

/*
// Middleware для добавления заголовков CORS
async fn cors_middleware<S>(mut req: axum::http::Request<Body>, service: S) -> axum::http::Response<Body>
where
    S: Service<axum::http::Request<Body>, Response = axum::http::Response<Body>> + Send + 'static,
    S::Future: Send,
{
    let mut res = service.call(req).await.unwrap();

    // Добавляем заголовки CORS
    res.headers_mut().insert("Access-Control-Allow-Origin", "*".parse().unwrap());

    res
}


// Обработчик для запроса /projects с добавлением заголовков CORS
pub async fn get_projects_with_cors() -> impl IntoResponse {
    // Создаем сервис, который будет включать в себя middleware для CORS
    let service = axum::routing::get(get_projects)
    .layer(axum::AddExtensionLayer::new("cors"));

    // Обертываем сервис в middleware для добавления заголовков CORS
    axum::serve(cors_middleware).boxed_local()
}
*/