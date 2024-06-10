use deadpool_diesel::sqlite::Pool;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    domain::{Attendee, Event},
    error::AppError,
    infra::schema::{attendees, check_ins, events},
};

use super::{event::EventDB, Repository};

#[derive(Clone, Debug, PartialEq, Queryable)]
pub struct AttendeeDB {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub event_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub checked_in_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Debug, PartialEq, Queryable)]
pub struct AttendeeWithoutCheckInDB {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub event_id: String,
}

#[derive(Clone)]
pub struct AttendeeRepository {
    pool: Box<Pool>,
}

impl Repository for AttendeeRepository {
    type Id = i32;
    type Model = Attendee;
    type Error = AppError;

    async fn create(&self, model: &Self::Model) -> Result<Self::Model, Self::Error> {
        let conn = self.pool.get().await?;
        let attendee = model.clone();
        let id = conn
            .interact(move |conn| {
                diesel::insert_into(attendees::table)
                    .values((
                        attendees::name.eq(attendee.name()),
                        attendees::email.eq(attendee.email()),
                        attendees::event_id.eq(attendee.event().id().to_string()),
                    ))
                    .returning(attendees::id)
                    .get_result::<i32>(conn)
            })
            .await??;

        let mut model = model.clone();
        model.set_id(id);

        Ok(model)
    }

    async fn get(&self, id: &Self::Id) -> Result<Self::Model, Self::Error> {
        let conn = self.pool.get().await?;
        let id = id.clone();
        let attendee = conn
            .interact(move |conn| {
                attendees::table
                    .find(id)
                    .first::<AttendeeWithoutCheckInDB>(conn)
                    .optional()
            })
            .await??;

        let attendee = match attendee {
            Some(attendee) => attendee,
            None => return Err(AppError::NotFound("Attendee not found.".to_string())),
        };

        let event_id = attendee.event_id.clone();

        let event = conn
            .interact(move |conn| events::table.find(event_id).first::<EventDB>(conn))
            .await??;

        let event_id = attendee.event_id.clone();

        let attendees_amount = conn
            .interact(move |conn| {
                attendees::table
                    .filter(attendees::event_id.eq(event_id))
                    .count()
                    .get_result::<i64>(conn)
            })
            .await??;

        Ok(Attendee::new(
            Some(attendee.id),
            attendee.name,
            attendee.email,
            Event::new(
                Some(event.id.parse().unwrap()),
                event.title,
                event.details,
                Some(event.slug),
                event.maximum_attendees,
                attendees_amount as i32,
            ),
            Some(attendee.created_at),
            None,
        ))
    }
}

impl AttendeeRepository {
    pub fn new(pool: Box<Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_from_event(
        &self,
        event_id: Uuid,
        filter: Option<String>,
        page_index: i64,
        page_size: i64,
    ) -> Result<(Vec<Attendee>, i64), AppError> {
        let conn = self.pool.get().await?;
        let event_id_ = event_id.to_string();

        let attendees = conn
            .interact(move |conn| {
                let mut attendees_query = attendees::table
                    .into_boxed()
                    .filter(attendees::event_id.eq(event_id_))
                    .left_join(check_ins::table)
                    .select((
                        attendees::id,
                        attendees::name,
                        attendees::email,
                        attendees::event_id,
                        attendees::created_at,
                        check_ins::created_at.nullable(),
                    ));

                if let Some(name) = filter {
                    attendees_query =
                        attendees_query.filter(attendees::name.like(format!("%{name}%")));
                }

                attendees_query
                    .limit(page_size)
                    .offset(page_size * page_index)
                    .load::<AttendeeDB>(conn)
            })
            .await??;

        let event_id_ = event_id.to_string();

        let total = conn
            .interact(move |conn| {
                let total = attendees::table
                    .filter(attendees::event_id.eq(event_id_))
                    .count()
                    .get_result::<i64>(conn);
                total
            })
            .await??;

        let event_id = event_id.to_string();

        let event = conn
            .interact(move |conn| events::table.find(event_id).first::<EventDB>(conn))
            .await??;

        Ok((
            attendees
                .iter()
                .map(|attendee| {
                    Attendee::new(
                        Some(attendee.id),
                        attendee.name.clone(),
                        attendee.email.clone(),
                        Event::new(
                            Some(event.id.clone().parse().unwrap()),
                            event.title.clone(),
                            event.details.clone(),
                            Some(event.slug.clone()),
                            event.maximum_attendees.clone(),
                            total.clone() as i32,
                        ),
                        Some(attendee.created_at),
                        attendee.checked_in_at,
                    )
                })
                .collect::<Vec<_>>(),
            total,
        ))
    }

    pub async fn check_in(&self, attendee: &Attendee) -> Result<(), AppError> {
        let conn = self.pool.get().await?;
        let attendee_id = attendee.id();

        conn.interact(move |conn| {
            diesel::insert_into(check_ins::table)
                .values(check_ins::attendee_id.eq(attendee_id))
                .execute(conn)
        })
        .await??;

        Ok(())
    }
}
