use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use la_transpiler::transpiler::lib_hello;
use la_transpiler::transpiler::{
    tokenize_string,
    ast_string,
    latex_string
};
use req_content::ReqContent;

mod req_content;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(lib_hello())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/tokenize")] 
async fn tokenize(req_body: web::Json<ReqContent>) -> impl Responder {
    HttpResponse::Ok().body(tokenize_string(req_body.content.clone()))
}

#[post("/ast")] 
async fn ast(req_body: web::Json<ReqContent>) -> impl Responder {
    HttpResponse::Ok().body(ast_string(req_body.content.clone()))
}

#[post("/latex")] 
async fn latex(req_body: web::Json<ReqContent>) -> impl Responder {
    HttpResponse::Ok().body(latex_string(req_body.content.clone()))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(tokenize)
            .service(ast)
            .service(latex)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}