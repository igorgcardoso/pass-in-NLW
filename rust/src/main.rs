use axum::Router;
use deadpool_diesel::sqlite::{Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use models::AttendeeWithCheckIn;
use routes::{
    attendees::{AttendeeBadgeResponse, Badge, __path_check_in, __path_get_attendee_badge},
    events::{
        CreateEventRequest, CreateEventResponse, GetAttendeesQuery, GetAttendeesResponse,
        GetEventResponse, RegisterForEventRequest, RegisterForEventResponse, __path_create_event,
        __path_get_attendees, __path_get_event, __path_register_for_event,
    },
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

mod error;
mod models;
mod routes;
mod schema;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(
            #[cfg(debug_assertions)]
            tracing::Level::DEBUG,
            #[cfg(not(debug_assertions))]
            tracing::Level::INFO,
        )
        .init();

    #[derive(OpenApi)]
    #[openapi(
        paths(
            create_event,
            get_event,
            register_for_event,
            get_attendees,
            get_attendee_badge,
            check_in
        ),
        components(schemas(
            CreateEventRequest,
            CreateEventResponse,
            GetEventResponse,
            RegisterForEventRequest,
            RegisterForEventResponse,
            GetAttendeesQuery,
            GetAttendeesResponse,
            AttendeeBadgeResponse,
            Badge,
            AttendeeWithCheckIn,
        ))
    )]
    struct ApiDoc;

    let doc = { ApiDoc::openapi() };

    let db_url = std::env::var("DATABASE_URL")?;
    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build()?;

    run_migrations(&pool).await;

    let state = AppState { pool };

    let app = Router::new()
        .nest("/", routes::events::router())
        .nest("/", routes::attendees::router())
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", doc))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3333));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}
