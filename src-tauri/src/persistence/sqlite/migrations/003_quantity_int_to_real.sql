UPDATE stocktaking_records
SET quantity = quantity * 100.0;

UPDATE journal_records
SET quantity = quantity * 100.0;

PRAGMA user_version = 3;