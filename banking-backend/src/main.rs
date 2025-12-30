use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Account {
    id: String,
    balance: f64,
}

#[get("/balance/{id}")]
async fn balance(path: web::Path<String>) -> impl Responder {
    let account = Account { id: path.into_inner(), balance: 1500.00 };
    web::Json(account)
}

#[post("/transfer")]
async fn transfer() -> impl Responder {
    "Transfer completed"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Backend running on port 8080");
    HttpServer::new(|| App::new().service(balance).service(transfer))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
