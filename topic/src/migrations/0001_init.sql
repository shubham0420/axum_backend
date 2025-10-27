CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE topics (
                       id                  uuid PRIMARY KEY DEFAULT gen_random_uuid(),
                       name                varchar(200) NOT NULL,
                       meta_tag            varchar(50) NOT NULL,          -- e.g. SEO tag/slug
                       small_description   varchar(255) NOT NULL,
                       meta_description    varchar(255) NOT NULL,
                       tile_image          varchar(255) NOT NULL,          -- card/tile image URL
                       banner_image        varchar(255) NOT NULL,          -- banner image URL
                       youtube_video_url   varchar(255),                          -- optional
                       created_on          timestamptz NOT NULL DEFAULT now()
);