use crate::model::Scan;
use std::collections::HashMap;
use std::string::String;

pub struct Store {
    map: HashMap<String, Scan>
}

impl Store {
    pub fn new() -> Self {
        Store{
            map: HashMap::new(),
        }
    }

    fn key_for_record(scan: &Scan) -> String {
        Store::key_for_ip_port(&scan.ip, scan.port)
    }

    fn key_for_ip_port(ip: &str, port: i16) -> String {
        format!("{}:{}", ip, port)
    }

    pub fn insert_record(&mut self, scan: Scan) {
        let key = Store::key_for_record(&scan);
        self.map.insert(key, scan);
    }

    pub fn get_all(&self) -> Vec<Scan> {
        let mut res = Vec::new();

        for (_, val) in self.map.iter() {
            res.push(val.clone())
        }
        
       res.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

       res
    }

    pub fn get_record(&self, ip: &str, port: i16) -> Option<Scan> {
        let key = format!("{}:{}", ip, port);
        let res: Option<&Scan> = self.map.get(&key);
        return res.cloned();
    }

    pub fn update_record(&mut self, scan: Scan) -> Result<(), &'static str> {
        match self.get_record(&scan.ip, scan.port) {
            None => Err("no record exists"),
            Some(_) => {
                self.insert_record(scan);
                Ok(())
            }
        }
    }

    pub fn delete_record(&mut self, ip: &str, port: i16) -> Result<(), &'static str> {
        let key = Store::key_for_ip_port(ip, port);
        match self.map.remove(&key) {
            None => Err("no record for key"),
            Some(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use chrono::{Utc,TimeZone};

    #[test]
    fn store_insert() -> Result<(), Box<dyn Error>> {
        let mut store = Store::new();

        let record = Scan{
            ip: "1.2.3.4".to_owned(),
            port: 80,
            load_time_nanosec: 18,
            content_hash: "foobar".to_owned(),
            timestamp: Utc::now(),
        };

        store.insert_record(record);

        assert_eq!(store.map.contains_key("1.2.3.4:80"), true);

        Ok(())
    }

    #[test]
    fn store_get_record() -> Result<(), Box<dyn Error>> {
        let mut store = Store::new();

        let now = Utc::now();

        let record = Scan{
            ip: "1.2.3.4".to_owned(),
            port: 80,
            load_time_nanosec: 18,
            content_hash: "foobar".to_owned(),
            timestamp: now,
        };

        store.insert_record(record);

        let res = store.get_record("1.2.3.4", 80);
        assert!(res.is_some());

        let res = res.unwrap();
        
        assert_eq!(res.ip, "1.2.3.4");
        assert_eq!(res.port, 80);
        assert_eq!(res.load_time_nanosec, 18);
        assert_eq!(res.content_hash, "foobar");
        assert_eq!(res.timestamp, now);

        Ok(())
    }

    #[test]
    fn store_delete_record() -> Result<(), Box<dyn Error>> {
        let mut store = Store::new();
        let res = store.delete_record("foo", 20);

        assert!(res.is_err());

        // Insert a record now
        let record = Scan{
            ip: "1.2.3.4".to_owned(),
            port: 80,
            load_time_nanosec: 18,
            content_hash: "foobar".to_owned(),
            timestamp: Utc::now(),
        };

        store.insert_record(record);

        let res = store.delete_record("1.2.3.4", 80);

        assert!(res.is_ok());

        assert_eq!(store.map.contains_key("1.2.3.4:80"), false);


        Ok(())
    }

    #[test]
    fn store_get_all() -> Result<(), Box<dyn Error>> {
        let mut store = Store::new();

        // Insert a record now
        let record = Scan{
            ip: "1.2.3.4".to_owned(),
            port: 80,
            load_time_nanosec: 18,
            content_hash: "foobar".to_owned(),
            timestamp: Utc::now(),
        };

        store.insert_record(record);

        let record = Scan{
            ip: "8.8.8.8".to_owned(),
            port: 443,
            load_time_nanosec: 500,
            content_hash: "prev".to_owned(),
            timestamp: Utc.ymd(2021, 9, 20).and_hms(17, 10, 0),
        };

        store.insert_record(record);

        let res = store.get_all();

        assert_eq!(res.get(0).unwrap().ip, "8.8.8.8");
        assert_eq!(res.get(1).unwrap().ip, "1.2.3.4");

        Ok(())
    }

    #[test]
    fn store_update() -> Result<(), Box<dyn Error>> {
        let mut store = Store::new();

        let mut record = Scan{
            ip: "1.2.3.4".to_owned(),
            port: 80,
            load_time_nanosec: 18,
            content_hash: "foobar".to_owned(),
            timestamp: Utc::now(),
        };

        let res = store.update_record(record.clone());

        assert!(res.is_err());

        store.insert_record(record.clone());

        record.load_time_nanosec = 20;
        record.content_hash = "barfoo".to_owned();

        let res = store.update_record(record);

        assert!(res.is_ok());

        let res = store.get_record("1.2.3.4", 80);

        assert!(res.is_some());

        let res = res.unwrap();

        assert_eq!(res.content_hash, "barfoo".to_owned());
        assert_eq!(res.load_time_nanosec, 20);

        Ok(())
    }
}
