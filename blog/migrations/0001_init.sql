-- Enable UUID generation via pgcrypto (preferred) or switch to "uuid-ossp".
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Blogs table
CREATE TABLE blogs (
                       id                  uuid PRIMARY KEY DEFAULT gen_random_uuid(),
                       name                text        NOT NULL,
                       meta_tag            text        NOT NULL,          -- e.g. SEO tag/slug
                       small_description   text        NOT NULL,
                       meta_description    text        NOT NULL,
                       tile_image          text        NOT NULL,          -- card/tile image URL
                       banner_image        text        NOT NULL,          -- banner image URL
                       youtube_video_url   text,                          -- optional
                       created_on          timestamptz NOT NULL DEFAULT now()
);

-- Prevent duplicate SEO tags across blogs
CREATE UNIQUE INDEX ux_blogs_meta_tag ON blogs (meta_tag);

-- Useful partial text-search index (optional)
-- CREATE INDEX ix_blogs_name_trgm ON blogs USING gin (name gin_trgm_ops);
