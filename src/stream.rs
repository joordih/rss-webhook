use std::io::Cursor;
use std::{io::BufReader, time::Duration};

use app::{DateTime, FixedOffset};

use crate::error::FeedParseError;
use crate::Error;

pub trait FeedReader {
    fn title(&self) -> Option<&str>;
    fn link(&self) -> Option<&str>;
    fn updated(&self) -> Option<DateTime<FixedOffset>>;
    fn category(&self) -> Option<Vec<&str>>;
    fn uuid(&self) -> Option<&str>;
}
 
impl FeedReader for rss::Item {
    fn title(&self) -> Option<&str> {
        self.title()
    }

    fn link(&self) -> Option<&str> {
        self.link()
    }

    fn updated(&self) -> Option<DateTime<FixedOffset>> {
        self.pub_date().and_then(|d| DateTime::parse_from_rfc2822(d).ok())
    }
    
    fn category(&self) -> Option<Vec<&str>> {
        Some(self.categories().iter().map(|c| c.name()).collect())
    }
    
    fn uuid(&self) -> Option<&str> {
        self.guid().map(|guid| guid.value())
    }
}

impl FeedReader for atom_syndication::Entry {
    fn title(&self) -> Option<&str> {
        Some(self.title())
    }

    fn link(&self) -> Option<&str> {
        self.links()
            .iter()
            .find(|link| link.rel() == "alternate")
            .map(|link| link.href())
    }

    fn updated(&self) -> Option<DateTime<FixedOffset>> {
        Some(self.updated().fixed_offset())
    }

    fn category(&self) -> Option<Vec<&str>> {
        Some(self.categories().iter().map(|c| c.term()).collect())
    }

    fn uuid(&self) -> Option<&str> {
        Some(self.id())
    }
}

pub enum Feed {
    RSS(rss::Channel),
    ATOM(atom_syndication::Feed),
}

type Endpoint = reqwest::Url;
pub struct Stream<> {
    endpoint: Endpoint,
    interval: Duration,
}

impl Stream<> {
    pub fn new(endpoint: Endpoint, interval: Duration) -> Stream<> {
        Stream {
            endpoint,
            interval
        }
    }

    pub fn parse_feed(&self, body: &str) -> Result<Feed, Error> {
        match body.parse::<atom_syndication::Feed>() {
            Ok(feed) => Ok(Feed::ATOM(feed)),
            Err(err) => {
                print!("Atom error: {:?}", err);
                
                match rss::Channel::read_from(BufReader::new(Cursor::new(body))) {
                    Ok(feed) => Ok(Feed::RSS((feed))),
                    Err(err) => Err(Error::FeedParseError(FeedParseError::RssError(err)))
                }
            }
         }
    }
}