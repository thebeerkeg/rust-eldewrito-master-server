CREATE TABLE IF NOT EXISTS players (
    id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
    uid VARCHAR(16) NOT NULL UNIQUE,
    updated DATETIME DEFAULT CURRENT_TIMESTAMP
)
