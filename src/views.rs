use std::ops::Deref;
use aws_sdk_s3::{Client, SdkError};
use aws_sdk_s3::error::PutObjectError;
use aws_sdk_s3::output::PutObjectOutput;
use rocket::{get, post, delete, State, Response};
use rocket::fs::{FileName, TempFile};
use rocket::serde::Serialize;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;
use crate::BUCKET_URL;
use crate::storage::{drop_file_from_s3, fetch_file_list, FileListItem, put_file_to_s3};
use crate::utils::tempfile_to_byte_stream;


#[get("/")]
pub async fn index(s3_client: &State<Client>) -> Template {
    #[derive(Serialize)]
    struct IndexPageContext {
        files: Vec<FileListItem>,
        bucket_url: &'static str
    }

    let files = fetch_file_list(s3_client)
        .await
        .unwrap_or(vec![]);

    Template::render("index", IndexPageContext { files, bucket_url: BUCKET_URL })
}

#[delete("/prune-file/<key>")]
pub async fn prune_file(s3_client: &State<Client>, key: String) -> &'static str {
    let drop_result = drop_file_from_s3(s3_client, key).await;

    match drop_result {
        Ok(_) => "ok",
        Err(_) => "failed"
    }
}

#[post("/upload-file", format="multipart", data="<file>")]
pub async fn upload_file(
    s3_client: &State<Client>,
    file: Form<TempFile<'_>>
) -> Status {
    let stream = tempfile_to_byte_stream(file.deref()).await;

    // FIXME: Working with uploaded file is not very correct. Fix that,
    //  use solution without temp file
    let put_result = put_file_to_s3(
        s3_client,
        file
            .raw_name()
            .unwrap_or(&FileName::new("untitled"))
            .dangerous_unsafe_unsanitized_raw()
            .as_str(),
        stream
    ).await;

    match put_result {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError
    }
}
