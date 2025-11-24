use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Unexpected;
use uuid::Uuid;

/// Custom deserializer for optional UUID that treats empty strings as None.
pub fn de_string_as_optional_uuid<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        None => Ok(None),
        Some(ref s) if s.is_empty() => Ok(None),
        Some(s) => s.parse::<Uuid>()
            .map(Some)
            .map_err(|_e| serde::de::Error::invalid_type(Unexpected::Str(&s), &"Valid Uuid")),
    }
}

/// Custom serializer for optional UUID that serializes None as an empty string.
pub fn ser_optional_uuid_as_string<S>(
    value: &Option<Uuid>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(uuid) => serializer.serialize_str(&uuid.to_string()),
        None => serializer.serialize_str(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(
            deserialize_with = "de_string_as_optional_uuid",
            serialize_with = "ser_optional_uuid_as_string"
        )]
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

    #[test]
    fn test_serialize_none_as_empty_string() {
        let test_struct = TestStruct { id: None };
        let json = serde_json::to_value(&test_struct).unwrap();
        assert_eq!(json, json!({"id": ""}));
    }

    #[test]
    fn test_serialize_some_uuid() {
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let test_struct = TestStruct { id: Some(uuid) };
        let json = serde_json::to_value(&test_struct).unwrap();
        assert_eq!(json, json!({"id": "550e8400-e29b-41d4-a716-446655440000"}));
    }

    #[test]
    fn test_roundtrip_none() {
        let original = TestStruct { id: None };
        let json = serde_json::to_value(&original).unwrap();
        let deserialized: TestStruct = serde_json::from_value(json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_roundtrip_some() {
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let original = TestStruct { id: Some(uuid) };
        let json = serde_json::to_value(&original).unwrap();
        let deserialized: TestStruct = serde_json::from_value(json).unwrap();
        assert_eq!(original, deserialized);
    }
}