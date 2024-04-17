//! src/routes/subscribe.rs

use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if !is_valid_name(&form.name) {
        return HttpResponse::Ok().finish();
    }
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns `true` if the input satisfies all our validation constraints
/// on subscriber names, `false` otherwise
fn is_valid_name(name: &str) -> bool {
    // `.trim()` returns a view over the input `s` without trailing
    // whitespace-like characters
    // `.is_empty` checks if the view contains any character.
    let is_empty_or_whitespace = name.trim().is_empty();

    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character : `å` is a single grapheme, but is comprised of two characters
    // (`a` and ``̊`).
    //
    // `graphemes` returns an iterator over the graphemes in th input `is`.
    // `true` specifies that we want to use the extended grapheme definition set,
    // the recommended one.
    let is_too_long = name.graphemes(true).count() > 256;

    //Iterate over all characters in the input `name` to chcek if any of them matches
    //one of the characters in the forbidden array.
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = name
        .chars()
        .any(|g| forbidden_characters.contains(&g));

    // Return `false` if ony of our conditionns have been violated
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscription (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
