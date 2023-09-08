CREATE TABLE IF NOT EXISTS player_infos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid text NOT NULL,
    ip text NOT NULL,
    client_name text NOT NULL,
    name text NOT NULL,
    service_tag text NOT NULL,
    primary_color text NOT NULL,
    updated DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(uid) REFERENCES players(uid) ON UPDATE CASCADE
)
