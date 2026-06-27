use futures_util::StreamExt;
use log::{error, info};
use minio::s3::{
    client::{Client, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
    types::{S3Api, ToStream},
};
use std::path::{Path, PathBuf};
use std::sync::{atomic::AtomicBool, Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("S3 error: {0}")]
    S3Error(Box<minio::s3::error::Error>),

    #[error("Disk IO error: {0}")]
    DiskError(#[from] std::io::Error),

    #[error("Not existed: {0}")]
    NotExisted(String),

    #[error("unknown error")]
    Unknown,
}

impl From<minio::s3::error::Error> for Error {
    fn from(e: minio::s3::error::Error) -> Self {
        Error::S3Error(Box::new(e))
    }
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

    pub async fn download_dir(
        &self,
        bucket: &str,
        dir: &str,
        local_path: &Path,
        sigint: &Arc<AtomicBool>,
    ) -> Result<(), Error> {
        // Check bucket exist or not.
        let exists: bool = self.client.bucket_exists(bucket).send().await?.exists;
        if !exists {
            error!("Bucket {} does not exist.", bucket);
            return Err(Error::NotExisted(bucket.to_string()));
        }

        // List objects
        let mut objects: Vec<String> = vec![];
        let mut resp = self
            .client
            .list_objects(bucket)
            .recursive(true)
            .use_api_v1(false) // use v2
            .include_versions(true)
            .to_stream()
            .await;
        while let Some(result) = resp.next().await {
            match result {
                Ok(resp) => {
                    for item in resp.contents {
                        objects.push(item.name);
                    }
                }

                Err(e) => {
                    error!("List object error.");
                    return Err(Error::S3Error(Box::new(e)));
                }
            }
        }

        // Filter objects
        let targets: Vec<&String> = objects.iter().filter(|&o| o.starts_with(dir)).collect();

        // Download objects
        for object in targets {
            if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            let obj_file = object.split('/').next_back().unwrap();
            let obj_file_path = local_path.join(obj_file);
            info!("Downloading: {}", obj_file);
            self.download_object(bucket, object, &obj_file_path).await?;
        }

        Ok(())
    }

    pub async fn download_object(
        &self,
        bucket: &str,
        object: &str,
        local_path: &PathBuf,
    ) -> Result<(), Error> {
        // Check bucket exist or not.
        let exists: bool = self.client.bucket_exists(bucket).send().await?.exists;
        if !exists {
            error!("Bucket {} does not exist.", bucket);
            return Err(Error::NotExisted(bucket.to_string()));
        }

        // Download object
        let get_object = self.client.get_object(bucket, object).send().await?;
        get_object.content.to_file(Path::new(local_path)).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = Error::NotExisted("bucket".to_string());
        assert!(format!("{}", err).contains("bucket"));

        let err = Error::Unknown;
        assert_eq!(format!("{}", err), "unknown error");

        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = Error::DiskError(io_err);
        assert!(format!("{}", err).contains("Disk IO"));
    }
}
