use aws_sdk_s3::ByteStream;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

pub async fn tempfile_to_byte_stream(temp: &TempFile<'_>) -> ByteStream {
    match temp {
        TempFile::File { .. } => {
            // NOTE: unwraps below will not ever panic, because file always exist
            let path = temp.path().unwrap();
            ByteStream::from_path(path).await.unwrap()
        }
        TempFile::Buffered { content } => {
            ByteStream::from(content.as_bytes().to_vec())
        }
    }
}

