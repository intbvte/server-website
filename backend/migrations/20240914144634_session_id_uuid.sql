ALTER TABLE sessions
    ALTER session_id TYPE UUID USING session_id::uuid;

ALTER TABLE sessions
    DROP CONSTRAINT sessions_user_id_key;