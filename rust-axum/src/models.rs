use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Insertable, Queryable, Selectable, Serialize, Debug, PartialEq)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub details: Option<String>,
    pub slug: String,
    pub maximum_attendees: Option<i32>,
}

#[derive(
    Insertable, Queryable, Selectable, Serialize, Debug, Associations, Identifiable, PartialEq,
)]
#[diesel(belongs_to(Event))]
pub struct Attendee {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub event_id: String,
}

#[derive(
    Insertable, Queryable, Selectable, Serialize, Debug, PartialEq, Associations, Identifiable,
)]
#[diesel(belongs_to(Attendee))]
pub struct CheckIn {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub attendee_id: i32,
}

#[derive(Queryable, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttendeeWithCheckIn {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub check_in_at: Option<NaiveDateTime>,
}
