CREATE TABLE check_ins (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    attendee_id INTEGER NOT NULL,
    CONSTRAINT check_ins_attendee_id_fk FOREIGN KEY (attendee_id) REFERENCES attendees(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX check_ins_attendee_id_key ON check_ins (attendee_id);
