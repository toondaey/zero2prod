use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    database::subscriptions::insert_subscriber, dtos::subscriptions::SubscriptionsFormData,
};

#[post("/subscriptions")]
#[tracing::instrument(
    name = "Adding a new subscriber...",
    skip(form, connection_pool),
    fields(request_id = %Uuid::new_v4(), email = %form.email, name = %form.name)
)]
pub async fn subscriptions(
    form: web::Form<SubscriptionsFormData>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_subscriber(&form, &connection_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
