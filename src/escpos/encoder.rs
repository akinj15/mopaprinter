use super::models::PrinterModel;
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontSize {
    Pequena,
    Normal,
    Dupla,
    Grande,
    Custom { width: u8, height: u8 },
}

impl FontSize {
    pub fn to_escpos_commands(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            FontSize::Pequena => {
                // ESC M 1 (Seleciona Fonte B compacta) + GS ! 0x00 (1x1)
                bytes.extend_from_slice(&[0x1B, 0x4D, 0x01]);
                bytes.extend_from_slice(&[0x1D, 0x21, 0x00]);
            }
            FontSize::Normal => {
                // ESC M 0 (Seleciona Fonte A padrão) + GS ! 0x00 (1x1)
                bytes.extend_from_slice(&[0x1B, 0x4D, 0x00]);
                bytes.extend_from_slice(&[0x1D, 0x21, 0x00]);
            }
            FontSize::Dupla => {
                // ESC M 0 (Fonte A) + GS ! 0x11 (2x largura, 2x altura)
                bytes.extend_from_slice(&[0x1B, 0x4D, 0x00]);
                bytes.extend_from_slice(&[0x1D, 0x21, 0x11]);
            }
            FontSize::Grande => {
                // ESC M 0 (Fonte A) + GS ! 0x11 (2x largura, 2x altura)
                bytes.extend_from_slice(&[0x1B, 0x4D, 0x00]);
                bytes.extend_from_slice(&[0x1D, 0x21, 0x11]);
            }
            FontSize::Custom { width, height } => {
                bytes.extend_from_slice(&[0x1B, 0x4D, 0x00]);
                let w = width.saturating_sub(1).min(7);
                let h = height.saturating_sub(1).min(7);
                let flag = (w << 4) | h;
                bytes.extend_from_slice(&[0x1D, 0x21, flag]);
            }
        }
        bytes
    }
}

// Deserializador flexível para aceitar números (ex: 1, 2) ou strings (ex: "normal", "2", "dupla")
impl<'de> Deserialize<'de> for FontSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FontSizeVisitor;

        impl<'de> serde::de::Visitor<'de> for FontSizeVisitor {
            type Value = FontSize;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("um número de 1 a 8 ou string ('normal', 'pequena', 'dupla', 'grande')")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let scale = v.clamp(1, 8) as u8;
                if scale == 1 {
                    Ok(FontSize::Normal)
                } else if scale == 2 {
                    Ok(FontSize::Dupla)
                } else {
                    Ok(FontSize::Custom {
                        width: scale,
                        height: scale,
                    })
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_i64(v as i64)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let clean = v.trim().to_lowercase();
                if let Ok(num) = clean.parse::<i64>() {
                    return self.visit_i64(num);
                }

                match clean.as_str() {
                    "pequena" | "small" | "fonte_b" => Ok(FontSize::Pequena),
                    "normal" | "regular" | "padrao" | "fonte_a" => Ok(FontSize::Normal),
                    "dupla" | "double" => Ok(FontSize::Dupla),
                    "grande" | "large" => Ok(FontSize::Grande),
                    _ => Ok(FontSize::Normal),
                }
            }
        }

        deserializer.deserialize_any(FontSizeVisitor)
    }
}

