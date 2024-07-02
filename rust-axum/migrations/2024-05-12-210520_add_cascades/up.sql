CREATE TABLE new_attendees (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    event_id TEXT NOT NULL,
    CONSTRAINT attendees_event_id_fk FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE new_check_ins (
    id SERIAL NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    attendee_id INTEGER NOT NULL,
    CONSTRAINT check_ins_attendee_id_fk FOREIGN KEY (attendee_id) REFERENCES new_attendees(id) ON DELETE CASCADE ON UPDATE CASCADE
);
INSERT INTO new_attendees (id, name, email, created_at, event_id) SELECT id, name, email, created_at, event_id FROM attendees;
INSERT INTO new_check_ins (id, created_at, attendee_id) SELECT id, created_at, attendee_id FROM check_ins;
DROP TABLE check_ins;
DROP TABLE attendees;
ALTER TABLE new_attendees RENAME TO attendees;
ALTER TABLE new_check_ins RENAME TO check_ins;
CREATE UNIQUE INDEX attendees_event_id_email_key ON attendees (event_id, email);
CREATE UNIQUE INDEX check_ins_attendee_id_key ON check_ins (attendee_id);
