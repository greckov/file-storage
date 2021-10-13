use aws_sdk_s3::Client;
use rocket::{get, State};
use rocket_dyn_templates::Template;
use serde::Serialize;
use crate::storage::{fetch_file_list, FileListItem};


#[derive(Serialize)]
struct IndexPageContext {
    files: Vec<FileListItem>
}


#[get("/")]
pub async fn index(s3_client: &State<Client>) -> Template {
    let files = fetch_file_list(s3_client)
        .await
        .unwrap_or(vec![]);

    Template::render("index", IndexPageContext { files })
}
