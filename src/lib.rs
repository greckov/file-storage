use rocket::{Build, Rocket, routes};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

mod views;
mod storage;

pub fn runserver() -> Rocket<Build> {
      rocket::build()
          .attach(Template::fairing())
          .mount("/", routes![
                views::index
          ])
          .mount("/static", FileServer::from(relative!("static")))
}
