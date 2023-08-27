CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS articles (
  id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
  source_id VARCHAR( 60 ),
  source_name VARCHAR( 90 ) NOT NULL,
  author TEXT,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  content TEXT NOT NULL,
  url TEXT NOT NULL,
  url_to_image TEXT,
  published_at bigint NOT NULL,
  created_at bigint NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW()) * 1000),
  updated_at bigint NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW()) * 1000),
  CONSTRAINT unique_url UNIQUE (url)
);

CREATE UNIQUE INDEX articles_url_idx ON articles (url);
CREATE INDEX articles_published_date_idx ON articles
  (((to_timestamp(published_at / 1000) at time zone 'UTC')::DATE));
