use crate::api_client::error::APIServiceErrorCode;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Error codes that the App Store Server API may return for Retention Messaging API requests.
///
/// [errorCode](https://developer.apple.com/documentation/retentionmessaging/errorcode)
#[derive(Debug, Copy, Clone, Deserialize_repr, Serialize_repr, PartialEq, Hash)]
#[repr(i64)]
pub enum ApiErrorCode {
    /// An error that indicates the product ID parameter is invalid.
    ///
    /// [InvalidProductIdError](https://developer.apple.com/documentation/appstoreserverapi/invalidproductiderror)
    InvalidProductId = 4000023,

    /// An error that indicates the image that's uploading is invalid.
    ///
    /// [InvalidImageError](https://developer.apple.com/documentation/retentionmessaging/invalidimageerror)
    InvalidImage = 4000161,

    /// An error that indicates the header text is too long.
    ///
    /// [HeaderTooLongError](https://developer.apple.com/documentation/retentionmessaging/headertoolongerror)
    HeaderTooLong = 4000162,

    /// An error that indicates the body text is too long.
    ///
    /// [BodyTooLongError](https://developer.apple.com/documentation/retentionmessaging/bodytoolongerror)
    BodyTooLong = 4000163,

    /// An error that indicates the locale is invalid.
    ///
    /// [InvalidLocaleError](https://developer.apple.com/documentation/retentionmessaging/invalidlocaleerror)
    InvalidLocale = 4000164,

    /// An error that indicates the alternative text for an image is too long.
    ///
    /// [AltTextTooLongError](https://developer.apple.com/documentation/retentionmessaging/alttexttoolongerror)
    AltTextTooLong = 4000175,

    /// An error that indicates when you reach the maximum number of uploaded images.
    ///
    /// [MaximumNumberOfImagesReachedError](https://developer.apple.com/documentation/retentionmessaging/maximumnumberofimagesreachederror)
    MaximumNumberOfImagesReached = 4030014,

    /// An error that indicates when you reach the maximum number of uploaded messages.
    ///
    /// [MaximumNumberOfMessagesReachedError](https://developer.apple.com/documentation/retentionmessaging/maximumnumberofmessagesreachederror)
    MaximumNumberOfMessagesReached = 4030016,

    /// An error that indicates the message isn't in the approved state, so you can't configure it as a default message.
    ///
    /// [MessageNotApprovedError](https://developer.apple.com/documentation/retentionmessaging/messagenotapprovederror)
    MessageNotApproved = 4030017,

    /// An error that indicates the image isn't in the approved state, so you can't configure it as part of a default message.
    ///
    /// [ImageNotApprovedError](https://developer.apple.com/documentation/retentionmessaging/imagenotapprovederror)
    ImageNotApproved = 4030018,

    /// An error that indicates the image is currently in use as part of a message, so you can't delete it.
    ///
    /// [ImageInUseError](https://developer.apple.com/documentation/retentionmessaging/imageinuseerror)
    ImageInUse = 4030019,

    /// An error that indicates the system can't find the image identifier.
    ///
    /// [ImageNotFoundError](https://developer.apple.com/documentation/retentionmessaging/imagenotfounderror)
    ImageNotFound = 4040014,

    /// An error that indicates the system can't find the message identifier.
    ///
    /// [MessageNotFoundError](https://developer.apple.com/documentation/retentionmessaging/messagenotfounderror)
    MessageNotFound = 4040015,

    /// An error that indicates the image identifier already exists.
    ///
    /// [ImageAlreadyExistsError](https://developer.apple.com/documentation/retentionmessaging/imagealreadyexistserror)
    ImageAlreadyExists = 4090000,

    /// An error that indicates the message identifier already exists.
    ///
    /// [MessageAlreadyExistsError](https://developer.apple.com/documentation/retentionmessaging/messagealreadyexistserror)
    MessageAlreadyExists = 4090001,

    /// An error that indicates that the request exceeded the rate limit.
    ///
    /// [RateLimitExceededError](https://developer.apple.com/documentation/appstoreserverapi/ratelimitexceedederror)
    RateLimitExceeded = 4290000,

    /// A general internal error occurred.
    ///
    /// [GeneralInternalError](https://developer.apple.com/documentation/retentionmessaging/generalinternalerror)
    GeneralInternalError = 5000000,

    /// An unknown error
    Unknown = -1,
}

impl APIServiceErrorCode for ApiErrorCode {
    fn code(&self) -> i64 {
        *self as i64
    }
    fn unknown() -> Self {
        Self::Unknown
    }
}