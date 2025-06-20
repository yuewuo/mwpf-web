# mwpf-web
A simple web application to demo MWPF decoder

## Building and Running

### Prerequisites
- Rust (latest stable version)
- Cargo

### Build and Run
```bash
cargo run
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
Response: `param1: value1, param2: value2, param3: value3`

### Partial Parameters
```bash
curl "http://127.0.0.1:8080/combine?name=test&value=123"
```
Response: `Name: test, Value: 123`

### No Parameters
```bash
curl "http://127.0.0.1:8080/combine"
```
Response: `No parameters provided`

## Endpoints

| Endpoint       | Method | Description                 | Parameters                           |
| -------------- | ------ | --------------------------- | ------------------------------------ |
| `/`            | GET    | Health check                | None                                 |
| `/combine`     | GET    | Combine specific parameters | name, value, category (all optional) |
| `/combine-all` | GET    | Combine any parameters      | Any query parameters                 |

## Customization

To add more specific parameters to the `/combine` endpoint, modify the `QueryParams` struct in `src/main.rs`:

```rust
#[derive(Debug, Deserialize)]
struct QueryParams {
    name: Option<String>,
    value: Option<String>,
    category: Option<String>,
    // Add your new parameters here
    new_param: Option<String>,
}
```

Then update the `combine_params` function to handle the new parameter. 