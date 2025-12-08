use chrono::NaiveDateTime;
use ip2location::{DB, Record};
use sqlx::FromRow;
use std::net::IpAddr;

const IPV6BIN: &str = "db/ip2location.BIN";

#[derive(FromRow, Clone)]
pub struct Country {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Default)]
pub struct CountryDetails {
    pub name: Option<String>,
    pub code: Option<String>,
}

impl TryFrom<IpAddr> for CountryDetails {
    type Error = ip2location::error::Error;

    fn try_from(value: IpAddr) -> Result<Self, Self::Error> {
        let db = DB::from_file(IPV6BIN)?;

        let record = db.ip_lookup(value)?;
        let rec = match record {
            Record::LocationDb(rec) => rec,
            _ => {
                return Err(ip2location::error::Error::RecordNotFound);
            }
        };

        let country = match rec.country {
            Some(country) => country,
            None => return Err(ip2location::error::Error::RecordNotFound),
        };

        Ok(CountryDetails {
            name: Some(country.long_name.to_string()),
            code: Some(country.short_name.to_string()),
        })
    }
}
