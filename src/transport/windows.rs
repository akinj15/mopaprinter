#[cfg(target_os = "windows")]
pub fn send_windows_spooler(printer_name: &str, data: &[u8]) -> Result<usize, String> {
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Graphics::Printing::{
        ClosePrinter, EndDocPrinter, EndPagePrinter, OpenPrinterW, StartDocPrinterW,
        StartPagePrinter, WritePrinter, DOC_INFO_1W,
    };

    // Converte strings para UTF-16 terminadas em nulo
    let mut name_wide: Vec<u16> = printer_name.encode_utf16().collect();
    name_wide.push(0);

    let mut doc_name_wide: Vec<u16> = "MopaPrinter ESC/POS Doc".encode_utf16().collect();
    doc_name_wide.push(0);

    let mut datatype_wide: Vec<u16> = "RAW".encode_utf16().collect();
    datatype_wide.push(0);

    unsafe {
        let mut h_printer: HANDLE = std::mem::zeroed();
        let open_res = OpenPrinterW(name_wide.as_ptr() as _, &mut h_printer, null());
        if open_res == 0 || h_printer.is_null() {
            return Err(format!(
                "Não foi possível abrir a impressora do Windows '{}'. Verifique se o nome está correto no Painel de Controle.",
                printer_name
            ));
        }

        let doc_info = DOC_INFO_1W {
            pDocName: doc_name_wide.as_mut_ptr(),
            pOutputFile: null_mut(),
            pDatatype: datatype_wide.as_mut_ptr(),
        };

        let job_id = StartDocPrinterW(h_printer, 1, &doc_info as *const DOC_INFO_1W as *const _);
        if job_id == 0 {
            ClosePrinter(h_printer);
            return Err(format!(
                "Falha ao iniciar documento de impressão (StartDocPrinterW) na impressora '{}'.",
                printer_name
            ));
        }

        let start_page = StartPagePrinter(h_printer);
        if start_page == 0 {
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return Err(format!(
                "Falha ao iniciar página de impressão (StartPagePrinter) na impressora '{}'.",
                printer_name
            ));
        }

        let mut written: u32 = 0;
        let write_res = WritePrinter(
            h_printer,
            data.as_ptr() as _,
            data.len() as u32,
            &mut written,
        );

        EndPagePrinter(h_printer);
        EndDocPrinter(h_printer);
        ClosePrinter(h_printer);

        if write_res == 0 || written != data.len() as u32 {
            return Err(format!(
                "Falha ao enviar todos os bytes para o Spooler do Windows (enviados {} de {} bytes).",
                written,
                data.len()
            ));
        }

        Ok(written as usize)
    }
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
pub fn send_windows_spooler(_printer_name: &str, _data: &[u8]) -> Result<usize, String> {
    Err("O envio via fila nativa do Spooler do Windows está disponível apenas quando a aplicação é executada em sistema operacional Windows.".to_string())
}
