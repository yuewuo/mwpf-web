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
curl "http://127.0.0.1:8080/decode?type=rsc-d-3&syndrome=0,1&html=1"
```
Response: `{ decode }`


### Code Information
```bash
curl "http://127.0.0.1:8080/codes"
```
Response: `{ 'rsc-d-5': { .. }, 'color-d-5': { .. } }`

## Endpoints

| Endpoint      | Method | Description      | Parameters                            |
| ------------- | ------ | ---------------- | ------------------------------------- |
| `/`           | GET    | Health check     | None                                  |
| `/api/decode` | GET    | Decode           | type, syndrome, html, visualizer_json |
| `/api/codes`  | GET    | Code Information |                                       |
