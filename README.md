## Development Notes

```bash
# Install `sqlx-cli`:
cargo install sqlx-cli

# Spin up a Postgres instance in the background:
docker-compose down -v && docker-compose up -d

# Create database:
source .env
sqlx database create

# Create and run migrations
sqlx migrate add add_article_table
sqlx migrate run
```

Others:
```bash
# Enable offline mode
cargo sqlx prepare

# Clean up
sqlx database drop
```
