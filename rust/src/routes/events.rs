use crate::{
    error::AppError,
    models::{Attendee, AttendeeWithCheckIn, Event},
    schema::*,
    utils::generate_slug,
    AppState,
};
use axum::{
    debug_handler,
    extract::{Query, State},
    Json,
};
use axum::{extract::Path, http::StatusCode};
use axum::{
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

pub fn router() -> Router<AppState> {
    let router = Router::new();
    let router = router
        .route("/events", post(create_event))
        .route("/events/:event_id", get(get_event))
        .route(
            "/events/:event_id/attendees",
            post(register_for_event).get(get_attendees),
        );

    router
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequest {
    #[validate(length(min = 4))]
    title: String,
    details: Option<String>,
    #[validate(range(min = 1))]
    maximum_attendees: Option<i32>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventResponse {
    event_id: uuid::Uuid,
}

#[debug_handler]
#[utoipa::path(
    post,
    tag = "events",
    path = "/events",
    request_body = CreateEventRequest,
    responses((status = 201, body = CreateEventResponse)),
)]
/// Create a new event
async fn create_event(
    state: State<AppState>,
    body: Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<CreateEventResponse>), AppError> {
    let event_id = uuid::Uuid::new_v4();
    let slug = generate_slug(&body.title.clone());
    let event = Event {
        id: event_id.to_string(),
        title: body.title.clone(),
        details: body.details.clone(),
        slug: slug.clone(),
        maximum_attendees: body.maximum_attendees.clone(),
    };

    let conn = state.pool.get().await.unwrap();
    match conn
        .interact(move |conn| {
            events::table
                .filter(events::slug.eq(slug))
                .first::<Event>(conn)
                .optional()
        })
        .await
    {
        Ok(Ok(Some(_))) => {
            return Err(AppError::BadRequest(
                "Another event with same title already exists".into(),
            ))
        }
        Ok(Ok(None)) => {}
        Ok(Err(err)) => return Err(AppError::from(err)),
        Err(err) => return Err(AppError::InternalServerError(err.to_string())),
    };

    match conn
        .interact(|conn| {
            diesel::insert_into(events::table)
                .values(event)
                .execute(conn)
        })
        .await
    {
        Ok(result) => match result {
            Ok(_) => {}
            Err(err) => return Err(AppError::from(err)),
        },
        Err(err) => return Err(AppError::BadRequest(err.to_string())),
    };

    Ok((StatusCode::CREATED, Json(CreateEventResponse { event_id })))
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetEventResponse {
    id: uuid::Uuid,
    title: String,
    details: Option<String>,
    slug: String,
    maximum_attendees: Option<i32>,
    attendees_amount: i64,
}

#[debug_handler]
#[utoipa::path(
    get,
    tag = "events",
    path = "/events/:event_id",
    params(("event_id" = Uuid, Path, description = "Event ID")),
    responses((status = 200, body = GetEventResponse)),
)]
/// Get an event
pub async fn get_event(
    state: State<AppState>,
    event_id: Path<Uuid>,
) -> Result<Json<GetEventResponse>, AppError> {
    let conn = state.pool.get().await.unwrap();
    let (event, attendee_amount) = match conn
        .interact(move |conn| {
            let event = events::table
                .find(event_id.to_string())
                .first::<Event>(conn);
            let attendees_ammount = attendees::table
                .filter(attendees::event_id.eq(event_id.to_string()))
                .count()
                .get_result::<i64>(conn);
            (event, attendees_ammount)
        })
        .await
    {
        Ok((result, attendee_amount)) => match (result, attendee_amount) {
            (Ok(event), Ok(amount)) => (event, amount),
            (Ok(event), _) => (event, 0),
            (Err(err), _) => return Err(AppError::from(err)),
        },
        Err(err) => return Err(AppError::InternalServerError(err.to_string())),
    };

    Ok(Json(GetEventResponse {
        id: Uuid::parse_str(&event.id).unwrap(),
        title: event.title,
        details: event.details,
        slug: event.slug,
        maximum_attendees: event.maximum_attendees,
        attendees_amount: attendee_amount,
    }))
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterForEventRequest {
    #[validate(length(min = 4))]
    name: String,
    #[validate(email)]
    email: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterForEventResponse {
    attendee_id: i32,
}

#[debug_handler]
#[utoipa::path(
    post,
    tag = "attendees",
    path = "/events/:event_id/attendees",
    params(("event_id" = Uuid, Path, description = "Event ID")),
    request_body = RegisterForEventRequest,
    responses((status = 201, body = RegisterForEventResponse)),
)]
/// Register an attendee
pub async fn register_for_event(
    state: State<AppState>,
    event_id: Path<Uuid>,
    body: Json<RegisterForEventRequest>,
) -> Result<(StatusCode, Json<RegisterForEventResponse>), AppError> {
    let conn = state.pool.get().await.unwrap();

    let email = body.email.clone();
    let event_id = event_id.to_string().clone();
    let event_id1 = event_id.clone();
    let event_id2 = event_id.clone();

    let (event, amount_of_attendees_for_event): (Event, i32) = match conn
        .interact(move |conn| {
            let event = events::table
                .find(event_id.clone())
                .first::<Event>(conn)
                .optional();
            let attendee_amount = attendees::table
                .filter(attendees::event_id.eq(event_id))
                .count()
                .get_result::<i64>(conn);

            (event, attendee_amount)
        })
        .await
    {
        Ok((Ok(Some(event)), Ok(ammount))) => (event, ammount as i32),
        Ok((Ok(Some(event)), Err(_))) => (event, 0),
        Ok((Ok(None), _)) => {
            return Err(AppError::InternalServerError(
                "Event not found.".to_string(),
            ))
        }
        Ok((Err(err), _)) => return Err(AppError::InternalServerError(err.to_string())),
        Err(err) => return Err(AppError::InternalServerError(err.to_string())),
    };

    match conn
        .interact(move |conn| {
            attendees::table
                .filter(attendees::event_id.eq(event_id1.to_string()))
                .filter(attendees::email.eq(email))
                .first::<Attendee>(conn)
                .optional()
        })
        .await
    {
        Ok(Ok(Some(_))) => {
            return Err(AppError::InternalServerError(
                "This e-mail is already registered for this event.".to_string(),
            ))
        }
        Ok(_) => {}
        Err(err) => return Err(AppError::InternalServerError(err.to_string())),
    };

    if event.maximum_attendees.is_some()
        && amount_of_attendees_for_event >= event.maximum_attendees.unwrap()
    {
        return Err(AppError::InternalServerError("Event is full.".to_string()));
    }

    let email = body.email.clone();
    let name = body.name.clone();

    let attendee_id = match conn
        .interact(move |conn| {
            diesel::insert_into(attendees::table)
                .values((
                    attendees::event_id.eq(event_id2),
                    attendees::name.eq(name),
                    attendees::email.eq(email),
                ))
                .returning(attendees::id)
                .get_result(conn)
        })
        .await
    {
        Ok(result) => match result {
            Ok(attendee_id) => attendee_id,
            Err(err) => return Err(AppError::from(err)),
        },
        Err(err) => return Err(AppError::InternalServerError(err.to_string())),
    };

    return Ok((
        StatusCode::CREATED,
        Json(RegisterForEventResponse { attendee_id }),
    ));
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetAttendeesResponse {
    attendees: Vec<AttendeeWithCheckIn>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct GetAttendeesQuery {
    query: Option<String>,
    #[validate(range(min = 0))]
    page_index: Option<i64>,
}

#[debug_handler]
#[utoipa::path(
    get,
    tag = "events",
    path = "/events/:event_id/attendees",
    params(("event_id" = Uuid, Path, description = "Event ID"), ("query" = GetAttendeesQuery, Query, description = "Query")),
    responses((status = 200, body = GetAttendeesResponse)),
)]
/// Get event attendees
pub async fn get_attendees(
    state: State<AppState>,
    event_id: Path<Uuid>,
    query: Query<GetAttendeesQuery>,
) -> Result<Json<GetAttendeesResponse>, AppError> {
    let conn = state.pool.get().await.unwrap();

    let name = query.query.clone();
    let page = query.page_index.unwrap_or(0);

    const PAGE_SIZE: i64 = 10;

    let attendees = match conn
        .interact(move |conn| {
            attendees::table
                .filter(attendees::event_id.eq(event_id.to_string()))
                .left_join(check_ins::table)
                .select((
                    attendees::id,
                    attendees::name,
                    attendees::email,
                    attendees::created_at,
                    check_ins::created_at.nullable(),
                ))
                .limit(PAGE_SIZE)
                .offset(page * PAGE_SIZE)
                .load::<AttendeeWithCheckIn>(conn)
        })
        .await
    {
        Ok(result) => match result {
            Ok(attendees) => attendees,
            Err(err) => return Err(AppError::from(err)),
        },
        Err(err) => return Err(AppError::InternalServerError(err.to_string())),
    };

    if let Some(name) = name {
        let attendees: Vec<_> = attendees
            .iter()
            .filter(|attendee| attendee.name.contains(&name))
            .cloned()
            .collect();

        return Ok(Json(GetAttendeesResponse { attendees }));
    }
    Ok(Json(GetAttendeesResponse { attendees }))
}
