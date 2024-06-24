CREATE TABLE IF NOT EXISTS users
(
    discord_id       BIGINT PRIMARY KEY                                 NOT NULL,
    discord_username TEXT                                               NOT NULL,
    minecraft_uuid   VARCHAR(36),
    created_at       TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_updated     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    is_admin         BOOLEAN                  DEFAULT FALSE             NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions
(
    id            SERIAL PRIMARY KEY       NOT NULL,
    user_id       BIGINT                   NOT NULL UNIQUE,
    session_id    TEXT                     NOT NULL,
    access_token  TEXT                     NOT NULL,
    refresh_token TEXT                     NOT NULL,
    expires_at    TIMESTAMP WITH TIME ZONE NOT NULL,
    expired       BOOLEAN DEFAULT FALSE    NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (discord_id)
);