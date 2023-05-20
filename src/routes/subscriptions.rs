use actix_web::{web, HttpResponse, post};

use crate::dtos::subscriptions::SubscriptionsFormData;


#[post("/subscriptions")]
pub async fn subscriptions(form: web::Form<SubscriptionsFormData>) -> HttpResponse {
    println!("{}, {}", form.email, form.name);
    HttpResponse::Ok().finish()
}
