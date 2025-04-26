
use actix_web::{HttpServer,App,web};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { //HttpServer::new create a new server
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  //  HttpResponse::Ok().body(req_body)
    HttpResponse::Created().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


