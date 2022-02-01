use super::svc::s3;
use apollo_router::configuration::{Configuration as RouterConfiguration, Configuration};
use apollo_router::ConfigurationKind;
use bytes::{Buf, Bytes};

#[derive(Debug)]
pub struct ConfigurationProvider {
    bytes: Option<bytes::Bytes>,
}

impl ConfigurationProvider {
    pub fn new() -> Self {
        Self { bytes: None }
    }

    pub async fn from_bucket(client: &s3::S3Client, bucket: String, key: String) -> Self {
        let bytes = s3::get_object(client, bucket, key).await;
        match bytes {
            Ok(bytes) => Self { bytes: Some(bytes) },
            Err(e) => Self { bytes: None },
        }
    }

    fn empty_config() -> RouterConfiguration {
        RouterConfiguration::builder().build()
    }
}

// impl From<ConfigurationProvider> for RouterConfiguration {
//     fn from(provider: ConfigurationProvider) -> Self {
//         match provider.bytes {
//             Some(bytes) => RouterConfiguration::builder().build().boxed(),
//             None => RouterConfiguration::builder().build().boxed(),
//         }
//     }
// }

impl Into<RouterConfiguration> for ConfigurationProvider {
    fn into(self) -> RouterConfiguration {
        match self.bytes {
            Some(bytes) => {
                let config: RouterConfiguration = serde_yaml::from_reader::<
                    bytes::buf::Reader<Bytes>,
                    RouterConfiguration,
                >(bytes.reader())
                .expect("");
                return config;
            }
            None => ConfigurationProvider::empty_config(),
        }
    }
}
