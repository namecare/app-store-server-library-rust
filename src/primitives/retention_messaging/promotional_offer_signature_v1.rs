use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// The promotional offer signature you generate using an earlier signature version.
///
/// [promotionalOfferSignatureV1](https://developer.apple.com/documentation/retentionmessaging/promotionaloffersignaturev1)
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PromotionalOfferSignatureV1 {
    /// The Base64-encoded cryptographic signature you generate using the offer parameters.
    pub encoded_signature: String,

    /// The subscription's product identifier.
    ///
    /// [productId](https://developer.apple.com/documentation/retentionmessaging/productid)
    pub product_id: String,

    /// A one-time-use UUID antireplay value you generate.
    pub nonce: Uuid,

    /// The UNIX time, in milliseconds, when you generate the signature.
    pub timestamp: i64,

    /// A string that identifies the private key you use to generate the signature.
    pub key_id: String,

    /// The subscription offer identifier that you set up in App Store Connect.
    pub offer_identifier: String,

    /// A UUID that you provide to associate with the transaction if the customer accepts the promotional offer.
    pub app_account_token: Option<Uuid>,
}

impl Serialize for PromotionalOfferSignatureV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("PromotionalOfferSignatureV1", 7)?;
        state.serialize_field("encodedSignature", &self.encoded_signature)?;
        state.serialize_field("productId", &self.product_id)?;
        state.serialize_field("nonce", &self.nonce.to_string().to_lowercase())?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("keyId", &self.key_id)?;
        state.serialize_field("offerIdentifier", &self.offer_identifier)?;
        if let Some(app_account_token) = &self.app_account_token {
            state.serialize_field("appAccountToken", &app_account_token.to_string().to_lowercase())?;
        } else {
            state.skip_field("appAccountToken")?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for PromotionalOfferSignatureV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Helper {
            encoded_signature: String,
            product_id: String,
            nonce: String,
            timestamp: i64,
            key_id: String,
            offer_identifier: String,
            app_account_token: Option<String>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let nonce = Uuid::parse_str(&helper.nonce)
            .map_err(|_| serde::de::Error::custom("Invalid UUID string for nonce"))?;
        let app_account_token = helper
            .app_account_token
            .map(|s| Uuid::parse_str(&s))
            .transpose()
            .map_err(|_| serde::de::Error::custom("Invalid UUID string for appAccountToken"))?;

        Ok(PromotionalOfferSignatureV1 {
            encoded_signature: helper.encoded_signature,
            product_id: helper.product_id,
            nonce,
            timestamp: helper.timestamp,
            key_id: helper.key_id,
            offer_identifier: helper.offer_identifier,
            app_account_token,
        })
    }
}