use crate::escpos::{encode_print_job, FontSize, PrinterModel};
use crate::transport::send_to_printer;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct ImprimirRequest {
    pub modelo_impressora: String,
    pub caminho_impressora: String,
    pub tamanho_fonte: FontSize,
    pub conteudo: String,
}

#[derive(Debug, Serialize)]
pub struct ImprimirResponse {
    pub sucesso: bool,
    pub mensagem: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_enviados: Option<usize>,
}

/// Handler do endpoint POST /imprimir
pub async fn imprimir_handler(
    Json(payload): Json<ImprimirRequest>,
) -> (StatusCode, Json<ImprimirResponse>) {
    info!(
        modelo = %payload.modelo_impressora,
        caminho = %payload.caminho_impressora,
        tamanho = ?payload.tamanho_fonte,
        "Recebida requisição de impressão"
    );

    let model = match PrinterModel::from_str(&payload.modelo_impressora) {
        Ok(m) => m,
        Err(_) => PrinterModel::Generica,
    };

    // Codifica os dados para comandos ESC/POS
    // Conforme o requisito: NUNCA corta automaticamente
    let escpos_bytes = encode_print_job(model, &payload.tamanho_fonte, &payload.conteudo);

    // Envia para o caminho/dispositivo configurado
    match send_to_printer(&payload.caminho_impressora, &escpos_bytes).await {
        Ok(bytes_written) => {
            info!(
                caminho = %payload.caminho_impressora,
                bytes = bytes_written,
                "Impressão concluída com sucesso"
            );
            (
                StatusCode::OK,
                Json(ImprimirResponse {
                    sucesso: true,
                    mensagem: "Impressão enviada com sucesso para a impressora.".to_string(),
                    bytes_enviados: Some(bytes_written),
                }),
            )
        }
        Err(err_msg) => {
            error!(
                caminho = %payload.caminho_impressora,
                erro = %err_msg,
                "Falha na comunicação com a impressora"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ImprimirResponse {
                    sucesso: false,
                    mensagem: format!("Erro ao enviar impressão: {}", err_msg),
                    bytes_enviados: None,
                }),
            )
        }
    }
}

/// Handler GET /imprimir informativo (caso alguém acesse via navegador)
pub async fn info_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "servico": "mopaprinter",
        "versao": "0.1.0",
        "status": "online",
        "porta": 36274,
        "instrucoes": "Envie uma requisição HTTP POST para /imprimir com o corpo JSON",
        "exemplo_payload": {
            "modelo_impressora": "epson",
            "caminho_impressora": "192.168.1.100:9100",
            "tamanho_fonte": "normal",
            "conteudo": "Ola mundo thermal!\n"
        }
    }))
}
