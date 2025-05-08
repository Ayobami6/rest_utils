# REST Utils API

A Rust-based REST API utility service providing token generation and utility data endpoints.

## Features

- Token-based authentication
- Utility data endpoints
- PostgreSQL database integration
- Containerized deployment

## Tech Stack

- **Backend**: Rust with Actix-web
- **Database**: PostgreSQL with Diesel ORM
- **Containerization**: Docker & Docker Compose

## Getting Started

### Prerequisites

- Rust (1.84+)
- PostgreSQL
- Docker & Docker Compose (optional)

### Environment Setup

Create a `.env` file with the following variables:

```
DATABASE_URL=postgres://username:password@localhost/database_name
PORT=8282
```

### Running Locally

```bash
# Install dependencies and build
cargo build

# Run migrations
diesel migration run

# Start the server
cargo run
```

### Docker Deployment

```bash
# Build and start with Docker Compose
docker-compose up -d rest_utils

# Or use the pre-built image
docker-compose up -d utils
```

## API Endpoints

### Health Check
```
GET /
```

### Generate Token
```
GET /tokens
```

### Get Utilities
```
GET /utils/first
Header: Token: <your-token>
```

## Development

```bash
# Watch mode for development
make watch

# Build release version
make release
```
