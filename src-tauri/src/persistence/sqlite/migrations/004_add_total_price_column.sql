ALTER TABLE journals ADD COLUMN total_price INTEGER DEFAULT 0;
ALTER TABLE stocktakings ADD COLUMN total_price INTEGER DEFAULT 0;

ALTER TABLE journal_records ADD COLUMN total_price INTEGER DEFAULT 0;
ALTER TABLE stocktaking_records ADD COLUMN total_price INTEGER DEFAULT 0;

UPDATE journal_records
SET total_price = unit_price * quantity;

UPDATE stocktaking_records
SET total_price = unit_price * quantity;

UPDATE journals
SET total_price = (
    SELECT COALESCE(SUM(total_price), 0)
    FROM journal_records jr
    WHERE jr.journal_id = journals.id
);

UPDATE stocktakings
SET total_price = (
    SELECT COALESCE(SUM(total_price), 0)
    FROM stocktaking_records sr
    WHERE sr.stocktaking_id = stocktakings.id
);


PRAGMA user_version = 4;