use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    database::subscriptions::insert_subscriber,
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
    dtos::subscriptions::SubscriptionsFormData,
};

#[post("/subscriptions")]
#[tracing::instrument(
    name = "Adding a new subscriber...",
    skip(form, connection_pool),
    fields(email = %form.email, name = %form.name)
)]
pub async fn subscriptions(
    form: web::Form<SubscriptionsFormData>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_subscriber = NewSubscriber {
        email: match SubscriberEmail::parse(form.0.email) {
            Ok(email) => email,
            Err(_) => return HttpResponse::BadRequest().finish()
        },
        name: match SubscriberName::parse(form.0.name) {
            Ok(name) => name,
            Err(_) => return HttpResponse::BadRequest().finish(),
        },
    };
    match insert_subscriber(&new_subscriber, &connection_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
