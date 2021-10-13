use file_server::{initialize_s3_client, runserver};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
      let s3_client = initialize_s3_client().await;
      runserver(s3_client).await
}

