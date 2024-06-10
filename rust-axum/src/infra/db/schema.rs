// @generated automatically by Diesel CLI.

diesel::table! {
    attendees (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
        event_id -> Text,
    }
}

diesel::table! {
    check_ins (id) {
        id -> Integer,
        created_at -> Timestamp,
        attendee_id -> Integer,
    }
}

diesel::table! {
    events (id) {
        id -> Text,
        title -> Text,
        details -> Nullable<Text>,
        slug -> Text,
        maximum_attendees -> Nullable<Integer>,
    }
}

diesel::joinable!(attendees -> events (event_id));
diesel::joinable!(check_ins -> attendees (attendee_id));

diesel::allow_tables_to_appear_in_same_query!(
    attendees,
    check_ins,
    events,
);
