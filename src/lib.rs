use aws_sdk_s3::Client;
use rocket::routes;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
pub use storage::initialize_s3_client;

mod views;
mod storage;
mod utils;

const BUCKET_URL: &str = "https://nure-cloud-task.s3.eu-central-1.amazonaws.com/";

pub async fn runserver(aws_client: Client) -> Result<(), rocket::Error> {
    let url_handlers = routes![
        views::index,
        views::upload_file,
        views::prune_file
    ];

    rocket::build()
        .attach(Template::fairing())
        .mount("/", url_handlers)
        .mount("/static", FileServer::from(relative!("static")))
        .manage(aws_client)
        .launch()
        .await
}
