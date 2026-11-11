use std::{env::var, error::Error, process, sync::Arc};

use axum::Router;
use backend_bdn::{AppState, router::create_router};
use dotenv::dotenv;
use sqlx::PgPool;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    init_tracing();

    let db_url = var("DATABASE_URL").expect("URL de base de datos no especificado");

    let state = Arc::new(AppState {
        pg: PgPool::connect(&db_url).await.unwrap_or_else(|e| {
            eprintln!(
                "No se pudo conectar a la base de datos en {}: {}",
                db_url, e
            );
            process::exit(1)
        }),
    });

    let app: Router = create_router().with_state(state);

    let port = var("PORT").unwrap_or_else(|_| {
        println!("No se definió el puerto. Utilizando el puerto 3000");
        "3000".to_string()
    });

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Error al abrir el puerto.");

    tracing::info!(
        "Servidor escuchando en el puerto {}.",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_exit())
        .await
        .expect("Error al iniciar el servidor");

    Ok(())
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
