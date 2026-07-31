use crate::api_client::error::APIServiceErrorCode;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Enum representing different API errors with associated status codes.
#[derive(Debug, Copy, Clone, Deserialize_repr, Serialize_repr, PartialEq, Hash)]
#[repr(i64)]
pub enum ApiErrorCode {
    /// An error that indicates an invalid request.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/generalbadrequesterror)
    GeneralBadRequest = 4000000,

    /// An error that indicates an invalid app identifier.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidappidentifiererror)
    InvalidAppIdentifier = 4000002,

    /// An error that indicates an invalid request revision.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidrequestrevisionerror)
    InvalidRequestRevision = 4000005,

    /// An error that indicates an invalid transaction identifier.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidtransactioniderror)
    InvalidTransactionId = 4000006,

    /// An error that indicates an invalid original transaction identifier.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidoriginaltransactioniderror)
    InvalidOriginalTransactionId = 4000008,

    /// An error that indicates an invalid extend-by-days value.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidextendbydayserror)
    InvalidExtendByDays = 4000009,

    /// An error that indicates an invalid reason code.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidextendreasoncodeerror)
    InvalidExtendReasonCode = 4000010,

    /// An error that indicates an invalid request identifier.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidrequestidentifiererror)
    InvalidRequestIdentifier = 4000011,

    /// An error that indicates that the start date is earlier than the earliest allowed date.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/startdatetoofarinpasterror)
    StartDateTooFarInPast = 4000012,

    /// An error that indicates that the end date precedes the start date, or the two dates are equal.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/startdateafterenddateerror)
    StartDateAfterEndDate = 4000013,

    /// An error that indicates the pagination token is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidpaginationtokenerror)
    InvalidPaginationToken = 4000014,

    /// An error that indicates the start date is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidstartdateerror)
    InvalidStartDate = 4000015,

    /// An error that indicates the end date is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidenddateerror)
    InvalidEndDate = 4000016,

    /// An error that indicates the pagination token expired.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/paginationtokenexpirederror)
    PaginationTokenExpired = 4000017,

    /// An error that indicates the notification type or subtype is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidnotificationtypeerror)
    InvalidNotificationType = 4000018,

    /// An error that indicates the request is invalid because it has too many constraints applied.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/multiplefilterssuppliederror)
    MultipleFiltersSupplied = 4000019,

    /// An error that indicates the test notification token is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidtestnotificationtokenerror)
    InvalidTestNotificationToken = 4000020,

    /// An error that indicates an invalid sort parameter.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidsorterror)
    InvalidSort = 4000021,

    /// An error that indicates an invalid product type parameter.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidproducttypeerror)
    InvalidProductType = 4000022,

    /// An error that indicates the product ID parameter is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidproductiderror)
    InvalidProductId = 4000023,

    /// An error that indicates an invalid subscription group identifier.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidsubscriptiongroupidentifiererror)
    InvalidSubscriptionGroupIdentifier = 4000024,

    /// An error that indicates the query parameter exclude-revoked is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidexcluderevokederror)
    InvalidExcludeRevoked = 4000025,

    /// An error that indicates an invalid in-app ownership type parameter.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidinappownershiptypeerror)
    InvalidInAppOwnershipType = 4000026,

    /// An error that indicates a required storefront country code is empty.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidemptystorefrontcountrycodelisterror)
    InvalidEmptyStorefrontCountryCodeList = 4000027,

    /// An error that indicates a storefront code is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidstorefrontcountrycodeerror)
    InvalidStorefrontCountryCode = 4000028,

    /// An error that indicates the revoked parameter contains an invalid value.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidrevokederror)
    InvalidRevoked = 4000030,

    /// An error that indicates the status parameter is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidstatuserror)
    InvalidStatus = 4000031,

    /// An error that indicates the value of the account tenure field is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidaccounttenureerror)
    InvalidAccountTenure = 4000032,

    /// An error that indicates the value of the app account token is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidappaccounttokenerror)
    InvalidAppAccountToken = 4000033,

    /// An error that indicates the consumption status is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidconsumptionstatuserror)
    InvalidConsumptionStatus = 4000034,

    /// An error that indicates the customer consented status is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidcustomerconsentederror)
    InvalidCustomerConsented = 4000035,

    /// An error that indicates the delivery status is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invaliddeliverystatuserror)
    InvalidDeliveryStatus = 4000036,

    /// An error that indicates the lifetime dollars purchased field is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidlifetimedollarspurchasederror)
    InvalidLifetimeDollarsPurchased = 4000037,

    /// An error that indicates the lifetime dollars refunded field is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidlifetimedollarsrefundederror)
    InvalidLifetimeDollarsRefunded = 4000038,

