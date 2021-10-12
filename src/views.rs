use rocket::get;
use rocket_dyn_templates::Template;
use serde::Serialize;


#[derive(Serialize)]
struct EmptyContext {}


#[get("/")]
pub fn index() -> Template {
    Template::render("index", EmptyContext {})
}
