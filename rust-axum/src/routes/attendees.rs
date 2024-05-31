use crate::{
    error::AppError,
    models::{Attendee, CheckIn},
    schema::*,
    AppState,
};
use axum::{
    debug_handler,
    extract::{Json, Path, State},
    http::{HeaderMap, StatusCode},
    routing::get,
    Router,
};
use diesel::prelude::*;
use serde::Serialize;
use url::Url;
use utoipa::ToSchema;

pub fn router() -> Router<AppState> {
    let router = Router::new();
    let router = router
        .route("/attendees/:attendee_id/badge", get(get_attendee_badge))
        .route("/attendees/:attendee_id/check-in", get(check_in));

    router
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttendeeBadgeResponse {
    badge: Badge,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    name: String,
    email: String,
    event_title: String,
    check_in_url: String,
}

#[debug_handler]
#[utoipa::path(
    get,
    tag = "attendees",
    path = "/attendees/:attendee_id/badge",
    params(("attendee_id" = i32, Path, description = "Attendee ID")),
    responses((status = 200, body = AttendeeBadgeResponse))
)]
/// Get an attendee's badge
async fn get_attendee_badge(
    state: State<AppState>,
    attendee_id: Path<i32>,
    headers: HeaderMap,
) -> Result<Json<AttendeeBadgeResponse>, AppError> {
    let conn = state.pool.get().await.unwrap();
    let attendee = match conn
        .interact(move |conn| {
            attendees::table
                .find(*attendee_id)
                .first::<Attendee>(conn)
                .optional()
        })
        .await
    {
        Ok(Ok(Some(attendee))) => attendee,
        Ok(Ok(None)) => return Err(AppError::NotFound("Attendee not found".into())),
        Ok(Err(e)) => return Err(e.into()),
        Err(e) => return Err(AppError::InternalServerError(e.to_string())),
    };
    let event_title: String = match conn
        .interact(move |conn| {
            events::table
                .find(&attendee.event_id)
                .select(events::title)
                .first::<String>(conn)
        })
        .await
    {
        Ok(Ok(event_title)) => event_title,
        Ok(Err(e)) => return Err(e.into()),
        Err(e) => return Err(AppError::InternalServerError(e.to_string())),
    };

    let base_url = Url::parse(&*format!(
        "{}://{}",
        headers
            .get("x-forwarded-proto")
            .map(|v| v.to_str().unwrap_or("http"))
            .unwrap_or("http"),
        headers
            .get("x-forwarded-host")
            .map(|v| v.to_str().unwrap_or("localhost"))
            .unwrap_or("localhost")
    ))
    .unwrap();

    let check_in_url = format!(
        "{}://{}/attendees/{}/check-in",
        base_url.scheme(),
        base_url.host_str().unwrap_or("localhost"),
        attendee.id
    );

    Ok(Json(AttendeeBadgeResponse {
        badge: Badge {
            name: attendee.name,
            email: attendee.email,
            event_title,
            check_in_url,
        },
    }))
}

#[debug_handler]
#[utoipa::path(
    get,
    tag = "check-ins",
    path = "/attendees/:attendee_id/check-in",
    params(("attendee_id" = i32, Path, description = "Attendee ID")),
    responses((status = 204, body = ()))
)]
/// Check in an attendee
pub async fn check_in(
    state: State<AppState>,
    attendee_id: Path<i32>,
) -> Result<(StatusCode, Json<()>), AppError> {
    let conn = state.pool.get().await.unwrap();
    let attendee_id = *attendee_id;
    let attendee_id_clone = attendee_id.clone();
    match conn
        .interact(move |conn| {
            check_ins::table
                .filter(check_ins::attendee_id.eq(attendee_id))
                .first::<CheckIn>(conn)
                .optional()
        })
        .await
    {
        Ok(Ok(Some(_))) => {
            return Err(AppError::BadRequest("Attendee already checked in".into()));
        }
        Ok(Ok(None)) => {}
        Ok(Err(e)) => return Err(e.into()),
        Err(e) => return Err(AppError::InternalServerError(e.to_string())),
    };

    match conn
        .interact(move |conn| {
            diesel::insert_into(check_ins::table)
                .values(check_ins::attendee_id.eq(attendee_id_clone))
                .execute(conn)
        })
        .await
    {
        Ok(Ok(_)) => {}
        Ok(Err(e)) => return Err(e.into()),
        Err(e) => return Err(AppError::InternalServerError(e.to_string())),
    };

    Ok((StatusCode::NO_CONTENT, Json(())))
}
