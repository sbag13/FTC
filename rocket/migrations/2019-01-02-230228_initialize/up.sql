CREATE TABLE users (
    -- id INTEGER PRIMARY KEY AUTOINCREMENT,
    mail VARCHAR NOT NULL PRIMARY KEY,
    password VARCHAR NOT NULL
);

CREATE TABLE auctions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    description TEXT NOT NULL,
    price REAL NOT NULL,
    date INTEGER NOT NULL
);

CREATE TABLE buynows (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    description TEXT NOT NULL,
    price REAL NOT NULL,
    amount INTEGER NOT NULL
);