use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

use crate::dtos::subscriptions::SubscriptionsFormData;

#[post("/subscriptions")]
pub async fn subscriptions(
    form: web::Form<SubscriptionsFormData>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4().to_string();
    let request_span = tracing::info_span!(
        "Adding a new subscriber...",
        %request_id,
        email = %form.email,
        name = %form.name
    );
    let _span = request_span.enter();
    let query_span = tracing::info_span!("Saving subscriber...");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    .execute(connection_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("New subscriber with name {} added", form.name);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Could not create new subscriber: {:#?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
