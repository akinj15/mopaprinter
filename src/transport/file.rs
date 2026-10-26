use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Envia bytes diretamente para um arquivo de dispositivo ou porta local (ex: /dev/usb/lp0, /dev/ttyUSB0, COM1)
pub fn send_file(path_str: &str, data: &[u8]) -> Result<usize, String> {
    #[cfg(target_os = "windows")]
    let normalized_path = {
        let upper = path_str.to_uppercase();
        if upper.starts_with("COM") || upper.starts_with("LPT") {
            if !path_str.starts_with(r"\\.\") {
                format!(r"\\.\{}", path_str)
            } else {
                path_str.to_string()
            }
        } else {
            path_str.to_string()
        }
    };

    #[cfg(not(target_os = "windows"))]
    let normalized_path = path_str.to_string();

    let path = Path::new(&normalized_path);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .map_err(|e| format!("Falha ao abrir o dispositivo/arquivo '{}': {}", path_str, e))?;

    file.write_all(data)
        .map_err(|e| format!("Falha ao escrever no dispositivo '{}': {}", path_str, e))?;

    file.flush()
        .map_err(|e| format!("Falha ao descarregar buffer do dispositivo '{}': {}", path_str, e))?;

    Ok(data.len())
}
