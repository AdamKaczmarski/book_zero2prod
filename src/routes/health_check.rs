use actix_web::HttpResponse;

pub async fn health_check() -> actix_web::HttpResponse {
    HttpResponse::Ok().finish()
}
