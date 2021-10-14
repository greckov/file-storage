use aws_sdk_s3::Client;
use rocket::{get, post, delete, State};
use rocket::fs::TempFile;
use rocket::serde::Serialize;
use rocket::form::{Form, FromForm};
use rocket_dyn_templates::Template;
use crate::storage::{drop_file_from_s3, fetch_file_list, FileListItem};


#[get("/")]
pub async fn index(s3_client: &State<Client>) -> Template {
    #[derive(Serialize)]
    struct IndexPageContext {
        files: Vec<FileListItem>
    }

    let files = fetch_file_list(s3_client)
        .await
        .unwrap_or(vec![]);

    Template::render("index", IndexPageContext { files })
}

#[delete("/prune-file/<key>")]
pub async fn prune_file(s3_client: &State<Client>, key: String) -> &'static str {
    let drop_result = drop_file_from_s3(s3_client, key).await;

    match drop_result {
        Ok(_) => "ok",
        Err(_) => "failed"
    }
}

#[derive(FromForm)]
pub struct UploadFileForm<'r> {
    pub file: TempFile<'r>
}

#[post("/upload-file", data="<_file>")]
pub async fn upload_file(
    _s3_client: &State<Client>,
    _file: Form<UploadFileForm<'_>>
) -> &'static str {
    "success"
}
