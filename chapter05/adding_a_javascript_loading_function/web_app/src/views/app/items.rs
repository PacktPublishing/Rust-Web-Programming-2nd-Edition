use actix_web::HttpResponse;
use super::content_loader::read_file;


pub async fn items() -> HttpResponse {
    let mut html_data = read_file("./templates/main.html");
    let javascript_data = read_file("./javascript/main.js");

    html_data = html_data.replace("{{JAVASCRIPT}}", &javascript_data);
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html_data)
}

