use std::{io::Write, time::Duration};
use std::io::Read;
use crate::error::Error;
use flate2::read::{DeflateDecoder, GzDecoder};
use reqwest::{header::{HeaderMap, ACCEPT_ENCODING, HOST, USER_AGENT}, Client};
use stream::{Feed, FeedReader, Stream};
use thiserror::Error;
use tokio::time;

mod stream;
mod error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    let url = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&type=10-k&company=&dateb=&owner=include&start=0&count=40&output=atom";

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Joordih Development jj@joordih.dev (Jordi Xavier)".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    headers.insert(HOST, "www.sec.gov".parse().unwrap());

    let client = Client::builder()
        .pool_max_idle_per_host(0)
        .timeout(Duration::from_secs(4))
        .build()?;
    
    let response = client.get(url)
        .headers(headers.clone())
        .send()
        .await
        .map_err(MyError::from)?;

    let response_url = response.url().clone();
    let content_encoding = response.headers().get(reqwest::header::CONTENT_ENCODING).cloned();
    let interval = Duration::from_secs_f64(1.0 / (5.0 + (3.0 * rand::random::<f64>())));

    let body = response.bytes().await.map_err(MyError::from)?;
    let stream = Stream::new(response_url, interval);

    let decompressed_data = body_decoder(body, content_encoding).await?;

    let parsed_feed = stream.parse_feed(&String::from_utf8_lossy(&decompressed_data));

    let entries: Vec<&dyn FeedReader> = match &parsed_feed {
        Ok(Feed::RSS(feed)) => feed.items().iter().map(|i| i as &dyn FeedReader).collect(),
        Ok(Feed::ATOM(feed)) => feed.entries().iter().map(|i| i as &dyn FeedReader).collect(),
        Err(_) => vec![],
    };

    time::sleep(Duration::from_secs(2)).await;

    let entry = entries.get(0).unwrap();

    let parts = entry.link().unwrap().split("index");
    let collection = parts.collect::<Vec<&str>>();
    let filing_url = format!("{}index-headers{}", collection.get(0).unwrap(), collection.get(1).unwrap());

    let body = fetch_filing(client, &filing_url, headers).await?;

    let mut file = std::fs::File::create("body.txt")?;
    file.write_all(String::from_utf8_lossy(&body).as_bytes())?;

    Ok(())
}

async fn fetch_filing(client: reqwest::Client, url: &str, headers: HeaderMap) -> Result<Vec<u8>, MyError> {
    let body_res = client.get(url)
        .headers(headers)
        .send()
        .await
        .map_err(MyError::from)?;

        let content_encoding = body_res.headers().get(reqwest::header::CONTENT_ENCODING).cloned();

    let body = body_res.bytes().await.map_err(MyError::from)?;
    let decompressed_data = body_decoder(body, content_encoding).await?;

    Ok(decompressed_data)
}


async fn body_decoder(body: bytes::Bytes, content_encoding: Option<reqwest::header::HeaderValue>) -> Result<Vec<u8>, MyError> {
    let mut decompressed_data = Vec::new();

    let encoding = content_encoding.and_then(|v| v.to_str().ok().map(|s| s.to_string()));

    match encoding.as_deref() {
        Some("gzip") => {
            let mut decoder = GzDecoder::new(&body[..]);
            decoder.read_to_end(&mut decompressed_data).map_err(MyError::from)?;
        },
        Some("deflate") => {
            let mut decoder = DeflateDecoder::new(&body[..]);
            decoder.read_to_end(&mut decompressed_data).map_err(MyError::from)?;
        },
        _ => {
            decompressed_data = body.to_vec();
        }
    }

    Ok(decompressed_data)
}