use actix_web::{web, App, HttpResponse, HttpServer};
// No longer importing PgConnection!
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing_futures::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name= %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );
    // `Result` has two variants: `Ok` and `Err`.
	// The first for successes, the second for failures.
	// We use a `match` statement to choose what to do based
	// on the outcome.
	// We will talk more about `Result` going forward!
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // `_request_span_guard` is dropped at the end of `subscribe`
    // That's when we "exit" the span
	// We use `get_ref` to get an immutable reference to the `PgConnection`
	// wrapped by `web::Data`.	// Using the pool as a drop-in replacement	
    .execute(pool.as_ref())
    // First we attach the instrumentation, then we `.await` it
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
