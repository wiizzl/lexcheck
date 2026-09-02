# LexCheck

![Tests](https://github.com/wiizzl/lexcheck/actions/workflows/ci.yaml/badge.svg)

A Proof of Concept (PoC) REST API for regulatory compliance and document validation.

## Stack

- Rust (Axum, Tokio, Serde)
- Docker

## Getting started

```sh
# With your own Rust toolchain
cargo run

# With Docker
docker compose up -d
```

## API Usage

A [Postman collection](./postman.json) is available to easily test the API endpoint.

- Endpoint : `POST /validate`
- Payload Examples :

```json
{
  "user_id": "U-1234",
  "age": 25,
  "document_status": "VERIFIED"
}
```

```json
{
  "user_id": "U-5678",
  "age": 17,
  "document_status": "VERIFIED"
}
```

```json
{
  "user_id": "U-9012",
  "age": 30,
  "document_status": "PENDING"
}
```
