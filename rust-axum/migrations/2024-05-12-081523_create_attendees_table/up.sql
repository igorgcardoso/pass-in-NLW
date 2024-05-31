CREATE TABLE attendees (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    event_id TEXT NOT NULL,
    CONSTRAINT attendees_event_id_fk FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE UNIQUE INDEX attendees_event_id_email_key ON attendees(event_id, email);
