use std::env::var;

use axum::{Router, routing::get};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");

    let db_url = var("DATABASE_URL").expect("URL de base de datos no especificado");

    let app: Router = Router::new().route("/health", get(|| async { "Hola" }));

    let port = var("PORT").unwrap_or_else(|_| {
        println!("No se definió el puerto. Utilizando el puerto 3000");
        "3000".to_string()
    });

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Error al abrir el puerto.");

    axum::serve(listener, app)
        .await
        .expect("Error al iniciar el servidor");
}
