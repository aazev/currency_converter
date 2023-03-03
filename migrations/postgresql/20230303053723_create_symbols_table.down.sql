-- Drop the symbols_updated_at trigger and the update_symbols_updated_at() function
DROP TRIGGER symbols_updated_at ON symbols;

-- Drop the symbols table
DROP TABLE symbols;