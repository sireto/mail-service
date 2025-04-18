DO $$
DECLARE
    enum_exists boolean;
    col_exists boolean;
BEGIN
    -- 1. Create server_type enum if not exists
    SELECT EXISTS (
        SELECT 1 FROM pg_type WHERE typname = 'server_type'
    ) INTO enum_exists;
    
    IF NOT enum_exists THEN
        CREATE TYPE server_type AS ENUM ('SMTP', 'AWS');
    END IF;

    -- 2. Add aws_credentials column if not exists
    SELECT EXISTS (
        SELECT 1 
        FROM information_schema.columns 
        WHERE table_name = 'servers' 
        AND column_name = 'aws_credentials'
    ) INTO col_exists;
    
    IF NOT col_exists THEN
        ALTER TABLE servers 
        ADD COLUMN aws_credentials JSONB NOT NULL DEFAULT '{}';
    END IF;

    -- 3. Convert server_type column to ENUM type safely
    IF EXISTS (
        SELECT 1 
        FROM information_schema.columns 
        WHERE table_name = 'servers' 
        AND column_name = 'server_type' 
        AND data_type = 'character varying'
    ) THEN
        -- Add temporary check constraint
        ALTER TABLE servers 
        ADD CONSTRAINT temp_check_server_type 
        CHECK (server_type IN ('SMTP', 'AWS'));

        -- Convert column type using USING clause
        ALTER TABLE servers 
        ALTER COLUMN server_type TYPE server_type 
        USING server_type::server_type;

        -- Remove temporary constraint
        ALTER TABLE servers 
        DROP CONSTRAINT temp_check_server_type;
    END IF;
END $$;