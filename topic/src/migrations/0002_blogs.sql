-- blogs table (belongs to a blog)
CREATE TABLE blogs (
                          id                  uuid PRIMARY KEY DEFAULT gen_random_uuid(),
                          topic_id             uuid        NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
                          name                varchar(255)        NOT NULL,
                          meta_tag            varchar(255)        NOT NULL,
                          small_description   varchar(255)        NOT NULL,
                          meta_description    varchar(255)        NOT NULL,
                          tile_image          varchar(255)        NOT NULL,
                          banner_image        varchar(255)        NOT NULL,
                          separate_video      boolean     NOT NULL DEFAULT false,
                          video_url           varchar(255),                           -- optional video URL
                          created_on          timestamptz NOT NULL DEFAULT now(),
                          description         text
);

-- -- Fast listing newest-first per topic
-- CREATE INDEX ix_blogs_topic_created ON blogs (topic_id, created_on DESC);
