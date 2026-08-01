use chrono::{DateTime, Utc};

pub trait DecodedSignedData {
    fn signed_date_optional(&self) -> Option<DateTime<Utc>>;
}