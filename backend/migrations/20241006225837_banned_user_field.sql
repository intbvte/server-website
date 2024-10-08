ALTER TABLE users
    ADD IF NOT EXISTS banned BOOLEAN DEFAULT FALSE NOT NULL;

ALTER TABLE users
    ALTER minecraft_uuid TYPE UUID USING minecraft_uuid::uuid;