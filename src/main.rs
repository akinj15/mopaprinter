mod api;
mod escpos;
mod transport;

use api::{imprimir_handler, info_handler};
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const PORTA_FIXA: u16 = 36274;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicialização dos logs estruturados
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mopaprinter=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Configuração das rotas (sem CORS conforme requisito)
    let app = Router::new()
        .route("/imprimir", post(imprimir_handler))
        .route("/imprimir", get(info_handler))
        .route("/", get(info_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], PORTA_FIXA));
    info!("Iniciando mopaprinter na porta fixa http://0.0.0.0:{}", PORTA_FIXA);
    info!("Endpoint de impressão: POST http://localhost:{}/imprimir", PORTA_FIXA);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
