ALTER TABLE sessions
    ALTER session_id TYPE UUID USING session_id::uuid;

DO
$$
    BEGIN
        IF EXISTS (SELECT 1
                   FROM pg_constraint
                   WHERE conname = 'sessions_user_id_key') THEN
            ALTER TABLE sessions
                DROP CONSTRAINT sessions_user_id_key;
        END IF;
    END
$$;