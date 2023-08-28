# sqlx-news-example

This is an example command-line app to combine the use of [Postgres](https://www.postgresql.org/), [sqlx](https://github.com/launchbadge/sqlx) and [NewsAPI](https://newsapi.org/). The main idea is to query news articles from NewsAPI, and dump it to a Postgres database with full type safety.

## Basic Usage

Populate `.env` based on `.env.example`. You can get the NewsAPI API token by signing up to NewsAPI for free.

```bash
# Spin up a Postgres instance in the background:
docker-compose down -v && docker-compose up -d

# Health check
cargo run -- --app health-check

# Fetch and dump articles
cargo run -- --app fetch-and-dump-articles --query "dogecoin elon musk"

# Print 10 random articles
cargo run -- --app print-articles --limit 10
```

## Development Notes

```bash
# Install `sqlx-cli`:
cargo install sqlx-cli

# Create database:
source .env
sqlx database create

# Create and run migrations
sqlx migrate add -r add_article_table

# Run/revert migrations
sqlx migrate run
sqlx migrate revert
```

Others:
```bash
# PSQL session:
psql -d $DATABASE_URL -P expanded=auto

# Enable offline mode:
cargo sqlx prepare

# Clean up:
sqlx database drop
```
