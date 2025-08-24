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

CREATE TABLE IF NOT EXISTS orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    order_type TEXT NOT NULL,
    bot_name TEXT NOT NULL,
    entry_price INTEGER NOT NULL,
    exit_price INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    pnl INTEGER NOT NULL,
    roe INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    closed_at TEXT NOT NULL,
    fee INTEGER NOT NULL,
    leverage INTEGER NOT NULL,
)

CREATE TABLE IF NOT EXISTS bot_state (
    name TEXT PRIMARY KEY,
    symbol TEXT,
    timeframe: TEXT,
    strategy_name TEXT,
    capital INTEGER,
    bot_group TEXT,
    is_not_active INTEGER,
    wins NUMBER,
    losses NUMBER,
    log TEXT,
    started_at TEXT,
    last_scanned TEXT,
    leverage NUMBER,
    take_profit_ratio NUMBER,
    stop_loss_ratio NUMBER,
    is_trailing_stop_active bool,
    trailing_stop_activation_point NUMBER,
    in_pos INTEGER,
    order_type TEXT,
    order_created_at TEXT,
    order_scanned_at TEXT,
    order_quantity NUMBER,
    order_capital NUMBER,
    order_capital_with_leverage NUMBER,
    order_entry_price NUMBER,
    order_stop_loss NUMBER,
    order_take_profit NUMBER,
    order_fee NUMBER,
    pnl NUMBER,
    roe NUMBER,
)

INSERT INTO bot_state (
    name,
    symbol,
    timeframe,
    strategy_name,
    capital,
    bot_group,
    is_not_active,
    wins,
    losses,
    log,
    started_at,
    last_scanned,
    leverage,
    take_profit_ratio,
    stop_loss_ratio,
    is_trailing_stop_active,
    trailing_stop_activation_point,
    in_pos,
    order_type,
    order_created_at,
    order_scanned_at,
    order_quantity,
    order_capital,
    order_capital_with_leverage,
    order_entry_price,
    order_stop_loss,
    order_take_profit,
    order_fee,
    pnl,
    roe,
) VALUES (?1, ?2, ?3, ?4, ?5, ?6 ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30)