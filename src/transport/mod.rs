pub mod file;
pub mod tcp;
pub mod windows;

use std::path::Path;

/// Despacha os bytes de impressão para o destino correto de acordo com o formato do caminho
pub async fn send_to_printer(caminho: &str, data: &[u8]) -> Result<usize, String> {
    let clean = caminho.trim();

    if clean.is_empty() {
        return Err("O caminho da impressora não pode ser vazio.".to_string());
    }

    // 1. Prefixo explícito tcp://
    if let Some(target) = clean.strip_prefix("tcp://") {
        return tcp::send_tcp(target, data).await;
    }

    // 2. Prefixo explícito file://
    if let Some(target) = clean.strip_prefix("file://") {
        return file::send_file(target, data);
    }

    // 3. Prefixo explícito spooler:// ou winspool:// (útil no Windows)
    if let Some(target) = clean.strip_prefix("spooler://").or_else(|| clean.strip_prefix("winspool://")) {
        return windows::send_windows_spooler(target, data);
    }

    // 4. Padrão IP:Porta ou hostname:porta (ex: 192.168.1.100:9100)
    if let Some((host, port_str)) = clean.rsplit_once(':') {
        if !host.is_empty() && port_str.chars().all(|c| c.is_ascii_digit()) && !host.contains('/') && !host.contains('\\') {
            return tcp::send_tcp(clean, data).await;
        }
    }

    // 5. Padrão apenas IP (ex: 192.168.1.100 ou 10.0.0.50) -> usa porta padrão 9100
    let parts: Vec<&str> = clean.split('.').collect();
    if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
        return tcp::send_tcp(clean, data).await;
    }

    // 6. Caminhos de sistema de arquivos ou dispositivos Linux/Unix (ex: /dev/usb/lp*, /tmp/..., ./...)
    if clean.starts_with('/') || clean.starts_with("./") || clean.starts_with("../") || clean.contains('/') {
        return file::send_file(clean, data);
    }

    // 7. Padrão portas COM/LPT do Windows (ex: COM1, COM3, \\.\COM1, LPT1) ou caminhos absolutos Windows (C:\...)
    let upper = clean.to_uppercase();
    if upper.starts_with(r"\\.\")
        || (upper.starts_with("COM") && upper[3..].chars().all(|c| c.is_ascii_digit()))
        || (upper.starts_with("LPT") && upper[3..].chars().all(|c| c.is_ascii_digit()))
        || (clean.len() >= 3 && clean.chars().nth(1) == Some(':') && (clean.chars().nth(2) == Some('\\') || clean.chars().nth(2) == Some('/')))
        || clean.contains('\\')
    {
        return file::send_file(clean, data);
    }

    // 8. Se for um arquivo existente no sistema de arquivos
    if Path::new(clean).exists() {
        return file::send_file(clean, data);
    }

    // 9. No Windows, qualquer outro nome é interpretado como o Nome da Impressora na Fila do Spooler
    #[cfg(target_os = "windows")]
    {
        return windows::send_windows_spooler(clean, data);
    }

    // Se estiver em Linux/macOS e não foi identificado como TCP nem arquivo existente
    #[cfg(not(target_os = "windows"))]
    {
        Err(format!(
            "Caminho '{}' não reconhecido. Use caminhos como '/dev/usb/lp0', '192.168.x.x:9100' ou 'file:///caminho'.",
            clean
        ))
    }
}
