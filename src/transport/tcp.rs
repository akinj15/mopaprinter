use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Envia bytes para uma impressora de rede via TCP (porta padrão 9100 caso omitida)
pub async fn send_tcp(target: &str, data: &[u8]) -> Result<usize, String> {
    let address = if !target.contains(':') {
        format!("{}:9100", target)
    } else {
        target.to_string()
    };

    let connect_timeout = Duration::from_secs(5);
    let mut stream = timeout(connect_timeout, TcpStream::connect(&address))
        .await
        .map_err(|_| format!("Timeout de conexão ao tentar alcançar a impressora de rede '{}'", address))?
        .map_err(|e| format!("Falha ao conectar na impressora de rede '{}': {}", address, e))?;

    let write_timeout = Duration::from_secs(5);
    timeout(write_timeout, stream.write_all(data))
        .await
        .map_err(|_| format!("Timeout ao enviar dados para a impressora '{}'", address))?
        .map_err(|e| format!("Falha ao escrever dados no socket TCP: {}", e))?;

    stream
        .flush()
        .await
        .map_err(|e| format!("Falha ao descarregar buffer TCP: {}", e))?;

    Ok(data.len())
}
