use crate::{
    domain::Event,
    error::AppError,
    infra::schema::{attendees, check_ins, events},
};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use uuid::Uuid;

use super::Repository;

#[derive(Clone, Debug, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = events)]
pub struct EventDB {
    pub id: String,
    pub title: String,
    pub details: Option<String>,
    pub slug: String,
    pub maximum_attendees: Option<i32>,
}

#[derive(Clone)]
pub struct EventRepository {
    pool: Box<Pool>,
}

impl Repository for EventRepository {
    type Model = Event;
    type Id = Uuid;
    type Error = AppError;

    async fn create(&self, model: &Self::Model) -> Result<Self::Model, Self::Error> {
        let conn = self.pool.get().await?;
        let event = model.clone();
        conn.interact(move |conn| {
            diesel::insert_into(events::table)
                .values((
                    events::id.eq(event.id().to_string()),
                    events::title.eq(event.title()),
                    events::details.eq(event.details()),
                    events::slug.eq(event.slug()),
                    events::maximum_attendees.eq(event.maximum_attendees()),
                ))
                .execute(conn)
        })
        .await??;

        Ok(model.clone())
    }

    async fn get(&self, id: &Self::Id) -> Result<Self::Model, Self::Error> {
        let conn = self.pool.get().await?;
        let id_ = id.clone();
        let id = id.to_string();

        let (event, attendees_amount) = conn
            .interact(move |conn| {
                let event = events::table
                    .find(id.clone())
                    .first::<EventDB>(conn)
                    .optional();

                let attendees_amount = attendees::table
                    .left_join(check_ins::table)
                    .filter(attendees::event_id.eq(id))
                    .count()
                    .get_result::<i64>(conn);

                (event, attendees_amount)
            })
            .await?;

        match event? {
            Some(event) => Ok(Event::new(
                Some(id_),
                event.title,
                event.details,
                Some(event.slug),
                event.maximum_attendees,
                attendees_amount.unwrap_or(0) as i32,
            )),
            None => {
                return Err(AppError::NotFound(
                    "Event with this ID not found".to_string(),
                ))
            }
        }
    }
}

impl EventRepository {
    pub fn new(pool: Box<Pool>) -> Self {
        Self { pool }
    }
}
