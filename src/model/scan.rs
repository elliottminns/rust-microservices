use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::string::String;

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Scan {
    pub ip: String,
    pub port: i16,
    pub load_time_nanosec: i64,
    pub content_hash: String,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn scan_should_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let data = "{\
                        \"ip\":\"8.8.8.8\",\
                        \"port\": 80,
                        \"load_time_nanosec\":50000, \
                        \"content_hash\":\"73d1a9ab21fce25e\", \
                        \"timestamp\":\"2022-07-31T14:17:00Z\" \
                    }";

        let s: Scan  = serde_json::from_str(data)?;

        assert_eq!(s.ip, "8.8.8.8");
        assert_eq!(s.port, 80);
        assert_eq!(s.load_time_nanosec, 50000);
        assert_eq!(s.content_hash, "73d1a9ab21fce25e");
        assert_eq!(s.timestamp, Utc.ymd(2022, 7, 31).and_hms(14, 17, 0));

        Ok(())
    }
}
