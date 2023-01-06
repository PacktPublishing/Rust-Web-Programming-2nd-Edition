mod create;
mod get;
mod edit;

use actix_web::web::{ServiceConfig, post, scope, get};


pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
            .route("get", get().to(get::get))
            .route("edit", post().to(edit::edit))
    );
}
