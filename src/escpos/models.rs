use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterModel {
    Epson,
    Elgin,
    Bematech,
    Daruma,
    Generica,
}

impl FromStr for PrinterModel {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase();
        if normalized.contains("epson") {
            Ok(PrinterModel::Epson)
        } else if normalized.contains("elgin") {
            Ok(PrinterModel::Elgin)
        } else if normalized.contains("bema") {
            Ok(PrinterModel::Bematech)
        } else if normalized.contains("daruma") {
            Ok(PrinterModel::Daruma)
        } else {
            Ok(PrinterModel::Generica)
        }
    }
}

impl PrinterModel {
    /// Retorna a sequência de inicialização recomendada para o modelo
    pub fn init_sequence(&self) -> Vec<u8> {
        match self {
            PrinterModel::Bematech => {
                // ESC @ para reset + ativação de codepage Latin1/PC850
                vec![0x1B, 0x40, 0x1B, 0x74, 0x02]
            }
            PrinterModel::Elgin => {
                // ESC @ + ESC t 2 (PC850)
                vec![0x1B, 0x40, 0x1B, 0x74, 0x02]
            }
            PrinterModel::Daruma => {
                // ESC @ + ESC t 2
                vec![0x1B, 0x40, 0x1B, 0x74, 0x02]
            }
            PrinterModel::Epson | PrinterModel::Generica => {
                // ESC @ + ESC t 2 (PC850 Multilingual)
                vec![0x1B, 0x40, 0x1B, 0x74, 0x02]
            }
        }
    }
}