    /// An error that indicates the platform parameter is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidplatformerror)
    InvalidPlatform = 4000039,

    /// An error that indicates the play time parameter is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidplaytimeerror)
    InvalidPlayTime = 4000040,

    /// An error that indicates the sample content provided parameter is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invalidsamplecontentprovidederror)
    InvalidSampleContentProvided = 4000041,

    /// An error that indicates the user status parameter is invalid.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/invaliduserstatuserror)
    InvalidUserStatus = 4000042,

    /// An error that indicates the transaction is not consumable.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/transactionnotconsumableerror)
    #[deprecated(since = "2.1.0")]
    InvalidTransactionNotConsumable = 4000043,

    /// An error that indicates the transaction identifier represents an unsupported in-app purchase type.
    ///
    /// [InvalidTransactionTypeNotSupportedError](https://developer.apple.com/documentation/appstoreserverapi/invalidtransactiontypenotsupportederror)
    InvalidTransactionTypeNotSupported = 4000047,

    /// An error that indicates the endpoint doesn't support an app transaction ID.
    ///
    /// [AppTransactionIdNotSupportedError](https://developer.apple.com/documentation/appstoreserverapi/apptransactionidnotsupportederror)
    AppTransactionIdNotSupportedError = 4000048,

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

    /// An error that indicates the app account token value is not a valid UUID.
    ///
    /// [InvalidAppAccountTokenUUID](https://developer.apple.com/documentation/appstoreserverapi/invalidappaccounttokenuuiderror)
    InvalidAppAccountTokenUUID = 4000183,

    /// An error that indicates the transaction is for a product the customer obtains through Family Sharing, which the endpoint doesn't support.
    ///
    /// [FamilyTransactionNotSupported](https://developer.apple.com/documentation/appstoreserverapi/familytransactionnotsupportederror)
    FamilyTransactionNotSupported = 4000185,

    /// An error that indicates the endpoint expects an original transaction identifier.
    ///
    /// [TransactionIdNotOriginalTransactionId](https://developer.apple.com/documentation/appstoreserverapi/transactionidnotoriginaltransactioniderror)
    TransactionIdNotOriginalTransactionId = 4000187,

    /// An error the API returns that indicates the performance test request is invalid.
    ///
    /// [InvalidPerformanceTestRequestError](https://developer.apple.com/documentation/retentionmessaging/invalidperformancetestrequesterror)
    InvalidPerformanceTestRequest = 4000211,

    /// An error that indicates the request ID is invalid.
    ///
    /// [InvalidRequestIdError](https://developer.apple.com/documentation/retentionmessaging/invalidrequestiderror)
    InvalidRequestId = 4000212,

    /// An error that indicates an error with an existing test.
    ///
    /// [ExistingPerformanceTestRunError](https://developer.apple.com/documentation/retentionmessaging/existingperformancetestrunerror)
    ExistingPerformanceTestRun = 4000213,

    /// An error that indicates the URL is invalid.
    ///
    /// [BadRequestRealtimeUrlError](https://developer.apple.com/documentation/retentionmessaging/badrequestrealtimeurlerror)
    BadRequestRealtimeUrl = 4000215,

    /// An error that indicates the image size provided is invalid.
    ///
    /// [BadRequestImageSizeError](https://developer.apple.com/documentation/retentionmessaging/badrequestimagesizeerror)
    BadRequestImageSize = 4000216,

    /// An error that indicates there are too many bullet points.
    ///
    /// [BadRequestTooManyBulletPointsError](https://developer.apple.com/documentation/retentionmessaging/badrequesttoomanybulletpointserror)
    BadRequestTooManyBulletPoints = 4000218,

    /// An error that indicates the text for a bullet point is too long.
    ///
    /// [BadRequestBulletPointTextTooLongError](https://developer.apple.com/documentation/retentionmessaging/badrequestbulletpointtexttoolongerror)
    BadRequestBulletPointTextTooLong = 4000219,

    /// An error that indicates that no image object is included, but the request indicates that the header should be placed above the image.
    ///
    /// [BadRequestAboveImageRequiresAnImageError](https://developer.apple.com/documentation/retentionmessaging/badrequestaboveimagerequiresanimageerror)
    BadRequestAboveImageRequiresAnImage = 4000224,

    /// An error that indicates the subscription doesn't qualify for a renewal-date extension due to its subscription state.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/subscriptionextensionineligibleerror)
    SubscriptionExtensionIneligible = 4030004,

    /// An error that indicates the subscription doesn’t qualify for a renewal-date extension because it has already received the maximum extensions.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/subscriptionmaxextensionerror)
    SubscriptionMaxExtension = 4030005,

