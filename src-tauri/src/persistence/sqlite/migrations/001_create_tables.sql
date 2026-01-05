-- create tables

-- table of sequence for suplier id generation
CREATE TABLE suppliers_id_sequence (
    name TEXT PRIMARY KEY,
    value INTEGER DEFAULT 0
);

INSERT INTO suppliers_id_sequence (
    name,
    value
) VALUES (
    'supplier_id',
    0
);

-- table of sequence for supply id generation
CREATE TABLE supplies_id_sequence (
    name TEXT PRIMARY KEY,
    value INTEGER DEFAULT 0
);

INSERT INTO supplies_id_sequence (
    name,
    value
) VALUES (
    'supply_id',
    0
);

-- table of sequence for journal id generation
CREATE TABLE journals_id_sequence (
    name TEXT PRIMARY KEY,
    value INTEGER DEFAULT 0
);

INSERT INTO journals_id_sequence (
    name,
    value
) VALUES (
    'journal_id',
    0
);

-- table of sequence for stocktaking id generation
CREATE TABLE stocktakings_id_sequence (
    name TEXT PRIMARY KEY,
    value INTEGER DEFAULT 0
);

INSERT INTO stocktakings_id_sequence (
    name,
    value
) VALUES (
    'stocktaking_id',
    0
);

-- table of suppliers
CREATE TABLE suppliers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

-- table of supplies
CREATE TABLE supplies (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    unit_name TEXT NOT NULL,
    supplier_id INTEGER,
    FOREIGN KEY(supplier_id) REFERENCES suppliers(id)
);

-- table of journals
CREATE TABLE journals (
    id INTEGER PRIMARY KEY,
    recorded_at INTEGER NOT NULL -- timestamp jorunal recorded at
);

-- table of journal records
CREATE TABLE journal_records (
    supply_id INTEGER NOT NULL,
    supply_name TEXT NOT NULL,
    supplier_id INTEGER NOT NULL,
    supplier_name TEXT NOT NULL,
    unit_name TEXT NOT NULL,
    unit_price INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    journal_id INTEGER NOT NULL,
    FOREIGN KEY(supplier_id) REFERENCES suppliers(id),
    FOREIGN KEY(supply_id) REFERENCES supplies(id),
    FOREIGN KEY(journal_id) REFERENCES journals(id)
);

-- table of stocktakings
CREATE TABLE stocktakings (
    id INTEGER PRIMARY KEY,
    recorded_at INTEGER NOT NULL -- timestamp jorunal recorded at
);

-- table of stocktaking records
CREATE TABLE stocktaking_records (
    unit_name TEXT NOT NULL,
    unit_price INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    supply_id INTEGER NOT NULL,
    supply_name TEXT NOT NULL,
    stocktaking_id INTEGER NOT NULL,
    FOREIGN KEY(supply_id) REFERENCES supplies(id),
    FOREIGN KEY(stocktaking_id) REFERENCES stocktakings(id)
);

PRAGMA user_version = 1;