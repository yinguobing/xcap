use log::error;
use minio::s3::{
    args::{BucketExistsArgs, ObjectConditionalReadArgs},
    client::{Client, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
};
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("S3 error: {0}")]
    S3Error(#[from] minio::s3::error::Error),
    #[error("Reqwest error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Disk IO error: {0}")]
    DiskError(#[from] std::io::Error),
    #[error("Not existed: {0}")]
    NotExisted(String),
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug)]
pub struct Agent {
    client: Client,
}

impl Agent {
    pub fn new(
        base_url: &str,
        region: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Result<Self, Error> {
        let mut base_url = base_url.parse::<BaseUrl>()?;
        base_url.region = region.to_string();
        let static_provider = StaticProvider::new(access_key, secret_key, None);
        let client = ClientBuilder::new(base_url)
            .provider(Some(Box::new(static_provider)))
            .build()?;
        Ok(Self { client })
    }

    pub async fn download_object(
        &self,
        bucket: &str,
        object: &str,
        local_path: &PathBuf,
    ) -> Result<(), Error> {
        // Check bucket exist or not.
        let exists: bool = self
            .client
            .bucket_exists(&BucketExistsArgs::new(&bucket)?)
            .await?;
        if !exists {
            error!("Bucket {} does not exist.", bucket);
            return Err(Error::NotExisted(bucket.to_string()));
        }

        // Download object
        let obj_dscp = ObjectConditionalReadArgs::new(bucket, object)?;
        let response = self.client.get_object(&obj_dscp).await?;
        if response.status().is_success() {
            fs::write(
                local_path,
                response.bytes().await.map_err(|e| Error::RequestError(e))?,
            )?;
        }
        Ok(())
    }
}
