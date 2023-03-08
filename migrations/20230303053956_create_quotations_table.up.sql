--postgresql table quotations
CREATE TABLE quotations (
    id BIGSERIAL  PRIMARY KEY,
    symbol_id BIGINT  NOT NULL,
    base_symbol_id BIGINT  NOT NULL,
    date DATE NOT NULL,
    open DECIMAL(20, 8) NOT NULL,
    close DECIMAL(20, 8) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT NULL,
    FOREIGN KEY (symbol_id) REFERENCES symbols(id) ON DELETE CASCADE,
    FOREIGN KEY (base_symbol_id) REFERENCES symbols(id) ON DELETE CASCADE
);

CREATE TRIGGER quotations_updated_at
BEFORE UPDATE ON quotations
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();
