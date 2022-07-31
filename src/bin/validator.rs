use data::Scan;
use std::error::Error;
use chrono::{Utc,TimeZone};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    // Check that scans are empty
    let resp = client.get("http://localhost:8080/v1/scans")
        .send()
        .await?
        .json::<Vec<Scan>>()
        .await?;

    assert_eq!(resp.len(), 0, "micro service instance is not in correct state");

    let scan0_body = "{\"ip\":\"8.8.8.8\",\"port\":80,\
        \"content_hash\":\"foobar\",\"load_time_nanosec\":100,\
        \"timestamp\":\"2022-07-31T16:26:16Z\"}";

    let scan1_body = "{\"ip\":\"1.1.1.1\",\"port\":443,\
        \"content_hash\":\"barfoo\",\"load_time_nanosec\":231,\
        \"timestamp\":\"2022-06-20T17:10:32Z\"}";

    let scan1_update = "{\"ip\":\"1.1.1.1\",\"port\":443,\
        \"content_hash\":\"bazbarfoo\",\"load_time_nanosec\":8912,\
        \"timestamp\":\"2022-06-20T17:10:32Z\"}";

    // Create Scan 0
    let resp = client.post("http://localhost:8080/v1/scans")
        .body(scan0_body)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    assert_eq!(resp.status(), 201, "create scan 0 should have correct status");

    // Create Scan 0 again
    let resp = client.post("http://localhost:8080/v1/scans")
        .body(scan0_body)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    assert_eq!(resp.status(), 400, "second create scan 0 should have failed status");

    // Create Scan 1
    let resp = client.post("http://localhost:8080/v1/scans")
        .body(scan1_body)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    assert_eq!(resp.status(), 201, "create scan 1 should have correct status");

    // Read All Scans
    // Check that scans are empty
    let resp = client.get("http://localhost:8080/v1/scans")
        .send()
        .await?
        .json::<Vec<Scan>>()
        .await?;

    assert_eq!(resp.len(), 2, "creates did not work");

    assert_eq!(resp.get(0).unwrap().ip, "1.1.1.1", "scans should be ordered correctly");

    // Read Scan 0

    let resp = client.get("http://localhost:8080/v1/scans/8.8.8.8/80")
        .send()
        .await?
        .json::<Option<Scan>>()
        .await?;

    assert!(resp.is_some());

    let scan = resp.unwrap();
    assert_eq!(scan.ip, "8.8.8.8", "ip should be correct");
    assert_eq!(scan.port, 80, "port should be correct");
    assert_eq!(scan.content_hash, "foobar", "content hash should be correct");
    assert_eq!(scan.load_time_nanosec, 100, "load time should be correct");
    assert_eq!(
        scan.timestamp,
        Utc.ymd(2022, 7, 31).and_hms(16, 26, 16), 
        "timestamp should be correct"
    );

    // Read Scan 1
    let resp = client.get("http://localhost:8080/v1/scans/1.1.1.1/443")
        .send()
        .await?
        .json::<Option<Scan>>()
        .await?;

    assert!(resp.is_some());

    let scan = resp.unwrap();
    assert_eq!(scan.ip, "1.1.1.1", "ip should be correct");
    assert_eq!(scan.port, 443, "port should be correct");
    assert_eq!(scan.content_hash, "barfoo", "content hash should be correct");
    assert_eq!(scan.load_time_nanosec, 231, "load time should be correct");
    assert_eq!(
        scan.timestamp,
        Utc.ymd(2022, 6, 20).and_hms(17, 10, 32), 
        "timestamp should be correct"
    );

    // Read no scan
    let resp = client.get("http://localhost:8080/v1/scans/1.1.1.1/80")
        .send()
        .await?
        .json::<Option<Scan>>()
        .await?;

    assert!(resp.is_none(), "should return null for no scan");

    // Update Scan 1
    let resp = client.put("http://localhost:8080/v1/scans")
        .body(scan1_update)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    assert_eq!(resp.status(), 200, "update should have a successful response");

    let resp = client.get("http://localhost:8080/v1/scans/1.1.1.1/443")
        .send()
        .await?
        .json::<Option<Scan>>()
        .await?;

    assert!(resp.is_some(), "should return some for an existing object");

    let scan = resp.unwrap();
    assert_eq!(scan.ip, "1.1.1.1", "ip should be correct");
    assert_eq!(scan.port, 443, "port should be correct");
    assert_eq!(scan.content_hash, "bazbarfoo", "content hash should be correct");
    assert_eq!(scan.load_time_nanosec, 8912, "load time should be correct");
    assert_eq!(
        scan.timestamp,
        Utc.ymd(2022, 6, 20).and_hms(17, 10, 32), 
        "timestamp should be correct"
    );

    // Update non existant scan
    let scan0_non = "{\"ip\":\"8.8.8.8\",\"port\":443,\
        \"content_hash\":\"bazbarfoo\",\"load_time_nanosec\":8912,\
        \"timestamp\":\"2022-06-20T17:10:32Z\"}";

    let resp = client.put("http://localhost:8080/v1/scans")
        .body(scan0_non)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    assert_eq!(resp.status(), 400, "updating non existing record should fail");

    // Delete Scans
    let resp = client.delete("http://localhost:8080/v1/scans/8.8.8.8/80")
        .send()
        .await?;

    assert_eq!(resp.status(), 200, "deleting scan 0 should succeed");

    let resp = client.delete("http://localhost:8080/v1/scans/1.1.1.1/443")
        .send()
        .await?;

    assert_eq!(resp.status(), 200, "deleting scan 1 should succeed");

    // Check that scans are empty
    let resp = client.get("http://localhost:8080/v1/scans")
        .send()
        .await?
        .json::<Vec<Scan>>()
        .await?;

    assert_eq!(resp.len(), 0, "deletes did not work as expected");

    println!("All tests successfully passed");

    Ok(())
}
