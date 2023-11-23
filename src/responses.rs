use actix_web::http::header;
use actix_web::HttpResponse;

pub fn response_text(content: String) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/plain; charset=utf-8"))
        .body(content)
}

pub fn response_internal_server_error(content: String) -> HttpResponse {
    HttpResponse::InternalServerError()
        .insert_header((header::CONTENT_TYPE, "text/plain; charset=utf-8"))
        .body(content)
}
