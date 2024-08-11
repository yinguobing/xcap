use log::info;
use minio::s3::{
    args::{BucketExistsArgs, MakeBucketArgs, ObjectConditionalReadArgs},
    client::ClientBuilder,
    creds::StaticProvider,
    http::BaseUrl,
};
use std::path::Path;

pub async fn download(
    base_url: &str,
    bucket: &str,
    path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let base_url = "https://play.min.io".parse::<BaseUrl>()?;

    info!("Trying to connect: `{:?}`", base_url);

    let static_provider = StaticProvider::new(
        "Q3AM3UQ867SPQQA43P2F",
        "zuf+tfteSlswRu7BJ86wekitnifILbZam1KYY3TG",
        None,
    );

    let client = ClientBuilder::new(base_url.clone())
        .provider(Some(Box::new(static_provider)))
        .build()?;

    let bucket_name: &str = "asiatrip";

    // Check 'bucket_name' bucket exist or not.
    let exists: bool = client
        .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
        .await
        .unwrap();

    // Make 'bucket_name' bucket if not exist.
    if !exists {
        client
            .make_bucket(&MakeBucketArgs::new(&bucket_name).unwrap())
            .await
            .unwrap();
    }

    // File we are going to upload to the bucket
    let filename: &Path = Path::new("/tmp/asiaphotos.zip");

    // Name of the object that will be stored in the bucket
    let object_name: &str = "asiaphotos-2015.zip";

    info!("filename {}", &filename.to_str().unwrap());

    let obj_dscp = ObjectConditionalReadArgs::new(bucket_name, object_name).unwrap();
    let result = client.get_object(&obj_dscp).await?;

    if result.status().is_success() {
        info!(
        "file `{}` is successfully downloaded as object `{object_name}` from bucket `{bucket_name}`.",
        filename.display()
    );
    } else {
        info!("file `{}` download failed.", filename.display());
    }
    Ok(())
}
