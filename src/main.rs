use std::env::var;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");

    let db_url = var("DATABASE_URL").expect("URL de base de datos no especificado");
}
