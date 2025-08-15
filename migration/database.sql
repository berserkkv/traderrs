ALTER TABLE bots RENAME TO bots_old;

CREATE TABLE IF NOT EXISTS bots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    capital REAL NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now','+3 hours')),
    wins INTEGER NOT NULL DEFAULT 0,
    losses INTEGER NOT NULL DEFAULT 0,
);

INSERT INTO bots (id, name, created_at) SELECT id, name, created_at FROM bots_old;

DROP TABLE bots_old;


CREATE TABLE IF NOT EXISTS bots (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    capital REAL NOT NULL,
                    created_at TEXT NOT NULL DEFAULT (datetime('now', '+3 hours')),
                    wins INTEGER NOT NULL DEFAULT 0,
                    losses INTEGER NOT NULL DEFAULT 0
                );