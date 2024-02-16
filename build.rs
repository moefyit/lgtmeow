use indicatif::{ProgressBar, ProgressStyle};
use reqwest::ClientBuilder;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::{fs::File, io::Write};
use tokio_stream::StreamExt;

static METADATA_CACHE_FILE_NAME: &str = "emojikitchen.json";
static PAW_PRINTS_CODEPOINT: &str = "1f43e";
static PAW_PRINTS_KITCHEN_METADATA_FILE_NAME: &str = "paw_prints_kitchen_data.json";
static KITCHEN_METADATA_URL: &str =
    "https://raw.githubusercontent.com/xsalazar/emoji-kitchen-backend/main/app/metadata.json";

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KitchenMetaData {
    pub known_supported_emoji: Vec<String>,
    pub data: HashMap<String, Value>,
}

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

pub async fn download_file<S, P, FB, FD, FA>(
    url: reqwest::Url,
    path: P,
    before_download_hook: FB,
    on_download_hook: FD,
    after_download_hook: FA,
) -> Result<(), reqwest::Error>
where
    P: AsRef<Path>,
    FB: FnOnce(&reqwest::Url, &PathBuf) -> Pin<Box<dyn Future<Output = S>>>,
    FD: Fn(S, &[u8]) -> Pin<Box<dyn Future<Output = S>>>,
    FA: FnOnce(S) -> Pin<Box<dyn Future<Output = ()>>>,
{
    let path = path.as_ref().to_path_buf();
    if let Some(p) = path.parent() {
        if !p.exists() {
            std::fs::create_dir_all(p).expect("Could not create cache directory");
        }
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

#[allow(dead_code)]
fn dummy_before_download_hook(_: &reqwest::Url, _: &PathBuf) -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async {})
}
#[allow(dead_code)]
fn dummy_on_download_hook(_: (), _chunk: &[u8]) -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async {})
}
#[allow(dead_code)]
fn dummy_after_download_hook(_: ()) -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async {})
}

struct DownloadState {
    total_size: u64,
    downloaded: usize,
    progress_bar: Box<ProgressBar>,
}

#[allow(dead_code)]
pub async fn get_metadata_with_progressbar<P>(
    cache_path: P,
) -> Result<KitchenMetaData, reqwest::Error>
where
    P: AsRef<Path>,
{
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

    if !cache_path.as_ref().exists() {
        eprintln!("Metadata not found, downloading...");
        download_file(
            reqwest::Url::try_from(KITCHEN_METADATA_URL).expect("Could not parse URL"),
            cache_path.as_ref(),
            before_download_hook,
            on_download_hook,
            after_download_hook,
        )
        .await?;
    }
    let response = std::fs::read_to_string(cache_path).unwrap();
    let metadata: KitchenMetaData = serde_json::from_str(&response).unwrap();
    Ok(metadata)
}

#[tokio::main]
async fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let paw_prints_kitchen_metadata_path =
        Path::new(&out_dir).join(PAW_PRINTS_KITCHEN_METADATA_FILE_NAME);
    let metadata_cache_path = Path::new(&out_dir).join(METADATA_CACHE_FILE_NAME);
    let metadata = get_metadata_with_progressbar(metadata_cache_path)
        .await
        .unwrap();
    let paw_prints_kitchen_data = metadata
        .data
        .get(PAW_PRINTS_CODEPOINT)
        .expect("Could not get paw prints data");
    let paw_prints_kitchen_string = serde_json::to_string(paw_prints_kitchen_data)
        .expect("Could not serialize paw prints data");
    std::fs::write(paw_prints_kitchen_metadata_path, paw_prints_kitchen_string)
        .expect("Could not write paw prints data");
    println!("cargo:rerun-if-changed=build.rs");
}
