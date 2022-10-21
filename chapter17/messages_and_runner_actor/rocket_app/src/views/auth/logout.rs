use rocket::response::content::RawHtml;


#[get("/logout")]
pub async fn logout() -> RawHtml<&'static str> {
        return RawHtml("<html>\
                <script>\
                    localStorage.removeItem('user-token'); \
                    window.location.replace(
                        document.location.origin);\
                </script>\
              </html>")
}
