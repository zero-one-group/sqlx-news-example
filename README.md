## Development Notes

```bash
# Install `sqlx-cli`:
cargo install sqlx-cli

# Spin up a Postgres instance in the background:
docker-compose down -v && docker-compose up -d

# Create database:
source .env
sqlx database create
```

Others:
```bash
# Clean up
sqlx database drop
```
