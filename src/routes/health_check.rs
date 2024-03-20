use actix_web::HttpResponse;

pub async fn health_check() -> actix_web::HttpResponse {
    return HttpResponse::Ok().finish();
}
