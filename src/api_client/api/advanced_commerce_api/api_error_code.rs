use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::api_client::error::APIServiceErrorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum APIErrorCode {
    /// The transaction was already refunded.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/alreadyrefunded)
    AlreadyRefunded = 4030021,

    /// When included, provide at least one item in items.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/atleastoneitem)
    AtLeastOneItem = 4000160,

    /// Provide either the displayName or a description.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/atleastoneofdisplaynameordescription)
    AtLeastOneOfDisplayNameOrDescription = 4000165,

    /// Bill cycle reset with effective later.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/billingcycleresetwitheffectivelater)
    BillingCycleResetWithEffectiveLater = 4000148,

    /// The targeted item in changeItems wasn't found.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/changeitemnotfound)
    ChangeItemNotFound = 4000146,

    /// Exceeds the maximum length of the description field.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/descriptionlengthexceeded)
    DescriptionLengthExceeded = 4000088,

    /// Exceeds the maximum length of the displayName field.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/displaynamelengthexceeded)
    DisplayNameLengthExceeded = 4000089,

    /// The addItems and changeItems entries cannot be empty.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/emptyaddchangeitems)
    EmptyAddChangeItems = 4000139,

    /// An unknown error occurred.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/generalinternal)
    GeneralInternal = 5000000,

    /// An unknown error occurred. Please try again.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/generalinternalretryable)
    GeneralInternalRetryable = 5000001,

    /// The subscription is not active.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/inactiveacasub)
    InactiveACASub = 4030015,

    /// Insufficient funds for refund.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/insufficientfunds)
    InsufficientFunds = 4030020,

    /// The amount is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidamount)
    InvalidAmount = 4000132,

    /// The appAccountToken field must contain a valid UUID or an empty string.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidappaccounttoken)
    InvalidAppAccountToken = 4000033,

    /// The change reason is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidchangereason)
    InvalidChangeReason = 4000125,

    /// The consistencyToken value is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidconsistencytoken)
    InvalidConsistencyToken = 4000082,

    /// The currency value is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidcurrency)
    InvalidCurrency = 4000053,

    /// The description is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invaliddescription)
    InvalidDescription = 4000119,

    /// The displayName is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invaliddisplayname)
    InvalidDisplayName = 4000118,

    /// The offer periodCount is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidofferperiodcount)
    InvalidOfferPeriodCount = 4000129,

    /// The offer period is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidofferperiod)
    InvalidOfferPeriod = 4000128,

    /// The subscription offer price is higher than the regular subscription price.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidofferprice)
    InvalidOfferPrice = 4000152,

    /// The offer reason is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidofferreason)
    InvalidOfferReason = 4000126,

    /// The operation is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidoperation)
    InvalidOperation = 4000172,

    /// The previous subscription targeted is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidprevioussubscription)
    InvalidPreviousSubscription = 4000113,

    /// Previous original transaction id is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidprevioustransactionid)
    InvalidPreviousTransactionID = 4000096,

    /// Product changes are invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidproductchanges)
    InvalidProductChanges = 4000115,

    /// The requested product to change doesn't exist.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidproduct)
    InvalidProduct = 4000121,

    /// The prorated price was invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidproratedprice)
    InvalidProratedPrice = 4000151,

    /// The refundReason is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidrefundreason)
    InvalidRefundReason = 4000124,

    /// The refundType is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidrefundtype)
    InvalidRefundType = 4000123,

    /// The renewal period is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidrenewalperiod)
    InvalidRenewalPeriod = 4000130,

    /// The renewal price is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidrenewalprice)
    InvalidRenewalPrice = 4000131,

    /// The requestReferenceId value is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidrequestreferenceid)
    InvalidRequestReferenceID = 4000081,

    /// The salable duration is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidsalableduration)
    InvalidSalableDuration = 4000117,

    /// The targeted salable isn't configured as a generic salable.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidsalable)
    InvalidSalable = 4000116,

    /// The signature is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidsignature)
    InvalidSignature = 4000174,

    /// The SKU was invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidsku)
    InvalidSKU = 4000122,

    /// The storefront value is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidstorefront)
    InvalidStorefront = 4000028,

    /// The targetProductID value is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidtargetproductid)
    InvalidTargetProductID = 4000167,

    /// The taxCode is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidtaxproductcode)
    InvalidTaxProductCode = 4000127,

    /// The transactionId is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidtransactionid)
    InvalidTransactionId = 4000006,

    /// The number of items in subscription exceeds the limit.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/itemlimitexceeded)
    ItemLimitExceeded = 4000179,

    /// The payload is malformed.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/malformedpayload)
    MalformedPayload = 4000173,

    /// The request contains a billing period that doesn't align with the subscription's billing cycle.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/misalignedbillingcycle)
    MisalignedBillingCycle = 4000147,

    /// The storefronts mismatch.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/mismatchedstorefront)
    MismatchedStorefront = 4000133,

    /// Pricing isn't configured for the storefront.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/missingpricingconfigforstorefront)
    MissingPricingConfigForStorefront = 4000134,

    /// All items must be updated on a period change.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/missingupdateditemswithperiodchange)
    MissingUpdatedItemsWithPeriodChange = 4000140,

    /// More items were provided than allowed.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/moreitemsthanallowed)
    MoreItemsThanAllowed = 4000136,

    /// More offers were provided than allowed.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/moreoffersthanallowed)
    MoreOffersThanAllowed = 4000137,

    /// Multiple operations on a single SKU isn't allowed.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/multipleoperationsonsinglesku)
    MultipleOperationsOnSingleSKU = 4000143,

    /// Prorated price and offer price are mutually exclusive.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/multipleprices)
    MultiplePrices = 4000150,

    /// The price field must contain a positive number.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/negativeprice)
    NegativePrice = 4000086,

    /// Exceeds the maximum length of the price field.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/negativeproratedprice)
    NegativeProratedPrice = 4000091,

    /// The refundAmount must be a positive number.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/negativerefundamount)
    NegativeRefundAmount = 4000154,

    /// The required field, advancedCommerceData, was null.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nulladvancedcommercedata)
    NullAdvancedCommerceData = 4000171,

    /// The required field, currency, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullcurrency)
    NullCurrency = 4000098,

    /// The required field, currentSKU, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullcurrentsku)
    NullCurrentSKU = 4000169,

    /// The required field, description, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nulldescription)
    NullDescription = 4000107,

    /// The required field, descriptors, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nulldescriptors)
    NullDescriptors = 4000103,

    /// The required field, displayName, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nulldisplayname)
    NullDisplayName = 4000106,

    /// The required field, effective, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nulleffective)
    NullEffective = 4000111,

    /// The required field, item, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullitem)
    NullItem = 4000102,

    /// The required field, items, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullitems)
    NullItems = 4000101,

    /// The required field, SKU in changeItems, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullnewsku)
    NullNewSKU = 4000112,

    /// The required field, offer period, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullofferperiod)
    NullOfferPeriod = 4000092,

    /// The required field, periodCount, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullperiodcount)
    NullPeriodCount = 4000093,

    /// The required field, period, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullperiod)
    NullPeriod = 4000104,

    /// The required field, price, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullprice)
    NullPrice = 4000109,

    /// The required field, reason, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullreason)
    NullReason = 4000095,

    /// The refundAmount value is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullrefundamount)
    NullRefundAmount = 4000153,

    /// The required field, refundReason, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullrefundreason)
    NullRefundReason = 4000156,

    /// The required field, refundRiskingPreference, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullrefundrisking)
    NullRefundRisking = 4000159,

    /// The required field, refundType, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullrefundtype)
    NullRefundType = 4000157,

    /// The required field, requestInfo, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullrequestinfo)
    NullRequestInfo = 4000079,

    /// The required field, requestReferenceId, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullrequestreferenceid)
    NullRequestReferenceID = 4000080,

    /// The required field, retainBillingCycle, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullretainbillingcycle)
    NullRetainBillingCycle = 4000110,

    /// The required field, SKU, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullsku)
    NullSKU = 4000105,

    /// The required field, storefront, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullstorefront)
    NullStorefront = 4000100,

    /// The required field, targetProductID, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nulltargetproductid)
    NullTargetProductID = 4000166,

    /// The required field, taxCode, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nulltaxcode)
    NullTaxCode = 4000099,

    /// The required field, transactionId, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nulltransactionid)
    NullTransactionId = 4000085,

    /// The required field, version, is missing.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/nullversion)
    NullVersion = 4000083,

    /// An existing offer prevents changes to the item mid-cycle.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/offerpreventsitemmidcyclechange)
    OfferPreventsItemMidCycleChange = 4000177,

    /// At least one type of change must be provided in a modify subscription request.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/oneitemneededinmodify)
    OneItemNeededInModify = 4000063,

    /// The operation isn't allowed.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/operationnotallowed)
    OperationNotAllowed = 4000135,

    /// If one item has a refundReason value of SIMULATE_REFUND_DECLINE, all items must have a refundReason value of SIMULATE_REFUND_DECLINE.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/partialsimulaterefunddecline)
    PartialSimulateRefundDecline = 4000184,

    /// Pending subscription changes must specify a renewalItem, and if there are no pending changes, a renewalItem cannot be specified.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/pendingchangesmismatch)
    PendingChangesMismatch = 4000180,

    /// The transaction has pending refunds.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/pendingrefund)
    PendingRefund = 4000181,

    /// A period change at next cycle conflicts with addition at the current period.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/periodchangeeffectiveconflict)
    PeriodChangeEffectiveConflict = 4000142,

    /// Period change immediately with effective later.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/periodchangeimmediatewitheffectiveatnextbillingcycle)
    PeriodChangeImmediateWithEffectiveAtNextBillingCycle = 4000149,

    /// Period count must be a positive number.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/periodcountnotpositive)
    PeriodCountNotPositive = 4000094,

    /// Period reset conflicts with retaining billing cycle.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/periodresetwithretainbillingcycle)
    PeriodResetWithRetainBillingCycle = 4000141,

    /// Changing the price isn't supported as part of a modify items request.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/pricechangenotsupportedthroughmodifyitems)
    PriceChangeNotSupportedThroughModifyItems = 4000178,

    /// Provided SKU is already owned.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/productalreadyexists)
    ProductAlreadyExists = 4000114,

    /// The product isn't eligible for the requested operation.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/productnoteligible)
    ProductNotEligible = 4030023,

    /// Product not found.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/productnotfound)
    ProductNotFound = 4040016,

    /// The customer doesn't own the product.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/productnotowned)
    ProductNotOwned = 4030013,

    /// Only requests against the latest transaction can have a PRORATED refundType value.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/proratedonlylatesttransaction)
    ProratedOnlyLatestTransaction = 4000182,

    /// Rate limit exceeded.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/ratelimitexceeded)
    RateLimitExceeded = 4290000,

    /// Can't provide the refund amount because the refundType isn't CUSTOM.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/refundamountwithoutcustom)
    RefundAmountWithoutCustom = 4000155,

    /// The active subscription must contain at least one item and cannot be completely empty.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/removalallnotallowed)
    RemovalAllNotAllowed = 4000168,

    /// A product in removeItems wasn't found for the given subscription.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/removeitemnotfound)
    RemoveItemNotFound = 4000145,

    /// The removeItems object was present without addItems or changeItems.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/removeitemswithoutaddorchangeitems)
    RemoveItemsWithoutAddOrChangeItems = 4000144,

    /// The requestReferenceId was repeated.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/repeatedrequestreferenceid)
    RepeatedRequestReferenceId = 4000097,

    /// Only active subscriptions are revocable.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/revokeoninactivesubscription)
    RevokeOnInactiveSubscription = 4000186,

    /// The type SIMULATE_REFUND_DECLINE is only valid in Sandbox.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/simulaterefunddeclineonlyinsandbox)
    SimulateRefundDeclineOnlyInSandbox = 4000158,

    /// Exceeds the maximum length of the SKU field.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/skulengthexceeded)
    SKULengthExceeded = 4000087,

    /// The storefront changed.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/storefrontchange)
    StorefrontChange = 4030022,

    /// The subscription is already active, and cannot be reactivated or renewed at this time.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionalreadyactive)
    SubscriptionAlreadyActive = 4030011,

    /// The subscription already exists.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionalreadyexists)
    SubscriptionAlreadyExists = 4030009,

    /// The subscription was already migrated.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionalreadymigrated)
    SubscriptionAlreadyMigrated = 4000176,

    /// The subscription doesn't exist.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/subscriptiondoesnotexist)
    SubscriptionDoesNotExist = 4030008,

    /// The subscription isn't eligible for the requested changes.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionnoteligible)
    SubscriptionNotEligible = 4030010,

    /// Transaction id not found.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/transactionidnotfound)
    TransactionIdNotFound = 4040010,

    /// The transaction is not refundable.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/transactionnotrefundable)
    TransactionNotRefundable = 4030024,

    /// The transaction can't be refunded; customer can contact Apple Support for assistance.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/transactioncannotberefundedcontactsupport)
    TransactionCannotBeRefundedContactSupport = 4030025,

    /// Unauthorized.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/unauthorized)
    Unauthorized = 4010000,

    /// The value of version is invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/unexpectedversion)
    UnexpectedVersion = 4000084,
}

