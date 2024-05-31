PRAGMA foreign_keys = OFF;
CREATE TABLE new_attendees (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    event_id INTEGER NOT NULL,
    CONSTRAINT attendees_event_id_fk FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO new_attendees (id, name, email, created_at, event_id) SELECT id, name, email, created_at, event_id FROM attendees;
DROP TABLE attendees;
ALTER TABLE new_attendees RENAME TO attendees;
CREATE UNIQUE INDEX attendees_event_id_email_key ON attendees (event_id, email);
PRAGMA foreign_key_check;
PRAGMA foreign_keys = ON;
