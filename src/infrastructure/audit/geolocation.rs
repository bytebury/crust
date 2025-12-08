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

pub fn get_country_details(ip: IpAddr) -> Option<CountryDetails> {
    let db = DB::from_file(IPV6BIN).ok()?;

    let record = db.ip_lookup(ip).ok()?;
    let rec = match record {
        Record::LocationDb(rec) => rec,
        _ => return None,
    };

    let country = rec.country?;

    // This means that we didn't find a country.
    if country.long_name == "-" {
        return None;
    }

    Some(CountryDetails {
        name: Some(country.long_name.to_string()),
        code: Some(country.short_name.to_string()),
    })
}