impl APIServiceErrorCode for APIErrorCode {
    fn code(&self) -> i64 {
        *self as i64
    }
}

impl APIErrorCode {
    pub fn message(&self) -> &'static str {
        match self {
            APIErrorCode::AlreadyRefunded => "The transaction was already refunded.",
            APIErrorCode::AtLeastOneItem => "When included, provide at least one item in items.",
            APIErrorCode::AtLeastOneOfDisplayNameOrDescription => "Provide either the displayName or a description.",
            APIErrorCode::BillingCycleResetWithEffectiveLater => "Bill cycle reset with effective later.",
            APIErrorCode::ChangeItemNotFound => "The targeted item in changeItems wasn't found.",
            APIErrorCode::DescriptionLengthExceeded => "Exceeds the maximum length of the description field.",
            APIErrorCode::DisplayNameLengthExceeded => "Exceeds the maximum length of the displayName field.",
            APIErrorCode::EmptyAddChangeItems => "The addItems and changeItems entries cannot be empty.",
            APIErrorCode::GeneralInternal => "An unknown error occurred.",
            APIErrorCode::GeneralInternalRetryable => "An unknown error occurred. Please try again.",
            APIErrorCode::InactiveACASub => "The subscription is not active.",
            APIErrorCode::InsufficientFunds => "Insufficient funds for refund.",
            APIErrorCode::InvalidAmount => "The amount is invalid.",
            APIErrorCode::InvalidAppAccountToken => "The appAccountToken field must contain a valid UUID or an empty string.",
            APIErrorCode::InvalidChangeReason => "The change reason is invalid.",
            APIErrorCode::InvalidConsistencyToken => "The consistencyToken value is invalid.",
            APIErrorCode::InvalidCurrency => "The currency value is invalid.",
            APIErrorCode::InvalidDescription => "The description is invalid.",
            APIErrorCode::InvalidDisplayName => "The displayName is invalid.",
            APIErrorCode::InvalidOfferPeriodCount => "The offer periodCount is invalid.",
            APIErrorCode::InvalidOfferPeriod => "The offer period is invalid.",
            APIErrorCode::InvalidOfferPrice => "The subscription offer price is higher than the regular subscription price.",
            APIErrorCode::InvalidOfferReason => "The offer reason is invalid.",
            APIErrorCode::InvalidOperation => "The operation is invalid.",
            APIErrorCode::InvalidPreviousSubscription => "The previous subscription targeted is invalid.",
            APIErrorCode::InvalidPreviousTransactionID => "Previous original transaction id is invalid.",
            APIErrorCode::InvalidProductChanges => "Product changes are invalid.",
            APIErrorCode::InvalidProduct => "The requested product to change doesn't exist.",
            APIErrorCode::InvalidProratedPrice => "The prorated price was invalid.",
            APIErrorCode::InvalidRefundReason => "The refundReason is invalid.",
            APIErrorCode::InvalidRefundType => "The refundType is invalid.",
            APIErrorCode::InvalidRenewalPeriod => "The renewal period is invalid.",
            APIErrorCode::InvalidRenewalPrice => "The renewal price is invalid.",
            APIErrorCode::InvalidRequestReferenceID => "The requestReferenceId value is invalid.",
            APIErrorCode::InvalidSalableDuration => "The salable duration is invalid.",
            APIErrorCode::InvalidSalable => "The targeted salable isn't configured as a generic salable.",
            APIErrorCode::InvalidSignature => "The signature is invalid.",
            APIErrorCode::InvalidSKU => "The SKU was invalid.",
            APIErrorCode::InvalidStorefront => "The storefront value is invalid.",
            APIErrorCode::InvalidTargetProductID => "The targetProductID value is invalid.",
            APIErrorCode::InvalidTaxProductCode => "The taxCode is invalid.",
            APIErrorCode::InvalidTransactionId => "The transactionId is invalid.",
            APIErrorCode::ItemLimitExceeded => "The number of items in subscription exceeds the limit.",
            APIErrorCode::MalformedPayload => "The payload is malformed.",
            APIErrorCode::MisalignedBillingCycle => "The request contains a billing period that doesn't align with the subscription's billing cycle.",
            APIErrorCode::MismatchedStorefront => "The storefronts mismatch.",
            APIErrorCode::MissingPricingConfigForStorefront => "Pricing isn't configured for the storefront.",
            APIErrorCode::MissingUpdatedItemsWithPeriodChange => "All items must be updated on a period change.",
            APIErrorCode::MoreItemsThanAllowed => "More items were provided than allowed.",
            APIErrorCode::MoreOffersThanAllowed => "More offers were provided than allowed.",
            APIErrorCode::MultipleOperationsOnSingleSKU => "Multiple operations on a single SKU isn't allowed.",
            APIErrorCode::MultiplePrices => "Prorated price and offer price are mutually exclusive.",
            APIErrorCode::NegativePrice => "The price field must contain a positive number.",
            APIErrorCode::NegativeProratedPrice => "Exceeds the maximum length of the price field.",
            APIErrorCode::NegativeRefundAmount => "The refundAmount must be a positive number.",
            APIErrorCode::NullAdvancedCommerceData => "The required field, advancedCommerceData, was null.",
            APIErrorCode::NullCurrency => "The required field, currency, is missing.",
            APIErrorCode::NullCurrentSKU => "The required field, currentSKU, is missing.",
            APIErrorCode::NullDescription => "The required field, description, is missing.",
            APIErrorCode::NullDescriptors => "The required field, descriptors, is missing.",
            APIErrorCode::NullDisplayName => "The required field, displayName, is missing.",
            APIErrorCode::NullEffective => "The required field, effective, is missing.",
            APIErrorCode::NullItem => "The required field, item, is missing.",
            APIErrorCode::NullItems => "The required field, items, is missing.",
            APIErrorCode::NullNewSKU => "The required field, SKU in changeItems, is missing.",
            APIErrorCode::NullOfferPeriod => "The required field, offer period, is missing.",
            APIErrorCode::NullPeriodCount => "The required field, periodCount, is missing.",
            APIErrorCode::NullPeriod => "The required field, period, is missing.",
            APIErrorCode::NullPrice => "The required field, price, is missing.",
            APIErrorCode::NullReason => "The required field, reason, is missing.",
            APIErrorCode::NullRefundAmount => "The refundAmount value is invalid.",
            APIErrorCode::NullRefundReason => "The required field, refundReason, is missing.",
            APIErrorCode::NullRefundRisking => "The required field, refundRiskingPreference, is missing.",
            APIErrorCode::NullRefundType => "The required field, refundType, is missing.",
            APIErrorCode::NullRequestInfo => "The required field, requestInfo, is missing.",
            APIErrorCode::NullRequestReferenceID => "The required field, requestReferenceId, is missing.",
            APIErrorCode::NullRetainBillingCycle => "The required field, retainBillingCycle, is missing.",
            APIErrorCode::NullSKU => "The required field, SKU, is missing.",
            APIErrorCode::NullStorefront => "The required field, storefront, is missing.",
            APIErrorCode::NullTargetProductID => "The required field, targetProductID, is missing.",
            APIErrorCode::NullTaxCode => "The required field, taxCode, is missing.",
            APIErrorCode::NullTransactionId => "The required field, transactionId, is missing.",
            APIErrorCode::NullVersion => "The required field, version, is missing.",
            APIErrorCode::OfferPreventsItemMidCycleChange => "An existing offer prevents changes to the item mid-cycle.",
            APIErrorCode::OneItemNeededInModify => "At least one type of change must be provided in a modify subscription request.",
            APIErrorCode::OperationNotAllowed => "The operation isn't allowed.",
            APIErrorCode::PartialSimulateRefundDecline => "If one item has a refundReason value of SIMULATE_REFUND_DECLINE, all items must have a refundReason value of SIMULATE_REFUND_DECLINE.",
            APIErrorCode::PendingChangesMismatch => "Pending subscription changes must specify a renewalItem, and if there are no pending changes, a renewalItem cannot be specified.",
            APIErrorCode::PendingRefund => "The transaction has pending refunds.",
            APIErrorCode::PeriodChangeEffectiveConflict => "A period change at next cycle conflicts with addition at the current period.",
            APIErrorCode::PeriodChangeImmediateWithEffectiveAtNextBillingCycle => "Period change immediately with effective later.",
            APIErrorCode::PeriodCountNotPositive => "Period count must be a positive number.",
            APIErrorCode::PeriodResetWithRetainBillingCycle => "Period reset conflicts with retaining billing cycle.",
            APIErrorCode::PriceChangeNotSupportedThroughModifyItems => "Changing the price isn't supported as part of a modify items request.",
            APIErrorCode::ProductAlreadyExists => "Provided SKU is already owned.",
            APIErrorCode::ProductNotEligible => "The product isn't eligible for the requested operation.",
            APIErrorCode::ProductNotFound => "Product not found.",
            APIErrorCode::ProductNotOwned => "The customer doesn't own the product.",
            APIErrorCode::ProratedOnlyLatestTransaction => "Only requests against the latest transaction can have a PRORATED refundType value.",
            APIErrorCode::RateLimitExceeded => "Rate limit exceeded.",
            APIErrorCode::RefundAmountWithoutCustom => "Can't provide the refund amount because the refundType isn't CUSTOM.",
            APIErrorCode::RemovalAllNotAllowed => "The active subscription must contain at least one item and cannot be completely empty.",
            APIErrorCode::RemoveItemNotFound => "A product in removeItems wasn't found for the given subscription.",
            APIErrorCode::RemoveItemsWithoutAddOrChangeItems => "The removeItems object was present without addItems or changeItems.",
            APIErrorCode::RepeatedRequestReferenceId => "The requestReferenceId was repeated.",
            APIErrorCode::RevokeOnInactiveSubscription => "Only active subscriptions are revocable.",
            APIErrorCode::SimulateRefundDeclineOnlyInSandbox => "The type SIMULATE_REFUND_DECLINE is only valid in Sandbox.",
            APIErrorCode::SKULengthExceeded => "Exceeds the maximum length of the SKU field.",
            APIErrorCode::StorefrontChange => "The storefront changed.",
            APIErrorCode::SubscriptionAlreadyActive => "The subscription is already active, and cannot be reactivated or renewed at this time.",
            APIErrorCode::SubscriptionAlreadyExists => "The subscription already exists.",
            APIErrorCode::SubscriptionAlreadyMigrated => "The subscription was already migrated.",
            APIErrorCode::SubscriptionDoesNotExist => "The subscription doesn't exist.",
            APIErrorCode::SubscriptionNotEligible => "The subscription isn't eligible for the requested changes.",
            APIErrorCode::TransactionIdNotFound => "Transaction id not found.",
            APIErrorCode::TransactionNotRefundable => "The transaction is not refundable.",
            APIErrorCode::TransactionCannotBeRefundedContactSupport => "The transaction can't be refunded; customer can contact Apple Support for assistance.",
            APIErrorCode::Unauthorized => "Unauthorized.",
            APIErrorCode::UnexpectedVersion => "The value of version is invalid.",
        }
    }
}