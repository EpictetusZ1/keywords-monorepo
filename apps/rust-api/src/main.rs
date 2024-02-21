use actix_cors::Cors;
use actix_web::{post, web, App, HttpServer, Result, HttpResponse, http};
// use actix_web::{web, App, HttpResponse, HttpServer};

use serde::Deserialize;
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        // App::new().route("/", web::get().to(HttpResponse::Ok))
        //

        App::new()
            .wrap(cors)
            .service(submit)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
#[derive(Deserialize)]
struct ListingData {
    pub title: String,
    pub desc: String,
    pub req: String,
    pub skills: String,
}

#[post("/submit")]
async fn submit(posting: web::Json<ListingData>) -> Result<String> {
    Ok(format!("Welcome {}!", posting.desc))
}