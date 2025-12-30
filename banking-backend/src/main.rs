use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;
use chrono::Utc;

#[derive(Serialize)]
struct Transaction {
    id: String,
    merchant: String,
    amount: f64,
    date: String,
    category: String,
    icon: String, // Just a string identifier for the frontend to pick an icon
}

#[derive(Serialize)]
struct AccountData {
    balance: f64,
    currency: String,
    account_number: String,
    holder: String,
    income: f64,
    expense: f64,
    transactions: Vec<Transaction>,
}

#[get("/api/data")]
async fn get_dashboard_data() -> impl Responder {
    // SIMULATED DATABASE RESPONSE
    let data = AccountData {
        balance: 142592.00,
        currency: "USD".to_string(),
        account_number: "4921".to_string(), // Last 4 digits
        holder: "Admin User".to_string(),
        income: 8240.00,
        expense: 3450.00,
        transactions: vec![
            Transaction {
                id: Uuid::new_v4().to_string(),
                merchant: "Apple Store".to_string(),
                amount: 299.00,
                date: Utc::now().to_rfc3339(),
                category: "Electronics".to_string(),
                icon: "tech".to_string(),
            },
            Transaction {
                id: Uuid::new_v4().to_string(),
                merchant: "Uber".to_string(),
                amount: 45.50,
                date: Utc::now().to_rfc3339(),
                category: "Transport".to_string(),
                icon: "car".to_string(),
            },
            Transaction {
                id: Uuid::new_v4().to_string(),
                merchant: "Starbucks".to_string(),
                amount: 12.00,
                date: Utc::now().to_rfc3339(),
                category: "Food".to_string(),
                icon: "coffee".to_string(),
            },
        ],
    };

    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    println!("🔒 Banking Core (Private) running on port 8080");

    HttpServer::new(|| {
        App::new().service(get_dashboard_data)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}