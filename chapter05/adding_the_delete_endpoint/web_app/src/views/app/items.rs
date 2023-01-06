use actix_web::HttpResponse;
use super::content_loader::read_file;


pub async fn items() -> HttpResponse {
    let html_data = read_file(
        "./templates/main.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
