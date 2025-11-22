use serde::{Deserialize, Deserializer};
use serde::de::Unexpected;
use uuid::Uuid;

/// Custom deserializer for optional UUID that treats empty strings as None.
pub fn deserialize_optional_uuid<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        None => Ok(None),
        Some(ref s) if s.is_empty() => Ok(None),
        Some(s) => s.parse::<Uuid>()
            .map(Some)
            .map_err(|e| serde::de::Error::invalid_type(Unexpected::Str(&s), &"Valid Uuid")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(deserialize_with = "deserialize_optional_uuid")]
        id: Option<Uuid>,
    }

    #[test]
    fn test_deserialize_null_uuid() {
        let json = json!({"id": null});
        let result: TestStruct = serde_json::from_value(json).unwrap();
        assert_eq!(result.id, None);
    }

    #[test]
    fn test_deserialize_empty_string_uuid() {
        let json = json!({"id": ""});
        let result: TestStruct = serde_json::from_value(json).unwrap();
        assert_eq!(result.id, None);
    }

    #[test]
    fn test_deserialize_valid_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let json = json!({"id": uuid_str});
        let result: TestStruct = serde_json::from_value(json).unwrap();
        assert_eq!(result.id, Some(Uuid::parse_str(uuid_str).unwrap()));
    }

    #[test]
    fn test_deserialize_invalid_uuid() {
        let json = json!({"id": "not-a-valid-uuid"});
        let result: Result<TestStruct, _> = serde_json::from_value(json);
        assert!(result.is_err());

        // Verify the error message contains something about UUID parsing
        let err = result.unwrap_err();
        let err_msg = err.to_string().to_lowercase();
        assert!(err_msg.contains("uuid") || err_msg.contains("invalid"));
    }
}