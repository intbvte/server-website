DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM pg_constraint
                       WHERE conname = 'unique_minecraft_uuid') THEN
            ALTER TABLE users
                ADD CONSTRAINT unique_minecraft_uuid UNIQUE (minecraft_uuid);
        END IF;
    END
$$;