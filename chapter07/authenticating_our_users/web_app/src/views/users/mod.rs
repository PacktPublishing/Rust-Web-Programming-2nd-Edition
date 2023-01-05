mod create;

use actix_web::web::{ServiceConfig, post, scope};


pub fn user_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/user")
            .route("create", post().to(create::create))
    );
}
