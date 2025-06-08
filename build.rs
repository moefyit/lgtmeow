use indicatif::{ProgressBar, ProgressStyle};
use reqwest::ClientBuilder;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::LazyLock;
use std::time::{Duration, SystemTime};
use std::{fs::File, io::Write};
use tokio_stream::StreamExt;

static APP_NAME: &str = "lgtmeow";
static METADATA_SAVE_FILE_NAME: &str = "emojikitchen.json";
static PARTIAL_KITCHEIN_DATA_DIR: &str = "partial-kitchen-data";
static PERSISTABLE_CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::home_dir()
        .expect("Home directory not found. Ensure the environment has a valid home directory.")
        .join(format!(".cache/{}", APP_NAME))
});
static CACHE_ALIVE_TIME: Duration = Duration::from_secs(60 * 60 * 24); // 24 hours
#[cfg(feature = "emoji-paw-prints")]
static PAW_PRINTS_CODEPOINT: &str = "1f43e";
#[cfg(feature = "emoji-paw-prints")]
static PAW_PRINTS_FEATURE_NAME: &str = "paw-prints";
#[cfg(feature = "emoji-cat")]
static CAT_CODEPOINT: &str = "1f431";
#[cfg(feature = "emoji-cat")]
static CAT_FEATURE_NAME: &str = "cat";
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
        .use_rustls_tls()
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
            .next_back()
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

    let cache_is_valid = cache_path.as_ref().exists()
        && cache_path
            .as_ref()
            .metadata()
            .expect("Failed to access file metadata")
            .modified()
            .expect("Failed to retrieve file modification time")
            + CACHE_ALIVE_TIME
            > SystemTime::now();

    if !cache_is_valid {
        eprintln!("Metadata not found or expired, downloading...");
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

pub fn ensure_dir(dir: &Path) -> Result<(), std::io::Error> {
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let use_persistable_cache = env::var("USE_PERSISTABLE_CACHE")
        .map(|v| ["true", "1", "on", "yes"].contains(&v.as_str()))
        .unwrap_or(false);
    let metadata_save_path: PathBuf = if use_persistable_cache {
        let cache_dir = PERSISTABLE_CACHE_DIR.as_ref();
        ensure_dir(cache_dir).expect("Could not create cache directory");
        (**PERSISTABLE_CACHE_DIR).join(METADATA_SAVE_FILE_NAME)
    } else {
        PathBuf::from(&out_dir).join(METADATA_SAVE_FILE_NAME)
    };
    let kitchen_partial_data_dir = Path::new(&out_dir).join(PARTIAL_KITCHEIN_DATA_DIR);
    ensure_dir(&kitchen_partial_data_dir).expect("Could not create partial data directory");
    let metadata = get_metadata_with_progressbar(metadata_save_path)
        .await
        .unwrap();
    #[cfg(feature = "emoji-paw-prints")]
    {
        let paw_prints_partial_data_path =
            kitchen_partial_data_dir.join(format!("{}.json", PAW_PRINTS_FEATURE_NAME));
        let paw_prints_kitchen_data = metadata
            .data
            .get(PAW_PRINTS_CODEPOINT)
            .expect("Could not get paw prints data");
        let paw_prints_kitchen_string = serde_json::to_string(paw_prints_kitchen_data)
            .expect("Could not serialize paw prints data");
        std::fs::write(paw_prints_partial_data_path, paw_prints_kitchen_string)
            .expect("Could not write paw prints data");
    }
    #[cfg(feature = "emoji-cat")]
    {
        let cat_partial_data_path =
            kitchen_partial_data_dir.join(format!("{}.json", CAT_FEATURE_NAME));
        let cat_kitchen_data = metadata
            .data
            .get(CAT_CODEPOINT)
            .expect("Could not get cat data");
        let cat_kitchen_string =
            serde_json::to_string(cat_kitchen_data).expect("Could not serialize cat data");
        std::fs::write(cat_partial_data_path, cat_kitchen_string)
            .expect("Could not write cat data");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
