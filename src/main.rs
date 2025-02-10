use std::{error::Error, io::Read};
use flate2::read::{DeflateDecoder, GzDecoder};
use reqwest::{header::{HeaderMap, ACCEPT_ENCODING, HOST, USER_AGENT}, Client};
mod stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&type=10-k&company=&dateb=&owner=include&start=0&count=40&output=atom";
    let data = read_filings(url).await?;
        // println!("Atom feed first entry: {:?}", atom_feed.entries().)

    // let atom_str = 

    // match atom_str.parse::<Feed>().unwrap() {
    //     Feed::Atom(atom_feed) => println!("Atom feed first entry: {:?}", atom_feed.entries[0].title),
    //     _ => {}
    // };

    // &data).parse::<Feed>().unwrap() {
    //     Feed::R(atom_feed) => {
    //         Iterator::for_each(atom_feed.entries().iter(), |entry| {
    //             print!("Title: {}", entry.title())
    //         });
    //     }
    // };

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
