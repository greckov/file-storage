use std::error::Error;
use serde::Serialize;
use aws_sdk_s3::Client;

pub async fn initialize_s3_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

#[derive(Serialize)]
pub struct FileListItem {
    pub key: String,
    pub url: String,
    pub uploaded_at: String,
}

pub async fn fetch_file_list(client: &Client) -> Result<Vec<FileListItem>, Box<dyn Error>> {
    let resp = client
        .list_objects_v2()
        .bucket("nure-cloud-task")
        .send()
        .await?;

    let file_list = resp.contents
        .unwrap_or_default()
        .into_iter()
        .map(move |object| {
            FileListItem {
                key: object.key.expect("Empty object key is not supported"),
                url: String::from("#"),
                uploaded_at: String::from("stub")
            }
        })
        .collect();

    Ok(file_list)
}
