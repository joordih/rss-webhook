use std::{error::Error, io::{Read, Write}, sync::mpsc::channel, thread};
use flate2::read::{DeflateDecoder, GzDecoder};
use reqwest::{header::{HeaderMap, ACCEPT_ENCODING, HOST, USER_AGENT}, Client};
use rss::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&type=10-k&company=&dateb=&owner=include&start=0&count=40&output=atom";
    let (tx, rx) = channel();  // Create a channel for data transmission

    // Run the feed reader in a separate thread
    let handle = thread::spawn(move || {
        let _ = feed_reader(url, tx);  // Passing tx to the thread to send data
    });

    // Wait for the thread to finish and get the decompressed data
    handle.join().unwrap();  // Wait for the thread to finish (handles any panics)

    // Receive the decompressed data
    match rx.recv() {
        Ok(decompressed_data) => {
            // Process the decompressed data and create the RSS channel
            let channel = Channel::read_from(&decompressed_data[..])?;
            channel.write_to(::std::io::sink()).unwrap();  // You can write the data to stdout or any other destination

            // Convert channel to string and print it
            let string = channel.to_string();
            print!("{}", string);
        },
        Err(e) => {
            eprintln!("Error receiving data from channel: {}", e);
        }
    }

    Ok(())
}

async fn feed_reader(endpoint: &str, tx: std::sync::mpsc::Sender<Vec<u8>>) -> Result<(), Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Joordih Development jj@joordih.dev (Jordi Xavier)".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    headers.insert(HOST, "www.sec.gov".parse().unwrap());

    let client = Client::new();
    
    let response = client.get(endpoint)
        .headers(headers)
        .send()
        .await?;
    
    let content_encoding = response.headers().get(reqwest::header::CONTENT_ENCODING).cloned();
    
    let body = response.bytes().await?;
    let mut decompressed_data = Vec::new();  // Use Vec<u8> to store decompressed data

    let encoding = content_encoding.and_then(|v| v.to_str().ok().map(|s| s.to_string()));

    match encoding.as_deref() {
        Some("gzip") => {
            let mut decoder = GzDecoder::new(&body[..]);
            decoder.read_to_end(&mut decompressed_data)?;
        },
        Some("deflate") => {
            let mut decoder = DeflateDecoder::new(&body[..]);
            decoder.read_to_end(&mut decompressed_data)?;
        },
        _ => {
            decompressed_data = body.to_vec();
        }
    }

    // Send the decompressed data through the channel to the main thread
    if let Err(e) = tx.send(decompressed_data) {
        eprintln!("Error sending data through the channel: {}", e);
    }

    Ok(())
}


async fn read_filings(endpoint: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Joordih Development jj@joordih.dev (Jordi Xavier)".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    headers.insert(HOST, "www.sec.gov".parse().unwrap());

    let client = Client::new();
    
    let response = client.get(endpoint)
        .headers(headers)
        .send()
        .await?;
    
    let content_encoding = response.headers().get(reqwest::header::CONTENT_ENCODING).cloned();
    
    let body = response.bytes().await?;
    let mut decompressed_data = Vec::new();

    let encoding = content_encoding.and_then(|v| v.to_str().ok().map(|s| s.to_string()));

    match encoding.as_deref() {
        Some("gzip") => {
            let mut decoder = GzDecoder::new(&body[..]);
            decoder.read_to_end(&mut decompressed_data)?;
        },
        Some("deflate") => {
            let mut decoder = DeflateDecoder::new(&body[..]);
            decoder.read_to_end(&mut decompressed_data)?;
        },
        _ => {
            decompressed_data = body.to_vec();
        }
    }

    Ok(decompressed_data)
}