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
                    Ok(feed) => Ok(Feed::RSS(feed)),
                    Err(err) => Err(Error::FeedParseError(FeedParseError::RssError(err)))
                }
            }
         }
    }
}


pub struct SecDocument {
    header: FilingHeader,
    filer: FilingFiler,
    document: Vec<FilingDocument>,
}

//
//   █▀ █ █   █ █▄ █ ▄▀     █▀ █ █   ██▀ █▀▄ ▄▀▀ 
//   █▀ █ █▄▄ █ █ ▀█ ▀▄█    █▀ █ █▄▄ █▄▄ █▀▄ ▄█▀ 
//

pub struct FilingFiler<> {
    company_data: CompanyData,
    filing_values: FilingValues,
    business_address: BusinessAddress,
    mail_address: MailAddress,
    former_company: Vec<FormerCompany>,
}

pub struct CompanyData<> {
    name: String,
    cik: String,
    sic: String,
    organization_name: String,
    irs_number: String,
    fiscal_year_end: String,
}

pub struct FilingValues<> {
    form_type: Type,
    sec_act: String,
    sec_file_number: String,
    film_number: String,
}

pub struct BusinessAddress<> {
    primary_street: String,
    secondary_street: String,
    city: String,
    state: String,
    zip: String,
    business_phone: String,
}

pub struct MailAddress {
    primary_street: String,
    secondary_street: String,
    city: String,
    state: String,
    zip: String,
}

pub struct FormerCompany {
    former_conformed_name: String,
    date_of_name_change: String
}

pub struct FilingHeader<> {
    accession_number: String,
    filing_type: Type,
    accepted_date: DateTime<FixedOffset>,
    period_of_report: DateTime<FixedOffset>,
    filed_date: DateTime<FixedOffset>,
    changed_date: DateTime<FixedOffset>,
}

pub struct FilingDocument {
    document_type: Type,
    sequence: String,
    filename: String,
    description: String,
    document_label: String,
    document_url: String,
}

pub enum Type {
    K10,
    Q10,
    K8
}

//
//   █▀ █ █   █ █▄ █ ▄▀     █▀ █ █   ██▀ █▀▄ ▄▀▀ 
//   █▀ █ █▄▄ █ █ ▀█ ▀▄█    █▀ █ █▄▄ █▄▄ █▀▄ ▄█▀ 
//