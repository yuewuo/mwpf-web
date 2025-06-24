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

## Load Testing

```sh
cd loadtest
cargo run --release -- --host http://localhost:8080 --report-file=report-m1max.html --no-reset-metrics --run-time=30s --scenarios DecodeTransactions
cargo run --release -- --host http://localhost:8892 --report-file=report-aws.html --no-reset-metrics --run-time=30s --scenarios DecodeTransactions
cargo run --release -- --host http://localhost:8080 --report-file=report-m1max-html.html --no-reset-metrics --run-time=30s --scenarios DecodingProcessTransactions
```

When tested on Apple M1Max CPU, it can handle about 6000 requests per second.
The small t2 instance on AWS supports about 400 requests per second, which I think it's pretty much enough for our use case.
I don't think there will be more than 100 people using it simultaneously, and the response time is mostly below 500ms.
Note that this tests the decode interface using the worst-case inputs: all the data qubits are erased.
In practice, it should be better.

Here we did not test the decoding-process interface because I suppose people will not be using that too much.
When testing on M1Max, it can handle about 1000 requests per second, still not bad, with a response time below 120ms.
