-- Create the symbols table
CREATE TABLE symbols (
    id BIGSERIAL PRIMARY KEY,
    code CHAR(3) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NULL DEFAULT NULL
);

CREATE TRIGGER symbols_updated_at
BEFORE UPDATE ON symbols
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();
