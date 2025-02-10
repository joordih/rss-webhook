use std::{any, time::Duration};
use rand::random;
use crate::error::Error;
use flate2::read::{DeflateDecoder, GzDecoder};
use reqwest::{header::{HeaderMap, ACCEPT_ENCODING, HOST, USER_AGENT}, Client};
use stream::Stream;
use thiserror::Error;

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

    let client = Client::new();
    
    let response = client.get(url)
        .headers(headers)
        .send()
        .await
        .map_err(MyError::from)?;

    let interval = Duration::from_secs_f64(1.0 / (5.0 + (3.0 * rand::random::<f64>())));
    let stream = Stream::new(response.url().clone(), interval);

    // stream.parse_feed(body)
    Ok(())
}

//  █▀ █ █▄ █ █ ▄▀▀ █▄█    ▀█▀ █▄█ █ ▄▀▀ 
//  █▀ █ █ ▀█ █ ▄█▀ █ █     █  █ █ █ ▄█▀ 

// async fn body_decoder(body: bytes::Bytes) {
//     let content_encoding = response.headers().get(reqwest::header::CONTENT_ENCODING).cloned();
//     let mut decompressed_data = Vec::new();

//     let encoding = content_encoding.and_then(|v| v.to_str().ok().map(|s| s.to_string()));

//     match encoding.as_deref() {
//         Some("gzip") => {
//             let mut decoder = GzDecoder::new(&body[..]);
//             decoder.read_to_end(&mut decompressed_data).map_err(MyError::from)?;
//         },
//         Some("deflate") => {
//             let mut decoder = DeflateDecoder::new(&body[..]);
//             decoder.read_to_end(&mut decompressed_data).map_err(MyError::from)?;
//         },
//         _ => {
//             decompressed_data = body.to_vec();
//         }
//     }
// }

// 
// ▄▀▄ █   █▀▄    ▄▀▀ ▄▀▄ █▀▄ ██▀    █▀ ▄▀▄ █▀▄    ▀█▀ ██▀ ▄▀▀ ▀█▀ █ █▄ █ ▄▀     █▀▄ ▄▀▄ █ █ █▀▄ █▀▄ ▄▀▄ ▄▀▀ ██▀ ▄▀▀ 
// ▀▄▀ █▄▄ █▄▀    ▀▄▄ ▀▄▀ █▄▀ █▄▄    █▀ ▀▄▀ █▀▄     █  █▄▄ ▄█▀  █  █ █ ▀█ ▀▄█    █▀  ▀▄▀ ▀▄█ █▀▄ █▀  ▀▄▀ ▄█▀ █▄▄ ▄█▀ 
//
// async fn read_filings(endpoint: &str) -> Result<Vec<u8>, MyError> {
//     let mut headers = HeaderMap::new();
//     headers.insert(USER_AGENT, "Joordih Development jj@joordih.dev (Jordi Xavier)".parse().unwrap());
//     headers.insert(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
//     headers.insert(HOST, "www.sec.gov".parse().unwrap());

//     let client = Client::new();
    
//     let response = client.get(endpoint)
//         .headers(headers)
//         .send()
//         .await
//         .map_err(MyError::from)?;
    
//     let content_encoding = response.headers().get(reqwest::header::CONTENT_ENCODING).cloned();
    
//     let body = response.bytes().await.map_err(MyError::from)?;
//     let mut decompressed_data = Vec::new();

//     let encoding = content_encoding.and_then(|v| v.to_str().ok().map(|s| s.to_string()));

//     match encoding.as_deref() {
//         Some("gzip") => {
//             let mut decoder = GzDecoder::new(&body[..]);
//             decoder.read_to_end(&mut decompressed_data).map_err(MyError::from)?;
//         },
//         Some("deflate") => {
//             let mut decoder = DeflateDecoder::new(&body[..]);
//             decoder.read_to_end(&mut decompressed_data).map_err(MyError::from)?;
//         },
//         _ => {
//             decompressed_data = body.to_vec();
//         }
//     }

//     Ok(decompressed_data)
// }