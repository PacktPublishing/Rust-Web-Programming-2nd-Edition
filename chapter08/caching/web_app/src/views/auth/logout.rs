use actix_web::HttpResponse;


pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<html>\
                <script>\
                    localStorage.removeItem('user-token'); \
                    localStorage.removeItem('item-cache-date'); \
                    localStorage.removeItem('item-cache-data-pending'); \
                    localStorage.removeItem('item-cache-data-done'); \
                    window.location.replace(
                        document.location.origin);\
                </script>\
              </html>")
}
