use chrono::TimeZone;

pub type Timestamp = chrono::DateTime<chrono::Utc>;
pub type DateRange = (Timestamp, Timestamp);

pub fn prost2chrono(ts: &Option<prost_types::Timestamp>) -> Timestamp {
    match ts {
        Some(ts) => chrono::Utc
            .timestamp_opt(ts.seconds, ts.nanos as u32)
            .unwrap(),
        None => chrono::Utc::now(),
    }
}

pub fn chrono2prost(ts: Timestamp) -> Option<prost_types::Timestamp> {
    Some(prost_types::Timestamp {
        seconds: ts.timestamp(),
        nanos: 0,
    })
}

pub fn chrono_now_to_prost() -> Option<prost_types::Timestamp> {
    chrono2prost(chrono::Utc::now())
}
