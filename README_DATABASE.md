# OpenMMO PostgreSQL Development Environment Setup

This setup provides a complete PostgreSQL development environment for OpenMMO following the MVP specification.

## 🚀 Quick Start

### 1. Install Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Install Docker (Ubuntu/Debian example)
sudo apt-get update
sudo apt-get install -y docker.io docker-compose
sudo usermod -aG docker $USER
# Log out and back in for group changes to take effect
```

### 2. Set Up Environment

```bash
# Clone the repository (if not already done)
git clone <repository-url>
cd OpenMMO

# Copy environment template
cp .env.example .env

# Make setup script executable
chmod +x setup.sh

# Run the automated setup
./setup.sh
```

### 3. Manual Setup (Alternative)

```bash
# Start PostgreSQL database
docker-compose up -d db

# Wait for database to be ready (10-15 seconds)
sleep 15

# Run database migrations
cd server
cargo sqlx migrate run

# Start the server
cargo run
```

## 📋 What's Included

### Database Configuration
- **PostgreSQL 16** with Alpine Linux for efficiency
- **Connection pooling** with SQLx (10 max connections, 2 min connections)
- **Health checks** for container monitoring
- **Volume persistence** for data durability
- **Proper networking** with port 5432 exposed

### Database Schema
The following tables are created via migrations:

1. **accounts** - User authentication and management
2. **characters** - Player characters with stats and position
3. **character_stats** - Detailed character statistics
4. **inventory_items** - Item management system
5. **equipped_items** - Currently equipped gear
6. **progression** - XP, levels, and advancement
7. **quest_state** - Quest progress tracking

### Server Features
- **Database connectivity** with proper error handling
- **Health check endpoints** (`/health` and `/health/db`)
- **Connection pooling** for performance
- **Migration management** with SQLx
- **Structured logging** with tracing
- **Environment configuration** via .env files

## 🔧 Configuration

### Environment Variables (.env)

```env
# Database Configuration
DATABASE_URL=postgres://openmmo_user:openmmo_dev_password@localhost:5432/openmmo

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Logging
RUST_LOG=openmmo=debug,tower_http=debug

# Development Settings
ENVIRONMENT=development
```

### Docker Compose Configuration

The `docker-compose.yml` includes:
- PostgreSQL 16 with Alpine Linux
- Proper environment variables
- Health checks
- Volume persistence
- Network isolation

## 🗄️ Database Operations

### Migration Management

```bash
# Create new migration
cargo sqlx migrate add <migration_name>

# Run migrations
cargo sqlx migrate run

# Check migration status
cargo sqlx migrate info

# Revert last migration
cargo sqlx migrate revert
```

### Database Access

```bash
# Connect to database
docker-compose exec db psql -U openmmo_user -d openmmo

# View tables
\dt

# View table structure
\d accounts
\d characters
```

## 🧪 Testing

Lightweight database-related unit tests are included (no live Postgres required):

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_database_connection_string_parsing
```

### Test Coverage
- Database connection string parsing (URL validation only)
- Model serialization/deserialization (e.g., accounts)
- Error type formatting
- Character model validation/conversion checks

> Note: Integration tests that exercise a live database connection are not implemented yet.

## 🔍 Health Checks

### Server Health Endpoints

```bash
# Basic server health
curl http://localhost:8080/health

# Database connectivity
curl http://localhost:8080/health/db
```

### Expected Responses

```json
// Basic health
{
  "status": "healthy",
  "timestamp": "2025-12-13T14:40:59.123Z"
}

// Database health
{
  "status": "healthy",
  "database": "connected",
  "timestamp": "2025-12-13T14:40:59.123Z"
}
```

## 🛠️ Development Workflow

### 1. Making Schema Changes

```bash
# Create migration
cargo sqlx migrate add add_new_feature_table

# Edit migration file
vim migrations/<timestamp>_add_new_feature_table.sql

# Test migration
cargo sqlx migrate run

# Update server code to use new schema
# ...

# Run tests
cargo test
```

### 2. Database Query Development

The server uses SQLx with runtime query checking:

```rust
// Example: Find account by username
let account = sqlx::query(
    "SELECT * FROM accounts WHERE username = $1 AND is_active = true"
)
.bind(username)
.fetch_optional(pool)
.await?;
```

### 3. Error Handling

All database operations use proper error handling:

```rust
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database query failed: {0}")]
    QueryFailed(#[from] sqlx::Error),
    #[error("Account not found")]
    AccountNotFound,
    // ... more error types
}
```

## 🔒 Security Considerations

### Development Environment
- **Simple passwords** for easy development
- **Local access only** by default
- **No SSL/TLS** in development

### Production Readiness
- **Strong passwords** required
- **SSL/TLS encryption** needed
- **Connection limits** should be configured
- **Backup strategies** must be implemented

## 📊 Performance Tuning

### Connection Pool Settings
- **Max connections**: 10 (development)
- **Min connections**: 2 (always ready)
- **Acquire timeout**: 30 seconds
- **Idle timeout**: 10 minutes
- **Max lifetime**: 30 minutes

### Database Indexes
All tables include appropriate indexes:
- Primary keys (UUID)
- Foreign key relationships
- Frequently queried columns
- Unique constraints where needed

## 🚨 Troubleshooting

### Common Issues

1. **Database connection failed**
   ```bash
   # Check if database is running
   docker-compose ps
   
   # Check database logs
   docker-compose logs db
   
   # Test connection manually
   docker-compose exec db psql -U openmmo_user -d openmmo
   ```

2. **Migration errors**
   ```bash
   # Check migration status
   cargo sqlx migrate info
   
   # Re-run migrations
   cargo sqlx migrate run
   ```

3. **Server won't start**
   ```bash
   # Check environment variables
   cat .env
   
   # Check server logs
   cargo run --verbose
   ```

### Getting Help

- Check the logs: `docker-compose logs db` and `cargo run`
- Verify environment: `cat .env`
- Test database: `docker-compose exec db psql -U openmmo_user -d openmmo`
- Review documentation: `docs/database-setup.md`

## 📚 Additional Resources

- [SQLx Documentation](https://docs.rs/sqlx/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [OpenMMO Specification](open_mmorpg_mvp_specification.md)

---

This setup provides a solid foundation for OpenMMO development with proper database integration, following all the requirements from the MVP specification and AGENTS.md guidelines.
