use actix_cors::Cors;
use actix_web::{post, web, App, HttpServer, Result, HttpResponse, http,body::BoxBody, http::header::ContentType, HttpRequest, Responder,
};
use actix_web::web::post;

use serde::{Deserialize, Serialize};
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(submit)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Deserialize, Serialize, Clone)]
struct ListingData {
    pub title: String,
    pub desc: String,
    pub req: String,
    pub skills: String,
}

impl Responder for ListingData {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

// TODO: Add data to database

#[post("/submit")]
async fn submit(posting: web::Json<ListingData>) -> impl Responder {
    println!("Desc: {}", posting.desc);
    println!("Title: {}", posting.title);
    println!("Requirements: {}", posting.req);
    println!("Skills: {}", posting.skills);
    let temp = posting.clone();
    ListingData { ..temp }
}