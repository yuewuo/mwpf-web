# mwpf-web
A simple web application to demo MWPF decoder

## Building and Running

### Prerequisites
- Rust (latest stable version)
- Cargo

### Build and Run
```bash
cargo run
cd visualizer && npm run dev
```

The server will start on `http://127.0.0.1:8080`

### Build for Release
```bash
cargo build --release
```

## API Usage

### Health Check
```bash
curl http://127.0.0.1:8080/
```

### Decode
```bash
curl "http://127.0.0.1:8080/api/decode?code_id=rsc-depolarize-d-3&syndrome=0,1&with_html=1&cluster_node_limit=200"
```
Response: `{ decode }`


### Code Information
```bash
curl "http://127.0.0.1:8080/api/codes"
```
Response: `{ 'rsc-d-5': { .. }, 'color-d-5': { .. } }`

## Endpoints

| Endpoint      | Method | Description      | Parameters                                                  |
| ------------- | ------ | ---------------- | ----------------------------------------------------------- |
| `/`           | GET    | Health check     | None                                                        |
| `/api/decode` | GET    | Decode           | code_id, syndrome, with_html, with_json, cluster_node_limit |
| `/api/codes`  | GET    | Code Information |                                                             |
