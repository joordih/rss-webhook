use std::time::Duration;
mod lib;

pub trait FeedReader {
    fn title(&self) -> Option<&str>;
    fn link(&self) -> Option<&str>;
    fn updated(&self) -> Option<>
    fn uuid(&self) -> Option<&str>;
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