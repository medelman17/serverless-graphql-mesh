pub mod s3 {
    pub use aws_sdk_s3::{Client as S3Client, Error as S3Error};
    use bytes::Bytes;

    pub async fn get_object(
        client: &S3Client,
        bucket: String,
        key: String,
    ) -> Result<Bytes, S3Error> {
        let output = client.get_object().bucket(bucket).key(key).send().await?;
        let data = output.body.collect().await.map(|d| d.into_bytes());
        match data {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(S3Error::Unhandled(Box::new(e))),
        }
    }
}
