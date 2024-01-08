use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dtos::subscriptions::SubscriptionsFormData;

#[tracing::instrument(name = "Saving new subscriber...", skip(form, connection_pool))]
pub async fn insert_subscriber(
    form: &SubscriptionsFormData,
    connection_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    .execute(connection_pool)
    .await
    .map_err(|e| {
        tracing::error!("Could not create new subscriber: {:?}", e);
        e
    })?;
    Ok(())
}
