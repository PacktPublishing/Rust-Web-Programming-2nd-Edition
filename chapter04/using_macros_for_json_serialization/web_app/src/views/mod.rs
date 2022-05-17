mod auth;
mod to_do;

use auth::auth_views_factory;
use to_do::to_do_views_factory;

use actix_web::web::ServiceConfig;


/// Connects the server to the views being created. 
/// 
/// # Arguments 
/// * app: (&mut ServiceConfig) enabling the application configuration to be bound to the view factories
pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_views_factory(app);
}