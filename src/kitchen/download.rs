use super::super::storage::{ensure_dir, CACHE_DIR};
use super::metadata;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::ClientBuilder;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::{fs::File, io::Write};
use tokio_stream::StreamExt;

static KITCHEN_METADATA_URL: &str =
    "https://raw.githubusercontent.com/xsalazar/emoji-kitchen-backend/main/app/metadata.json";

async fn get_file_size(url: reqwest::Url) -> Result<u64, reqwest::Error> {
    let client = ClientBuilder::new()
        .build()
        .expect("Could not build client");
    let response = client.get(url).header("Range", "bytes=0-1").send().await?;
    let size = match response.status() {
        reqwest::StatusCode::PARTIAL_CONTENT => response
            .headers()
            .get("Content-Range")
            .expect("Could not get content range")
            .to_str()
            .expect("Could not convert to string")
            .split('/')
            .last()
            .expect("Could not get last")
            .parse()
            .expect("Could not parse"),
        _ => 0,
    };
    Ok(size)
}

pub async fn download_file<S, FB, FD, FA>(
    url: reqwest::Url,
    path: PathBuf,
    before_download_hook: FB,
    on_download_hook: FD,
    after_download_hook: FA,
) -> Result<(), reqwest::Error>
where
    FB: FnOnce(&reqwest::Url, &PathBuf) -> Pin<Box<dyn Future<Output = S>>>,
    FD: Fn(S, &[u8]) -> Pin<Box<dyn Future<Output = S>>>,
    FA: FnOnce(S) -> Pin<Box<dyn Future<Output = ()>>>,
{
    if let Some(p) = path.parent() {
        ensure_dir(p).expect("Could not create cache directory");
    }
    let mut hook_state = before_download_hook(&url, &path).await;
    let mut stream = reqwest::get(url).await?.bytes_stream();
    let mut file = File::create(&path).expect("Could not create cache file");

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        hook_state = on_download_hook(hook_state, &chunk).await;
        file.write_all(&chunk)
            .expect("Could not write to cache file");
    }
    after_download_hook(hook_state).await;
    Ok(())
}

pub fn dummy_before_download_hook(
    _: &reqwest::Url,
    _: &PathBuf,
) -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async {})
}
pub fn dummy_on_download_hook(_: (), _chunk: &[u8]) -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async {})
}
pub fn dummy_after_download_hook(_: ()) -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async {})
}

struct DownloadState {
    total_size: u64,
    downloaded: usize,
    progress_bar: Box<ProgressBar>,
}

#[allow(dead_code)]
pub async fn get_metadata_with_progressbar() -> Result<metadata::KitchenMetaData, reqwest::Error> {
    let cache_file = CACHE_DIR.join("emojikitchen.json");

    let before_download_hook = |url: &reqwest::Url,
                                _path: &PathBuf|
     -> Pin<Box<dyn Future<Output = DownloadState>>> {
        let url = url.clone();
        Box::pin(async {
            let total_size = get_file_size(url).await.unwrap();
            let pb = Box::new(ProgressBar::new(total_size));
            pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.magenta/238}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                    .unwrap()
                    .progress_chars("━╸━"));
            DownloadState {
                total_size,
                downloaded: 0,
                progress_bar: pb,
            }
        })
    };

    let on_download_hook =
        |state: DownloadState, chunk: &[u8]| -> Pin<Box<dyn Future<Output = DownloadState>>> {
            let chunk_size = chunk.len();
            Box::pin(async move {
                let downloaded = state.downloaded + chunk_size;
                state.progress_bar.set_position(downloaded as u64);
                DownloadState {
                    total_size: state.total_size,
                    downloaded,
                    progress_bar: state.progress_bar,
                }
            })
        };

    let after_download_hook = |state: DownloadState| -> Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(async move {
            state.progress_bar.finish_with_message("done!");
        })
    };

    if !std::path::Path::new(&cache_file).exists() {
        eprintln!("Metadata not found, downloading...");
        download_file(
            reqwest::Url::try_from(KITCHEN_METADATA_URL).expect("Could not parse URL"),
            cache_file.clone(),
            before_download_hook,
            on_download_hook,
            after_download_hook,
        )
        .await?;
    }
    let response = std::fs::read_to_string(&cache_file).unwrap();
    let metadata: metadata::KitchenMetaData = serde_json::from_str(&response).unwrap();
    Ok(metadata)
}

pub async fn get_metadata() -> Result<metadata::KitchenMetaData, reqwest::Error> {
    let cache_file = CACHE_DIR.join("emojikitchen.json");

    if !std::path::Path::new(&cache_file).exists() {
        // eprintln!("Metadata not found, downloading...");
        download_file(
            reqwest::Url::try_from(KITCHEN_METADATA_URL).expect("Could not parse URL"),
            cache_file.clone(),
            dummy_before_download_hook,
            dummy_on_download_hook,
            dummy_after_download_hook,
        )
        .await?;
    }
    let response = std::fs::read_to_string(&cache_file).unwrap();
    let metadata: metadata::KitchenMetaData = serde_json::from_str(&response).unwrap();
    Ok(metadata)
}
