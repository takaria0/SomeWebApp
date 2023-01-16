use actix_web::web::Data;
use actix_web::{get, post, guard, body::BoxBody, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Result};
use serde::{Deserialize, Serialize};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

//
//
// "/graphql"
//
//
struct Query ;

#[Object]
impl Query {
    async fn total_photos(&self) -> usize {
        42
    }
    async fn hello(&self) -> String {
        "Hello world!".to_string()
    }
}


// struct Photo {
//     name: String,
//     description: String,
// }

// struct Mutation;

// #[Object]
// impl Mutation {
//     async fn post_photo(&self, name: String, description: String) -> bool {
//         let photo = Photo {
//             name,
//             description
//         };
//         PHOTOS.lock().unwrap().push(photo);
//         true
//     }
//     async fn total_photos(&self) -> usize {
//         PHOTOS.lock().unwrap().len()
//     }
// }

type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
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
#[get("/index")]
async fn get_index() -> impl Responder {
    MyResponse { name: "Hello guys, I am from Rust programming language.".to_string() }
}
#[post("/index")]
async fn post_index(data: web::Json<MyRequest>) -> impl Responder {
    MyResponse { name: data.username.clone() + ", from Rust" }
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
    println!("manual_hello");
    HttpResponse::Ok().body("Hey there!")
}
//
// main
//
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql_index))
            .service(web::resource("/graphql").guard(guard::Get()).to(graphql_index_playground))
            .service(hello)
            .service(echo)
            .service(get_index)
            .service(post_index)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}