/// Converte caracteres Unicode/UTF-8 em bytes da codepage CP850 (Multilingual Latin I)
pub fn utf8_to_cp850(input: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_ascii() {
            out.push(ch as u8);
        } else {
            let cp850_byte = match ch {
                'Ç' => 0x80,
                'ü' => 0x81,
                'é' => 0x82,
                'â' => 0x83,
                'ä' => 0x84,
                'à' => 0x85,
                'å' => 0x86,
                'ç' => 0x87,
                'ê' => 0x88,
                'ë' => 0x89,
                'è' => 0x8A,
                'ï' => 0x8B,
                'î' => 0x8C,
                'ì' => 0x8D,
                'Ä' => 0x8E,
                'Å' => 0x8F,
                'É' => 0x90,
                'æ' => 0x91,
                'Æ' => 0x92,
                'ô' => 0x93,
                'ö' => 0x94,
                'ò' => 0x95,
                'û' => 0x96,
                'ù' => 0x97,
                'ÿ' => 0x98,
                'Ö' => 0x99,
                'Ü' => 0x9A,
                'ø' => 0x9B,
                '£' => 0x9C,
                'Ø' => 0x9D,
                '×' => 0x9E,
                'á' => 0xA0,
                'í' => 0xA1,
                'ó' => 0xA2,
                'ú' => 0xA3,
                'ñ' => 0xA4,
                'Ñ' => 0xA5,
                'ª' => 0xA6,
                'º' => 0xA7,
                '¿' => 0xA8,
                '®' => 0xA9,
                '¬' => 0xAA,
                '½' => 0xAB,
                '¼' => 0xAC,
                '¡' => 0xAD,
                '«' => 0xAE,
                '»' => 0xAF,
                'Á' => 0xB5,
                'Â' => 0xB6,
                'À' => 0xB7,
                '©' => 0xB8,
                'ã' => 0xC6,
                'Ã' => 0xC7,
                'Ê' => 0xD2,
                'Ë' => 0xD3,
                'È' => 0xD4,
                'Í' => 0xD6,
                'Î' => 0xD7,
                'Ï' => 0xD8,
                'Ì' => 0xDE,
                'Ó' => 0xE0,
                'ß' => 0xE1,
                'Ô' => 0xE2,
                'Ò' => 0xE3,
                'õ' => 0xE4,
                'Õ' => 0xE5,
                'µ' => 0xE6,
                'þ' => 0xE7,
                'Þ' => 0xE8,
                'Ú' => 0xE9,
                'Û' => 0xEA,
                'Ù' => 0xEB,
                'ý' => 0xEC,
                'Ý' => 0xED,
                '¯' => 0xEE,
                '´' => 0xEF,
                '±' => 0xF1,
                '‗' => 0xF2,
                '¾' => 0xF3,
                '¶' => 0xF4,
                '§' => 0xF5,
                '÷' => 0xF6,
                '¸' => 0xF7,
                '°' => 0xF8,
                '¨' => 0xF9,
                '·' => 0xFA,
                '¹' => 0xFB,
                '³' => 0xFC,
                '²' => 0xFD,
                _ => b'?',
            };
            out.push(cp850_byte);
        }
    }
    out
}

/// Codifica o payload completo para comandos ESC/POS prontos para envio.
/// IMPORTANTE: Conforme requisito do projeto, NÃO adiciona comando de corte automaticamente.
pub fn encode_print_job(model: PrinterModel, font_size: &FontSize, content: &str) -> Vec<u8> {
    let mut payload = Vec::new();

    // 1. Inicialização da impressora e seleção de codepage CP850
    payload.extend(model.init_sequence());

    // 2. Configuração do tamanho da fonte
    payload.extend(font_size.to_escpos_commands());

    // 3. Conversão do conteúdo para CP850 e adição ao buffer
    payload.extend(utf8_to_cp850(content));

    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_size_deserialization() {
        let size_str: FontSize = serde_json::from_str("\"pequena\"").unwrap();
        assert_eq!(size_str, FontSize::Pequena);

        let size_num: FontSize = serde_json::from_str("2").unwrap();
        assert_eq!(size_num, FontSize::Dupla);

        let size_num_str: FontSize = serde_json::from_str("\"3\"").unwrap();
        assert_eq!(size_num_str, FontSize::Custom { width: 3, height: 3 });
    }

    #[test]
    fn test_utf8_to_cp850_accents() {
        let text = "Atenção: Café e Pão R$ 10,00";
        let encoded = utf8_to_cp850(text);
        assert!(!encoded.is_empty());
        assert!(encoded.contains(&0x87)); // ç
        assert!(encoded.contains(&0xC6)); // ã
        assert!(encoded.contains(&0x82)); // é
    }

    #[test]
    fn test_encode_print_job_no_cut() {
        let job = encode_print_job(PrinterModel::Generica, &FontSize::Normal, "Teste de impressão\n");
        // Verifica se inicia com ESC @
        assert_eq!(&job[0..2], &[0x1B, 0x40]);
        // Garante que não possui corte automático ESC i (0x1b, 0x69) ou GS V (0x1d, 0x56) no final
        let len = job.len();
        if len >= 3 {
            let last_3 = &job[len - 3..];
            assert_ne!(last_3, &[0x1D, 0x56, 0x00]);
            assert_ne!(last_3, &[0x1D, 0x56, 0x01]);
        }
    }
}
