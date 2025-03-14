-- Check if column exists first
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 
        FROM information_schema.columns 
        WHERE table_name='servers' 
        AND column_name='server_type'
    ) THEN
        ALTER TABLE servers ADD COLUMN server_type VARCHAR(255);
    END IF;
END $$;