use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
extern crate dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct Student {
    id: i64,
    name: Option<String>,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/get_json")]
async fn get_json() -> impl Responder {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:root@localhost:5440/root")
        .await
        .unwrap();

    let student = sqlx::query_as!(Student, "select * from student where id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    let serialized = serde_json::to_string(&student).unwrap();
    format!("serialized: {:?}", serialized);
    HttpResponse::Ok().body(serialized)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(get_json))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
