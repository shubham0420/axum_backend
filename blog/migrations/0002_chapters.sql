-- Chapters table (belongs to a blog)
CREATE TABLE chapters (
                          id                  uuid PRIMARY KEY DEFAULT gen_random_uuid(),
                          blog_id             uuid        NOT NULL REFERENCES blogs(id) ON DELETE CASCADE,
                          name                text        NOT NULL,
                          meta_tag            text        NOT NULL,
                          small_description   text        NOT NULL,
                          meta_description    text        NOT NULL,
                          tile_image          text        NOT NULL,
                          banner_image        text        NOT NULL,
                          separate_video      boolean     NOT NULL DEFAULT false,
                          video_url           text,                           -- optional video URL
                          created_on          timestamptz NOT NULL DEFAULT now(),
                          description         text
);

-- Prevent duplicate chapter slugs *within the same blog*
CREATE UNIQUE INDEX ux_chapters_blog_meta_tag ON chapters (blog_id, meta_tag);

-- Fast listing newest-first per blog
CREATE INDEX ix_chapters_blog_created ON chapters (blog_id, created_on DESC);
