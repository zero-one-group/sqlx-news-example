CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS articles (
  id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
  source_id VARCHAR( 60 ) NOT NULL,
  source_name VARCHAR( 90 ) NOT NULL,
  author TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  content TEXT NOT NULL,
  url TEXT NOT NULL,
  url_to_image TEXT NOT NULL,
  published_at bigint NOT NULL
);
