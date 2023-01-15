use actix_web::{get, post, body::BoxBody, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
//
// "/index"
//
#[derive(Deserialize, Serialize)]
struct MyResponse {
    name: String,
}
#[derive(Deserialize, Serialize)]
struct MyRequest {
    username: String,
}

// Responder
impl Responder for MyResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
#[post("/index")]
async fn index(data: web::Json<MyRequest>) -> impl Responder {
    MyResponse { name: data.username.clone() }
}
//
// #[get("/")]
//
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
//
// #[get("/echo")]
//
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
//
// #[get("/hey")]
//
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
//
// main
//
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(index)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}