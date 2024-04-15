//! src/routes/subscribe.rs

use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // `Result` has two variants: `Ok` and `Err`.
    // The first for successes, the second for the failures.
    // We use a `match` statement to choose what to do based
    // on the outcome.

    let request_id = Uuid::new_v4();

    //Spans, like logs, have an associated level
    // `info_span` creates a span at the info_level
    let request_span = tracing::info_span!(
        "Adding as a new subscriber.",
        %request_id,
        subscriber_email = %form.name,
        subscriber_name = %form.email
    );

    //Using `enter` in an async function is a recipe for disaster!
    //Bear with me for now, but dont do this at home.
    //See the following section on `Instrumentation Futures`

    let _request_span_guard = request_span.enter();
    //`_request_span_guard` is the dropped at the end of `subscribe`
    // Thats when we "exit" the span

    let query_span= tracing::info_span!("Saving new subscriber details in the database!");

    match sqlx::query!(
        r#"
    INSERT INTO subscription (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    //We use `get_ref` to get an immutable reference to the `PgPool`
    //  wrapped by `web::Data`.
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("{request_id}: New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("{request_id}: Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
