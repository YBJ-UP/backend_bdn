use std::env::var;

use axum::Router;
use backend_bdn::routes::create_router;
use dotenv::dotenv;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing();

    let db_url = var("DATABASE_URL").expect("URL de base de datos no especificado");

    let app: Router = create_router();

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

fn init_tracing() {
    let fmt = fmt::layer()
        .json()
        .with_span_list(true)
        .with_current_span(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_target(false)
        .with_ansi(false)
        .with_timer(fmt::time::UtcTime::rfc_3339());

    registry().with(fmt).init();
}
