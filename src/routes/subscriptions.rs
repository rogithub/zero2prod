use actix_web::{ web, HttpResponse };
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc; 
use unicode_segmentation::UnicodeSegmentation;
use crate::domain::{ NewSubscriber, SubscriberName, SubscriberEmail };

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String
}

pub fn parse_subscribe(form: FormData) -> Result<NewSubscriber, String> {
    let name = SubscriberName::parse(form.name)?;
    let email = SubscriberEmail::parse(form.email)?;

    Ok(NewSubscriber { email, name })
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(form: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(form.name)?;
        let email = SubscriberEmail::parse(form.email)?;

        Ok(NewSubscriber { email, name })
    }
    
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
        form: web::Form<FormData>, 
        pool: web::Data<PgPool>
    ) -> HttpResponse {
    
    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    match insert_subscriber(&pool, &new_subscriber)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

/// Returns true if the input satisfies all our validation constraints
/// on subscriber names, false otherwise.
pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();

    // A grapheme is defined by the unicode standard as "user-percived"
    // character: Ä is a single grapheme, but it is composed of two characters.
    //
    // graphemes returns an iterator over the graphemes in the input s.
    // true specifies that we want to use the extended grapheme definition set,
    // the recommended one.
    let is_too_long = s.graphemes(true).count() > 255;

    // Iterate over all chars in the input s to check if any of them matches
    // one of the characters in the forbidden array.
    let forbidden_characters = ['/','(', ')', '"', '<', '>', '\\', '{', '}' ];
    let contains_forbidden_characters = s
        .chars()
        .any(|g| forbidden_characters.contains(&g));

    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)    
)]
pub async fn insert_subscriber(
    pool: &PgPool, 
    new_subscriber: &NewSubscriber
) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
            Uuid::new_v4(),
            new_subscriber.email.as_ref(),
            new_subscriber.name.as_ref(),
            Utc::now())
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    
    Ok(())
}