use super::svc::s3;
use apollo_router::SchemaKind;
use apollo_router_core::Schema;
use bytes::{Buf, Bytes};

#[derive(Debug)]
pub struct SchemaProvider {
    bytes: Option<bytes::Bytes>,
}

impl SchemaProvider {
    pub fn new() -> Self {
        Self { bytes: None }
    }

    pub async fn from_bucket(client: &s3::S3Client, bucket: String, key: String) -> Self {
        let bytes = s3::get_object(client, bucket, key).await.unwrap();
        Self {
            bytes: Option::Some(bytes),
        }
    }
}

impl Into<Schema> for SchemaProvider {
    fn into(self) -> Schema {
        match self.bytes {
            Some(bytes) => {
                let reader = std::str::from_utf8(bytes.as_ref()).unwrap();

                let schema: apollo_router_core::Schema = reader.parse().unwrap();
                return schema;
            }
            None => {
                panic!("No bytes")
            }
        }
    }
}
