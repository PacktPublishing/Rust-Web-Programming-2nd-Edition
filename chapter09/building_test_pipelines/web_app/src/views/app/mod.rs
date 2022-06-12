use actix_web::web;
mod items;
mod content_loader;


pub fn app_views_factory(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(items::items));
}
