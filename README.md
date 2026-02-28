# rust-otel-collector

A Rust service with **OpenTelemetry (OTEL) Collector** and **Jaeger** integration, following the same architecture as platform-orchestrator-core.

## Features

- **OTEL Collector**: Receives traces and metrics via OTLP (gRPC on 4317, HTTP on 4318) and forwards traces to Jaeger and metrics to Prometheus
- **Jaeger**: All-in-one backend for trace storage and the Jaeger Web UI
- **Structured tracing**: `tracing` + `tracing-opentelemetry` with JSON logs and OTLP export
- **Request ID**: Every request gets a unique ID (from `X-Request-Id` header or generated). It is attached to the trace span and to the response header so you can correlate logs and Jaeger traces by the same ID
- **Structured logs**: JSON logs with `request_id`, `method`, `path`, and span context (handler/service/repo); "request started" and "request completed" log lines include `request_id` and `status`
- **Health endpoints**: `/api/v1/livez`, `/api/v1/readyz`
- **Sample API**: Product and Item CRUD (one-to-many: Product has many Items). In-memory store, handler → service → repository layers, OTEL tracing on all endpoints
- **Config via env**: Port, OTEL endpoint, service name/version

## Architecture

```
src/
├── main.rs              # Entry point, init config + tracing + web
├── lib.rs               # Library root (app, pkg)
├── app/
│   ├── mod.rs
│   ├── route/           # Route aggregation
│   ├── dto/             # Shared DTOs (e.g. health)
│   ├── handler/         # HTTP handlers (call service)
│   ├── service/         # Business logic
│   └── repository/      # Data access
└── pkg/
    ├── config/          # App config (env-based)
    ├── tracing/         # OTEL tracer + meter, tracing-subscriber
    └── web/             # Axum server init, request_id middleware
```

## Prerequisites

- Rust 1.89+
- Docker and Docker Compose (for full stack)

## Run locally (no Docker)

1. Start OTEL Collector and Jaeger (e.g. via Docker Compose, see below), or use defaults (`http://localhost:4317`).
2. Set env (optional):
   - `PORT` (default: 8080)
   - `OTEL_EXPORTER_OTLP_ENDPOINT` or `OTEL_ENDPOINT` (default: http://localhost:4317)
   - `SERVICE_NAME`, `SERVICE_VERSION`, `ENVIRONMENT`
3. Run the app:

```bash
cargo run
```

- API: http://localhost:8080/api/v1/livez  

## Run with Docker Compose (app + OTEL Collector + Jaeger)

From the project root:

```bash
docker compose up --build
```

- **App**: http://localhost:8080  
- **Jaeger UI**: http://localhost:16686  
- **OTEL Collector**: OTLP gRPC 4317, HTTP 4318; Prometheus metrics on 9090  

The app is configured with `OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4317` so traces and metrics go to the collector, which forwards traces to Jaeger and exposes metrics for Prometheus.

## Request ID and correlating logs with traces

Each request gets a **request ID**: from the `X-Request-Id` header if the client sends it, otherwise a new UUID. The ID is:

- Set on the **request** span (`request_id`, `http.method`, `http.route`) so it appears in **Jaeger** on the root span and in **structured JSON logs** (span context)
- Sent back in the **response header** `X-Request-Id`

Use the same ID to find the trace in Jaeger and to grep logs (e.g. `jq '.fields.request_id'` or search for the ID in your log aggregator). The middleware logs `"request started"` and `"request completed"` with `request_id`, `method`, `path`, and `status`.

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8080 | HTTP server port |
| `OTEL_ENDPOINT` / `OTEL_EXPORTER_OTLP_ENDPOINT` | http://localhost:4317 | OTLP exporter endpoint |
| `SERVICE_NAME` | rust-otel-collector | Service name for telemetry |
| `SERVICE_VERSION` | 0.1.0 | Service version |
| `ENVIRONMENT` | development | Deployment environment |

## API endpoints

### Health
| Method | Path | Description |
|--------|------|-------------|
| GET | /api/v1/livez | Liveness probe |
| GET | /api/v1/readyz | Readiness probe |

### Products (name, description)
| Method | Path | Description |
|--------|------|-------------|
| GET | /api/v1/products | List all products |
| GET | /api/v1/products/:id | Get product by id |
| POST | /api/v1/products | Create product (body: `{"name":"","description":""}`) |
| PUT | /api/v1/products/:id | Update product (body: `{"name":"","description":""}` optional fields) |
| DELETE | /api/v1/products/:id | Delete product (409 if it has items) |
| GET | /api/v1/products/:id/items | List items for a product |

### Items (name, description, price, product_id)
| Method | Path | Description |
|--------|------|-------------|
| GET | /api/v1/items | List all items, or ?product_id= to filter by product |
| GET | /api/v1/items/:id | Get item by id |
| POST | /api/v1/items | Create item (body: `{"name":"","description":"","price":0,"product_id":"uuid"}`) |
| PUT | /api/v1/items/:id | Update item (optional fields) |
| DELETE | /api/v1/items/:id | Delete item |

## License

