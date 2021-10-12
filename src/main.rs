use rocket::{Build, launch, Rocket};
use file_server::runserver;

#[launch]
fn rocket() -> Rocket<Build> {
      runserver()
}
