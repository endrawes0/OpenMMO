# OpenMMO Database Setup

This document explains how to set up and run the PostgreSQL database for OpenMMO development.

## Prerequisites

- Docker and Docker Compose
- Rust and Cargo (for the server)

## Quick Start

1. **Start the database:**
   ```bash
   docker-compose up -d db
   ```

2. **Set up environment variables:**
   ```bash
   cp .env.example .env
   # Edit .env if needed (defaults should work for development)
   ```

3. **Run database migrations:**
   ```bash
   cd server
   cargo sqlx migrate run
   ```

4. **Start the server:**
   ```bash
   cargo run
   ```

## Database Configuration

The database is configured via Docker Compose with the following settings:

- **Database name**: `openmmo`
- **User**: `openmmo_user`
- **Password**: `openmmo_dev_password`
- **Port**: `5432` (mapped to host)

### Environment Variables

Create a `.env` file based on `.env.example`:

```env
DATABASE_URL=postgres://openmmo_user:openmmo_dev_password@localhost:5432/openmmo
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=openmmo=debug,tower_http=debug
ENVIRONMENT=development
```

## Database Schema

The database includes the following tables:

### Core Tables
- `accounts` - User accounts with authentication
- `characters` - Player characters with basic stats
- `character_stats` - Detailed character statistics
- `inventory_items` - Item inventory management
- `equipped_items` - Currently equipped items
- `progression` - XP, levels, and skill progression
- `quest_state` - Quest progress tracking

### Key Features

- **UUID primary keys** for all entities
- **Timestamp tracking** with automatic updates
- **Foreign key constraints** for data integrity
- **Indexes** for performance optimization
- **JSONB support** for flexible data storage

## Migration Management

### Creating New Migrations

```bash
cd server
cargo sqlx migrate add <migration_name>
```

### Running Migrations

```bash
cargo sqlx migrate run
```

### Reverting Migrations

```bash
cargo sqlx migrate revert
```

## Database Operations

The server includes several health check endpoints:

- `GET /health` - Basic server health
- `GET /health/db` - Database connectivity check

## Development Workflow

1. Make schema changes in migration files
2. Test migrations locally
3. Update server code to use new schema
4. Run tests to verify functionality

## Security Notes

- **Development passwords** are intentionally simple
- **Production deployments** should use strong passwords
- **Connection pooling** is configured for development
- **SSL/TLS** should be enabled in production

## Troubleshooting

### Database Connection Issues

1. Check if Docker is running: `docker ps`
2. Verify database container: `docker-compose ps`
3. Check logs: `docker-compose logs db`
4. Test connection: `docker-compose exec db psql -U openmmo_user -d openmmo`

### Migration Issues

1. Verify migration files exist in `migrations/` directory
2. Check database connection: `cargo sqlx migrate info`
3. Run with verbose output: `RUST_LOG=debug cargo sqlx migrate run`

### Performance Issues

1. Monitor connection pool: Check server logs
2. Add indexes to slow queries
3. Consider connection pool sizing for your workload

## Data Access Patterns

The server uses SQLx with:
- **Compile-time checked queries** for safety
- **Connection pooling** for performance
- **Async/await** for non-blocking operations
- **Structured error handling** with thiserror

Example usage:

```rust
use crate::db::{AccountQueries, DatabaseError};

// Find account by username
let account = AccountQueries::find_by_username(&pool, "playername").await?;

// Create new character
let character = CharacterQueries::create_character(
    &pool,
    account_id,
    "CharacterName",
    "warrior",
    100,  // health
    100,  // max_health
    "rage", // resource_type
    0,    // resource_value
    100,  // max_resource
).await?;
```