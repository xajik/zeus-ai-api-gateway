-- 31 12 2023: key_value_vector_index Up Migration

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE key_value_vector (
    id SERIAL PRIMARY KEY,
    vector_data vector(768),
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Trigger to update 'updated_at' timestamp on row update
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = CURRENT_TIMESTAMP;
   RETURN NEW;   
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER update_key_value_vector_modtime
    BEFORE UPDATE ON key_value_vector
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

-- Create a vector index
CREATE INDEX key_value_vector_idx ON key_value_vector USING ivfflat (vector_data);