    /// An error that indicates a subscription isn't directly eligible for a renewal date extension because the user obtained it through Family Sharing.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/familysharedsubscriptionextensionineligibleerror)
    FamilySharedSubscriptionExtensionIneligible = 4030007,

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

    /// An error that indicates that passing a performance test is required before you can set a URL for the production environment.
    ///
    /// [ForbiddenNoPassingTestError](https://developer.apple.com/documentation/retentionmessaging/forbiddennopassingtesterror)
    ForbiddenNoPassingTest = 4030026,

    /// An error that indicates the App Store account wasn’t found.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/accountnotfounderror)
    AccountNotFound = 4040001,

    /// An error response that indicates the App Store account wasn’t found, but you can try again.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/accountnotfoundretryableerror)
    AccountNotFoundRetryable = 4040002,

    /// An error that indicates the app wasn’t found.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/appnotfounderror)
    AppNotFound = 4040003,

    /// An error response that indicates the app wasn’t found, but you can try again.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/appnotfoundretryableerror)
    AppNotFoundRetryable = 4040004,

    /// An error that indicates an original transaction identifier wasn't found.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/originaltransactionidnotfounderror)
    OriginalTransactionIdNotFound = 4040005,

    /// An error response that indicates the original transaction identifier wasn’t found, but you can try again.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/originaltransactionidnotfoundretryableerror)
    OriginalTransactionIdNotFoundRetryable = 4040006,

    /// An error that indicates that the App Store server couldn’t find a notifications URL for your app in this environment.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/servernotificationurlnotfounderror)
    ServerNotificationUrlNotFound = 4040007,

    /// An error that indicates that the test notification token is expired or the test notification status isn’t available.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/testnotificationnotfounderror)
    TestNotificationNotFound = 4040008,

    /// An error that indicates the server didn't find a subscription-renewal-date extension request for the request identifier and product identifier you provided.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/statusrequestnotfounderror)
    StatusRequestNotFound = 4040009,

    /// An error that indicates a transaction identifier wasn't found.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/transactionidnotfounderror)
    TransactionIdNotFound = 4040010,

    /// An error that indicates the system can't find the image identifier.
    ///
    /// [ImageNotFoundError](https://developer.apple.com/documentation/retentionmessaging/imagenotfounderror)
    ImageNotFound = 4040014,

    /// An error that indicates the system can't find the message identifier.
    ///
    /// [MessageNotFoundError](https://developer.apple.com/documentation/retentionmessaging/messagenotfounderror)
    MessageNotFound = 4040015,

    /// An error the API returns if the service can't find the specified test run.
    ///
    /// [PerformanceTestRunNotFoundError](https://developer.apple.com/documentation/retentionmessaging/performancetestrunnotfounderror)
    PerformanceTestRunNotFound = 4040018,

    /// An error response that indicates an app transaction doesn't exist for the specified customer.
    ///
    /// [AppTransactionDoesNotExistError](https://developer.apple.com/documentation/appstoreserverapi/apptransactiondoesnotexisterror)
    AppTransactionDoesNotExist = 4040019,

    /// An error that indicates a default message isn’t configured.
    ///
    /// [DefaultMessageNotFoundError](https://developer.apple.com/documentation/retentionmessaging/defaultmessagenotfounderror)
    DefaultMessageNotFound = 4040020,

    /// An error that indicates that the URL for your endpoint isn’t configured.
    ///
    /// [RealtimeUrlNotFoundError](https://developer.apple.com/documentation/retentionmessaging/realtimeurlnotfounderror)
    RealtimeUrlNotFound = 4040021,

    /// An error that indicates the image identifier already exists.
    ///
    /// [ImageAlreadyExistsError](https://developer.apple.com/documentation/retentionmessaging/imagealreadyexistserror)
    ImageAlreadyExists = 4090000,

    /// An error that indicates the message identifier already exists.
    ///
    /// [MessageAlreadyExistsError](https://developer.apple.com/documentation/retentionmessaging/messagealreadyexistserror)
    MessageAlreadyExists = 4090001,

    /// An error that indicates that the request exceeded the rate limit.
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/ratelimitexceedederror)
    RateLimitExceeded = 4290000,

    /// An error that indicates a general internal error.
    ///
    /// [GeneralInternalError](https://developer.apple.com/documentation/appstoreserverapi/generalinternalerror)
    GeneralInternal = 5000000,

    /// An error response that indicates an unknown error occurred, but you can try again.
    ///
    /// [GeneralInternalRetryableError](https://developer.apple.com/documentation/appstoreserverapi/generalinternalretryableerror)
    GeneralInternalRetryable = 5000001,

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