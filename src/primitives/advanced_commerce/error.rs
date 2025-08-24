use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub error_code: ErrorCode,
    pub error_message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum ErrorCode {
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

impl ErrorCode {
    pub fn message(&self) -> &'static str {
        match self {
            ErrorCode::AlreadyRefunded => "The transaction was already refunded.",
            ErrorCode::AtLeastOneItem => "When included, provide at least one item in items.",
            ErrorCode::AtLeastOneOfDisplayNameOrDescription => "Provide either the displayName or a description.",
            ErrorCode::BillingCycleResetWithEffectiveLater => "Bill cycle reset with effective later.",
            ErrorCode::ChangeItemNotFound => "The targeted item in changeItems wasn't found.",
            ErrorCode::DescriptionLengthExceeded => "Exceeds the maximum length of the description field.",
            ErrorCode::DisplayNameLengthExceeded => "Exceeds the maximum length of the displayName field.",
            ErrorCode::EmptyAddChangeItems => "The addItems and changeItems entries cannot be empty.",
            ErrorCode::GeneralInternal => "An unknown error occurred.",
            ErrorCode::GeneralInternalRetryable => "An unknown error occurred. Please try again.",
            ErrorCode::InactiveACASub => "The subscription is not active.",
            ErrorCode::InsufficientFunds => "Insufficient funds for refund.",
            ErrorCode::InvalidAmount => "The amount is invalid.",
            ErrorCode::InvalidAppAccountToken => "The appAccountToken field must contain a valid UUID or an empty string.",
            ErrorCode::InvalidChangeReason => "The change reason is invalid.",
            ErrorCode::InvalidConsistencyToken => "The consistencyToken value is invalid.",
            ErrorCode::InvalidCurrency => "The currency value is invalid.",
            ErrorCode::InvalidDescription => "The description is invalid.",
            ErrorCode::InvalidDisplayName => "The displayName is invalid.",
            ErrorCode::InvalidOfferPeriodCount => "The offer periodCount is invalid.",
            ErrorCode::InvalidOfferPeriod => "The offer period is invalid.",
            ErrorCode::InvalidOfferPrice => "The subscription offer price is higher than the regular subscription price.",
            ErrorCode::InvalidOfferReason => "The offer reason is invalid.",
            ErrorCode::InvalidOperation => "The operation is invalid.",
            ErrorCode::InvalidPreviousSubscription => "The previous subscription targeted is invalid.",
            ErrorCode::InvalidPreviousTransactionID => "Previous original transaction id is invalid.",
            ErrorCode::InvalidProductChanges => "Product changes are invalid.",
            ErrorCode::InvalidProduct => "The requested product to change doesn't exist.",
            ErrorCode::InvalidProratedPrice => "The prorated price was invalid.",
            ErrorCode::InvalidRefundReason => "The refundReason is invalid.",
            ErrorCode::InvalidRefundType => "The refundType is invalid.",
            ErrorCode::InvalidRenewalPeriod => "The renewal period is invalid.",
            ErrorCode::InvalidRenewalPrice => "The renewal price is invalid.",
            ErrorCode::InvalidRequestReferenceID => "The requestReferenceId value is invalid.",
            ErrorCode::InvalidSalableDuration => "The salable duration is invalid.",
            ErrorCode::InvalidSalable => "The targeted salable isn't configured as a generic salable.",
            ErrorCode::InvalidSignature => "The signature is invalid.",
            ErrorCode::InvalidSKU => "The SKU was invalid.",
            ErrorCode::InvalidStorefront => "The storefront value is invalid.",
            ErrorCode::InvalidTargetProductID => "The targetProductID value is invalid.",
            ErrorCode::InvalidTaxProductCode => "The taxCode is invalid.",
            ErrorCode::InvalidTransactionId => "The transactionId is invalid.",
            ErrorCode::ItemLimitExceeded => "The number of items in subscription exceeds the limit.",
            ErrorCode::MalformedPayload => "The payload is malformed.",
            ErrorCode::MisalignedBillingCycle => "The request contains a billing period that doesn't align with the subscription's billing cycle.",
            ErrorCode::MismatchedStorefront => "The storefronts mismatch.",
            ErrorCode::MissingPricingConfigForStorefront => "Pricing isn't configured for the storefront.",
            ErrorCode::MissingUpdatedItemsWithPeriodChange => "All items must be updated on a period change.",
            ErrorCode::MoreItemsThanAllowed => "More items were provided than allowed.",
            ErrorCode::MoreOffersThanAllowed => "More offers were provided than allowed.",
            ErrorCode::MultipleOperationsOnSingleSKU => "Multiple operations on a single SKU isn't allowed.",
            ErrorCode::MultiplePrices => "Prorated price and offer price are mutually exclusive.",
            ErrorCode::NegativePrice => "The price field must contain a positive number.",
            ErrorCode::NegativeProratedPrice => "Exceeds the maximum length of the price field.",
            ErrorCode::NegativeRefundAmount => "The refundAmount must be a positive number.",
            ErrorCode::NullAdvancedCommerceData => "The required field, advancedCommerceData, was null.",
            ErrorCode::NullCurrency => "The required field, currency, is missing.",
            ErrorCode::NullCurrentSKU => "The required field, currentSKU, is missing.",
            ErrorCode::NullDescription => "The required field, description, is missing.",
            ErrorCode::NullDescriptors => "The required field, descriptors, is missing.",
            ErrorCode::NullDisplayName => "The required field, displayName, is missing.",
            ErrorCode::NullEffective => "The required field, effective, is missing.",
            ErrorCode::NullItem => "The required field, item, is missing.",
            ErrorCode::NullItems => "The required field, items, is missing.",
            ErrorCode::NullNewSKU => "The required field, SKU in changeItems, is missing.",
            ErrorCode::NullOfferPeriod => "The required field, offer period, is missing.",
            ErrorCode::NullPeriodCount => "The required field, periodCount, is missing.",
            ErrorCode::NullPeriod => "The required field, period, is missing.",
            ErrorCode::NullPrice => "The required field, price, is missing.",
            ErrorCode::NullReason => "The required field, reason, is missing.",
            ErrorCode::NullRefundAmount => "The refundAmount value is invalid.",
            ErrorCode::NullRefundReason => "The required field, refundReason, is missing.",
            ErrorCode::NullRefundRisking => "The required field, refundRiskingPreference, is missing.",
            ErrorCode::NullRefundType => "The required field, refundType, is missing.",
            ErrorCode::NullRequestInfo => "The required field, requestInfo, is missing.",
            ErrorCode::NullRequestReferenceID => "The required field, requestReferenceId, is missing.",
            ErrorCode::NullRetainBillingCycle => "The required field, retainBillingCycle, is missing.",
            ErrorCode::NullSKU => "The required field, SKU, is missing.",
            ErrorCode::NullStorefront => "The required field, storefront, is missing.",
            ErrorCode::NullTargetProductID => "The required field, targetProductID, is missing.",
            ErrorCode::NullTaxCode => "The required field, taxCode, is missing.",
            ErrorCode::NullTransactionId => "The required field, transactionId, is missing.",
            ErrorCode::NullVersion => "The required field, version, is missing.",
            ErrorCode::OfferPreventsItemMidCycleChange => "An existing offer prevents changes to the item mid-cycle.",
            ErrorCode::OneItemNeededInModify => "At least one type of change must be provided in a modify subscription request.",
            ErrorCode::OperationNotAllowed => "The operation isn't allowed.",
            ErrorCode::PartialSimulateRefundDecline => "If one item has a refundReason value of SIMULATE_REFUND_DECLINE, all items must have a refundReason value of SIMULATE_REFUND_DECLINE.",
            ErrorCode::PendingChangesMismatch => "Pending subscription changes must specify a renewalItem, and if there are no pending changes, a renewalItem cannot be specified.",
            ErrorCode::PendingRefund => "The transaction has pending refunds.",
            ErrorCode::PeriodChangeEffectiveConflict => "A period change at next cycle conflicts with addition at the current period.",
            ErrorCode::PeriodChangeImmediateWithEffectiveAtNextBillingCycle => "Period change immediately with effective later.",
            ErrorCode::PeriodCountNotPositive => "Period count must be a positive number.",
            ErrorCode::PeriodResetWithRetainBillingCycle => "Period reset conflicts with retaining billing cycle.",
            ErrorCode::PriceChangeNotSupportedThroughModifyItems => "Changing the price isn't supported as part of a modify items request.",
            ErrorCode::ProductAlreadyExists => "Provided SKU is already owned.",
            ErrorCode::ProductNotEligible => "The product isn't eligible for the requested operation.",
            ErrorCode::ProductNotFound => "Product not found.",
            ErrorCode::ProductNotOwned => "The customer doesn't own the product.",
            ErrorCode::ProratedOnlyLatestTransaction => "Only requests against the latest transaction can have a PRORATED refundType value.",
            ErrorCode::RateLimitExceeded => "Rate limit exceeded.",
            ErrorCode::RefundAmountWithoutCustom => "Can't provide the refund amount because the refundType isn't CUSTOM.",
            ErrorCode::RemovalAllNotAllowed => "The active subscription must contain at least one item and cannot be completely empty.",
            ErrorCode::RemoveItemNotFound => "A product in removeItems wasn't found for the given subscription.",
            ErrorCode::RemoveItemsWithoutAddOrChangeItems => "The removeItems object was present without addItems or changeItems.",
            ErrorCode::RepeatedRequestReferenceId => "The requestReferenceId was repeated.",
            ErrorCode::RevokeOnInactiveSubscription => "Only active subscriptions are revocable.",
            ErrorCode::SimulateRefundDeclineOnlyInSandbox => "The type SIMULATE_REFUND_DECLINE is only valid in Sandbox.",
            ErrorCode::SKULengthExceeded => "Exceeds the maximum length of the SKU field.",
            ErrorCode::StorefrontChange => "The storefront changed.",
            ErrorCode::SubscriptionAlreadyActive => "The subscription is already active, and cannot be reactivated or renewed at this time.",
            ErrorCode::SubscriptionAlreadyExists => "The subscription already exists.",
            ErrorCode::SubscriptionAlreadyMigrated => "The subscription was already migrated.",
            ErrorCode::SubscriptionDoesNotExist => "The subscription doesn't exist.",
            ErrorCode::SubscriptionNotEligible => "The subscription isn't eligible for the requested changes.",
            ErrorCode::TransactionIdNotFound => "Transaction id not found.",
            ErrorCode::TransactionNotRefundable => "The transaction is not refundable.",
            ErrorCode::TransactionCannotBeRefundedContactSupport => "The transaction can't be refunded; customer can contact Apple Support for assistance.",
            ErrorCode::Unauthorized => "Unauthorized.",
            ErrorCode::UnexpectedVersion => "The value of version is invalid.",
        }
    }
}