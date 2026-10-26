# MopaPrinter 🖨️

Serviço REST em **Rust** ultrarrápido, leve e multiplataforma para impressão térmica via protocolo **ESC/POS** no **Windows** e **Linux**.

---

## 🚀 Principais Características

- **Serviço HTTP REST**: Porta fixa `36274` (sem CORS).
- **Multiplataforma**: Executável nativo `.exe` para Windows e binário ELF para Linux.
- **Protocolo ESC/POS Nativo**: Compatível com as principais marcas do mercado (Epson, Elgin, Bematech, Daruma e genéricas chinesas como Xprinter, Sunmi, etc.).
- **Tratamento de Codepage (CP850)**: Acentuação correta do Português do Brasil (`ç`, `ã`, `é`, `ô`, `í`, `ú`, `º`, `ª`, etc.) sem caracteres estranhos.
- **Tipos de Conexão Suportados**:
  - **Rede TCP/IP**: `192.168.1.100:9100` (porta 9100 padrão se omitida).
  - **Porta / Dispositivo Linux**: `/dev/usb/lp0`, `/dev/ttyUSB0`, etc.
  - **Porta Serial Windows**: `COM1`, `COM3`, `\\.\COM1`.
  - **Fila/Spooler Nativo do Windows**: Envio RAW direto pelo nome da impressora no Windows (ex: `"EPSON TM-T20X"`).
- **Controle de Corte**: Nunca corta o papel automaticamente (o corte é controlado exclusivamente pelo conteúdo enviado).

---

## 📡 Especificação do Endpoint

### `POST /imprimir`
Envia um documento formatado em ESC/POS para a impressora.

#### Corpo da Requisição (JSON)
```json
{
  "modelo_impressora": "epson",
  "caminho_impressora": "192.168.1.100:9100",
  "tamanho_fonte": "normal",
  "conteudo": "MOPAPRINTER TESTE\nValor: R$ 45,50\nObrigado pela preferência!\n"
}
```

#### Parâmetros

| Campo | Tipo | Descrição | Exemplos |
| :--- | :--- | :--- | :--- |
| `modelo_impressora` | `string` | Modelo ou fabricante da impressora térmica. | `"epson"`, `"elgin"`, `"bematech"`, `"daruma"`, `"generica"` |
| `caminho_impressora` | `string` | Endereço IP, caminho de dispositivo ou nome da impressora no Windows. | `"192.168.1.200:9100"`, `"/dev/usb/lp0"`, `"COM1"`, `"EPSON TM-T20X"` |
| `tamanho_fonte` | `string` ou `number` | Tamanho da fonte e multiplicador de largura/altura. | `"pequena"`, `"normal"`, `"dupla"`, `"grande"`, `1` a `8` |
| `conteudo` | `string` | Texto completo a ser impresso com quebras de linha (`\n`). | `"Texto da nota fiscal...\n"` |

#### Respostas

**Sucesso (`200 OK`)**:
```json
{
  "sucesso": true,
  "mensagem": "Impressão enviada com sucesso para a impressora.",
  "bytes_enviados": 75
}
```

**Erro (`500 Internal Server Error`)**:
```json
{
  "sucesso": false,
  "mensagem": "Erro ao enviar impressão: Falha ao conectar na impressora de rede '192.168.1.100:9100': Connection refused"
}
```

---

## 🧪 Exemplos de Uso

### 1. Via cURL (Terminal)

#### Impressora em Rede (TCP)
```bash
curl -X POST http://localhost:36274/imprimir \
  -H "Content-Type: application/json" \
  -d '{
    "modelo_impressora": "epson",
    "caminho_impressora": "192.168.1.100:9100",
    "tamanho_fonte": "normal",
    "conteudo": "================================\n   RESTAURANTE SABOR BRASIL     \n================================\n1x Almoço Executivo    R$ 35,00\n1x Suco Natural        R$  8,00\n--------------------------------\nTOTAL:                 R$ 43,00\n================================\n\n\n"
  }'
```

#### Impressora USB no Linux
```bash
curl -X POST http://localhost:36274/imprimir \
  -H "Content-Type: application/json" \
  -d '{
    "modelo_impressora": "elgin",
    "caminho_impressora": "/dev/usb/lp0",
    "tamanho_fonte": "pequena",
    "conteudo": "Impressão USB direta no Linux\n"
  }'
```

#### Fila de Spooler do Windows (Pelo Nome)
```bash
curl -X POST http://localhost:36274/imprimir \
  -H "Content-Type: application/json" \
  -d '{
    "modelo_impressora": "bematech",
    "caminho_impressora": "MP-4200 TH",
    "tamanho_fonte": "dupla",
    "conteudo": "SENHA DE ATENDIMENTO: 104\n\n\n"
  }'
```

---

## 🛠️ Compilação e Geração de Executáveis

### 1. Automático via GitHub Actions (Recomendado)
O projeto inclui um fluxo de trabalho em `.github/workflows/release.yml`:
- Ao criar e enviar uma tag de versão (ex: `git tag v1.0.0 && git push origin v1.0.0`), o GitHub Actions compila nativamente nos servidores Linux e Windows e gera automaticamente:
  - `mopaprinter-linux-x86_64.tar.gz` (Binário Linux)
  - `mopaprinter-windows-x86_64.zip` (Executável Windows `mopaprinter.exe`)
- Os artefatos ficam disponíveis tanto para download nas execuções de *Actions* quanto na aba de *Releases* do repositório.

### 2. Compilação Local

#### No Linux
```bash
cargo build --release
# O binário gerado estará em: target/release/mopaprinter
```

#### No Windows
```cmd
cargo build --release
:: O executável gerado estará em: target\release\mopaprinter.exe
```

---

## 📜 Licença
Distribuído sob licença MIT.
