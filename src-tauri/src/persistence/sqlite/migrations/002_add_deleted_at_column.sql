-- add deleted_at column

ALTER TABLE supplies ADD COLUMN deleted_at INTEGER;
ALTER TABLE suppliers ADD COLUMN deleted_at INTEGER;
ALTER TABLE journals ADD COLUMN deleted_at INTEGER;
ALTER TABLE stocktakings ADD COLUMN deleted_at INTEGER;

PRAGMA user_version = 2;