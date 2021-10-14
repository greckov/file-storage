use serde::Serialize;
use aws_sdk_s3::{ByteStream, Client, SdkError};
use aws_sdk_s3::error::{DeleteObjectError, ListObjectsV2Error, PutObjectError};
use aws_sdk_s3::model::ObjectCannedAcl;
use aws_sdk_s3::output::{DeleteObjectOutput, PutObjectOutput};
use crate::{BUCKET_NAME, BUCKET_URL};

pub async fn initialize_s3_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

#[derive(Serialize)]
pub struct FileListItem {
    pub key: String,
    pub url: String,
}

pub async fn fetch_file_list(client: &Client) -> Result<
    Vec<FileListItem>,
    SdkError<ListObjectsV2Error>
> {
    let resp = client
        .list_objects_v2()
        .bucket(BUCKET_NAME)
        .send()
        .await?;

    let file_list = resp.contents
        .unwrap_or_default()
        .into_iter()
        .map(move |object| {
            let object_key = object.key.expect("Empty object key is not supported");

            FileListItem {
                url: String::from(format!("{}{}", BUCKET_URL, &object_key)),
                key: object_key,
            }
        })
        .collect();

    Ok(file_list)
}

pub async fn drop_file_from_s3(
    client: &Client,
    key: String
) -> Result<DeleteObjectOutput, SdkError<DeleteObjectError>> {
    let filename = html_escape::decode_html_entities(&key);

    client
        .delete_object()
        .bucket(BUCKET_NAME)
        .key(filename)
        .send()
        .await
}

pub async fn put_file_to_s3(
    client: &Client,
    name: &str,
    stream: ByteStream
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {

    client
        .put_object()
        .set_acl(Some(ObjectCannedAcl::PublicRead))
        .bucket(BUCKET_NAME)
        .key(name)
        .body(stream)
        .send()
        .await
}
