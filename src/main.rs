use std::env::var;

use axum::{Router, routing::get};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");

    let db_url = var("DATABASE_URL").expect("URL de base de datos no especificado");

    let app: Router = Router::new()
        .route("/health", get(|| async { "Es servidor está saludable" }))
        .route("/", get(|| async { "El servidor está activo." }));

    let port = var("PORT").unwrap_or_else(|_| {
        println!("No se definió el puerto. Utilizando el puerto 3000");
        "3000".to_string()
    });

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Error al abrir el puerto.");

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_exit())
        .await
        .expect("Error al iniciar el servidor");
}

async fn graceful_exit() {
    tokio::signal::ctrl_c().await.unwrap();
    tracing::warn!("Señal de salida recibida. Apagando servidor...")
}
