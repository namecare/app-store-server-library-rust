use std::fmt;

use http::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api_client::api_client::ApiClient;
use crate::api_client::error::{ApiClientError, ConfigurationError};
use crate::api_client::transport::Transport;
use crate::models::advanced_commerce_request_refund_request::AdvancedCommerceRequestRefundRequest;
use crate::models::advanced_commerce_request_refund_response::AdvancedCommerceRequestRefundResponse;
use crate::models::advanced_commerce_subscription_cancel_request::AdvancedCommerceSubscriptionCancelRequest;
use crate::models::advanced_commerce_subscription_cancel_response::AdvancedCommerceSubscriptionCancelResponse;
use crate::models::advanced_commerce_subscription_change_metadata_request::AdvancedCommerceSubscriptionChangeMetadataRequest;
use crate::models::advanced_commerce_subscription_change_metadata_response::AdvancedCommerceSubscriptionChangeMetadataResponse;
use crate::models::advanced_commerce_subscription_migrate_request::AdvancedCommerceSubscriptionMigrateRequest;
use crate::models::advanced_commerce_subscription_migrate_response::AdvancedCommerceSubscriptionMigrateResponse;
use crate::models::advanced_commerce_subscription_price_change_request::AdvancedCommerceSubscriptionPriceChangeRequest;
use crate::models::advanced_commerce_subscription_price_change_response::AdvancedCommerceSubscriptionPriceChangeResponse;
use crate::models::advanced_commerce_subscription_revoke_request::AdvancedCommerceSubscriptionRevokeRequest;
use crate::models::advanced_commerce_subscription_revoke_response::AdvancedCommerceSubscriptionRevokeResponse;
use crate::models::app_store_environment::Environment;

/// The error returned by [`AdvancedCommerceApiClient`].
#[derive(Debug, Clone)]
pub struct AdvancedCommerceApiClientError {
    inner: ApiClientError,
    api_error: ApiErrorCode,
}

impl AdvancedCommerceApiClientError {
    pub fn status(&self) -> u16 {
        self.inner.status()
    }

    pub fn raw_code(&self) -> Option<i64> {
        self.inner.raw_code()
    }

    pub fn message(&self) -> Option<&str> {
        self.inner.message()
    }

    pub fn api_error(&self) -> ApiErrorCode {
        self.api_error
    }

    pub fn inner(&self) -> &ApiClientError {
        &self.inner
    }
}

impl std::error::Error for AdvancedCommerceApiClientError {}

impl fmt::Display for AdvancedCommerceApiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, API Error: {:?}", self.inner, self.api_error)
    }
}

impl From<ApiClientError> for AdvancedCommerceApiClientError {
    fn from(inner: ApiClientError) -> Self {
        let api_error = inner
            .raw_code()
            .map(ApiErrorCode::from_code)
            .unwrap_or(ApiErrorCode::Unknown);
        Self { inner, api_error }
    }
}

pub struct AdvancedCommerceApiClient<T: Transport> {
    inner: ApiClient<T>,
}

impl<T: Transport> AdvancedCommerceApiClient<T> {
    /// Creates a new Advanced Commerce API client.
    ///
    /// # Arguments
    ///
    /// * `signing_key` - The private key used for signing JWT tokens
    /// * `key_id` - The key identifier from App Store Connect
    /// * `issuer_id` - The issuer ID from App Store Connect
    /// * `bundle_id` - The app's bundle identifier
    /// * `environment` - The environment to use (Production or Sandbox). Xcode environment is not supported for API calls.
    /// * `transport` - The HTTP transport implementation
    ///
    /// # Errors
    ///
    /// Returns an error if the Xcode environment is provided, as it's only for local receipt validation.
    pub fn new(
        signing_key: Vec<u8>,
        key_id: &str,
        issuer_id: &str,
        bundle_id: &str,
        environment: Environment,
        transport: T,
    ) -> Result<Self, ConfigurationError> {
        Ok(Self {
            inner: ApiClient::new(
                signing_key,
                key_id,
                issuer_id,
                bundle_id,
                environment,
                transport,
            )?,
        })
    }

    async fn request<Res, B>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
    ) -> Result<Res, AdvancedCommerceApiClientError>
    where
        Res: DeserializeOwned,
        B: Serialize,
    {
        let req = self
            .inner
            .build_request(path, method, body)?;
        self.inner
            .make_request_with_response_body(req)
            .await
            .map_err(Into::into)
    }

    /// Turn off automatic renewal to cancel a customer's auto-renewable subscription.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/cancel-a-subscription)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier of the auto-renewable subscription to cancel.
    /// * `subscription_cancel_request` - The request body that includes information about the subscription to cancel.
    ///
    /// # Returns
    ///
    /// A response that indicates the subscription was successfully cancelled.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn cancel_subscription(
        &self,
        transaction_id: &str,
        subscription_cancel_request: &AdvancedCommerceSubscriptionCancelRequest,
    ) -> Result<AdvancedCommerceSubscriptionCancelResponse, AdvancedCommerceApiClientError> {
        let path = format!(
            "/advancedCommerce/v1/subscription/cancel/{}",
            transaction_id
        );
        self.request(
            path.as_str(),
            Method::POST,
            Some(subscription_cancel_request),
        )
        .await
    }

    /// Update the SKU, display name, and description associated with a subscription,
    /// without affecting the subscription's billing or its service.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/change-subscription-metadata)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier of the auto-renewable subscription to get changes to its metadata.
    ///   Use the subscription's original transaction ID or any subsequent transaction ID
    ///   of a transaction related to the subscription.
    /// * `subscription_change_metadata_request` - The request body that contains the metadata changes.
    ///
    /// # Returns
    ///
    /// A response that indicates the metadata was successfully changed.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn change_subscription_metadata(
        &self,
        transaction_id: &str,
        subscription_change_metadata_request: &AdvancedCommerceSubscriptionChangeMetadataRequest,
    ) -> Result<AdvancedCommerceSubscriptionChangeMetadataResponse, AdvancedCommerceApiClientError> {
        let path = format!(
            "/advancedCommerce/v1/subscription/changeMetadata/{}",
            transaction_id
        );
        self.request(
            path.as_str(),
            Method::POST,
            Some(subscription_change_metadata_request),
        )
        .await
    }

    /// Increase or decrease the price of an auto-renewable subscription, a bundle,
    /// or individual items within a subscription at the next renewal.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/change-subscription-price)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - A transaction identifier of the auto-renewable subscription that is subject to the price change.
    ///   Use the subscription's original transaction ID or any subsequent transaction ID
    ///   of a transaction related to the subscription.
    /// * `subscription_price_change_request` - The request body that contains the details of the price change.
    ///
    /// # Returns
    ///
    /// A response that contains signed JWS renewal and JWS transaction information after a subscription price change request.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn change_subscription_price(
        &self,
        transaction_id: &str,
        subscription_price_change_request: &AdvancedCommerceSubscriptionPriceChangeRequest,
    ) -> Result<AdvancedCommerceSubscriptionPriceChangeResponse, AdvancedCommerceApiClientError> {
        let path = format!(
            "/advancedCommerce/v1/subscription/changePrice/{}",
            transaction_id
        );
        self.request(
            path.as_str(),
            Method::POST,
            Some(subscription_price_change_request),
        )
        .await
    }

    /// Migrate a subscription that a customer purchased through In-App Purchase
    /// to a subscription you manage using the Advanced Commerce API.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/migrate-subscription-to-advanced-commerce-api)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier of the auto-renewable subscription to migrate.
    ///   Use the subscription's original transaction ID or any subsequent transaction ID
    ///   of a transaction related to the subscription.
    /// * `subscription_migrate_request` - The request body that contains the details for the migration.
    ///
    /// # Returns
    ///
    /// A response that indicates the subscription was successfully migrated.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn migrate_subscription(
        &self,
        transaction_id: &str,
        subscription_migrate_request: &AdvancedCommerceSubscriptionMigrateRequest,
    ) -> Result<AdvancedCommerceSubscriptionMigrateResponse, AdvancedCommerceApiClientError> {
        let path = format!(
            "/advancedCommerce/v1/subscription/migrate/{}",
            transaction_id
        );
        self.request(
            path.as_str(),
            Method::POST,
            Some(subscription_migrate_request),
        )
        .await
    }

    /// Request a refund for a one-time charge or subscription transaction.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/request-transaction-refund)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier for which you request a refund.
    /// * `request_refund_request` - The request body for the refund.
    ///
    /// # Returns
    ///
    /// A response that indicates the refund request was successfully processed.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn request_transaction_refund(
        &self,
        transaction_id: &str,
        request_refund_request: &AdvancedCommerceRequestRefundRequest,
    ) -> Result<AdvancedCommerceRequestRefundResponse, AdvancedCommerceApiClientError> {
        let path = format!(
            "/advancedCommerce/v1/transaction/requestRefund/{}",
            transaction_id
        );
        self.request(path.as_str(), Method::POST, Some(request_refund_request))
            .await
    }

    /// Immediately cancel a customer's subscription and all the items that are included in the subscription,
    /// and request a full or prorated refund.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/revoke-subscription)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier of the auto-renewable subscription to revoke.
    ///   Use the subscription's original transaction ID or any subsequent transaction ID
    ///   of a transaction related to the subscription.
    /// * `subscription_revoke_request` - The request body for revoking the subscription.
    ///
    /// # Returns
    ///
    /// A response that indicates the subscription was successfully revoked.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn revoke_subscription(
        &self,
        transaction_id: &str,
        subscription_revoke_request: &AdvancedCommerceSubscriptionRevokeRequest,
    ) -> Result<AdvancedCommerceSubscriptionRevokeResponse, AdvancedCommerceApiClientError> {
        let path = format!(
            "/advancedCommerce/v1/subscription/revoke/{}",
            transaction_id
        );
        self.request(
            path.as_str(),
            Method::POST,
            Some(subscription_revoke_request),
        )
        .await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum ApiErrorCode {
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/acapriceincreaseisnotcurrentlysupportedinindiaerror)
    ACAPriceIncreaseIsNotCurrentlySupportedInIndiaError = 4000221,

    /// Prorated price should not be present for change items with type effective later.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidproratedpriceforchangeitemwitheffectivelatererror)
    InvalidProratedPriceForChangeItemWithEffectiveLaterError = 4000222,

    /// The subscription offer configuration is invalid. Free trial offers must use a period count of 1.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/freetrialoffermustuseperiodcountofoneerror)
    FreeTrialOfferMustUsePeriodCountOfOneError = 4000223,

    /// Migration isn't allowed because a price increase was already communicated to the customer.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/migrationnotallowedwhenpriceincreasecommunicatederror)
    MigrationNotAllowedWhenPriceIncreaseCommunicatedError = 4030027,

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

    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/dependentskuscannotbechainederror)
    DependentSKUsCannotBeChainedError = 4000193,

    /// Invalid request. dependentSKUs can't be shared between multiple SKUs.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/dependentskuscannotbesharederror)
    DependentSKUsCannotBeSharedError = 4000192,

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

    /// Price provided for a changed item with an offer while subject to a price increase was invalid.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidpriceforchangeiteminpriceincreaseerror)
    InvalidPriceForChangeItemInPriceIncreaseError = 4000214,

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

    /// SKU values provided in request must be a SKU that will renew.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/invalidskuprovidedmustbecurrentskusettorenewerror)
    InvalidSKUProvidedMustBeCurrentSKUSetToRenewError = 4000220,

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

    /// The same SKU can't be repeated in this request.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/itemcannotbespecifiedmultipletimeserror)
    ItemCannotBeSpecifiedMultipleTimesError = 4000194,

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

    /// AdvancedCommercePeriod reset conflicts with retaining billing cycle.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/periodresetwithretainbillingcycle)
    PeriodResetWithRetainBillingCycle = 4000141,

    /// A price change can't be issued when the price increase has already been communicated.
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/pricechangecannotbeissuedwhenalreadycommunicatederror)
    PriceChangeCannotBeIssuedWhenAlreadyCommunicatedError = 4000205,

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

    /// An unknown error
    Unknown = -1,
}

impl ApiErrorCode {
    /// Maps a raw error code returned by the Advanced Commerce API into a known
    /// `ApiErrorCode` variant, falling back to `ApiErrorCode::Unknown`.
    pub fn from_code(code: i64) -> Self {
        match code {
            4000221 => Self::ACAPriceIncreaseIsNotCurrentlySupportedInIndiaError,
            4000222 => Self::InvalidProratedPriceForChangeItemWithEffectiveLaterError,
            4000223 => Self::FreeTrialOfferMustUsePeriodCountOfOneError,
            4030027 => Self::MigrationNotAllowedWhenPriceIncreaseCommunicatedError,
            4030021 => Self::AlreadyRefunded,
            4000160 => Self::AtLeastOneItem,
            4000165 => Self::AtLeastOneOfDisplayNameOrDescription,
            4000148 => Self::BillingCycleResetWithEffectiveLater,
            4000146 => Self::ChangeItemNotFound,
            4000193 => Self::DependentSKUsCannotBeChainedError,
            4000192 => Self::DependentSKUsCannotBeSharedError,
            4000088 => Self::DescriptionLengthExceeded,
            4000089 => Self::DisplayNameLengthExceeded,
            4000139 => Self::EmptyAddChangeItems,
            5000000 => Self::GeneralInternal,
            5000001 => Self::GeneralInternalRetryable,
            4030015 => Self::InactiveACASub,
            4030020 => Self::InsufficientFunds,
            4000132 => Self::InvalidAmount,
            4000033 => Self::InvalidAppAccountToken,
            4000125 => Self::InvalidChangeReason,
            4000082 => Self::InvalidConsistencyToken,
            4000053 => Self::InvalidCurrency,
            4000119 => Self::InvalidDescription,
            4000118 => Self::InvalidDisplayName,
            4000129 => Self::InvalidOfferPeriodCount,
            4000128 => Self::InvalidOfferPeriod,
            4000152 => Self::InvalidOfferPrice,
            4000126 => Self::InvalidOfferReason,
            4000172 => Self::InvalidOperation,
            4000113 => Self::InvalidPreviousSubscription,
            4000096 => Self::InvalidPreviousTransactionID,
            4000214 => Self::InvalidPriceForChangeItemInPriceIncreaseError,
            4000115 => Self::InvalidProductChanges,
            4000121 => Self::InvalidProduct,
            4000151 => Self::InvalidProratedPrice,
            4000124 => Self::InvalidRefundReason,
            4000123 => Self::InvalidRefundType,
            4000130 => Self::InvalidRenewalPeriod,
            4000131 => Self::InvalidRenewalPrice,
            4000081 => Self::InvalidRequestReferenceID,
            4000117 => Self::InvalidSalableDuration,
            4000116 => Self::InvalidSalable,
            4000174 => Self::InvalidSignature,
            4000122 => Self::InvalidSKU,
            4000220 => Self::InvalidSKUProvidedMustBeCurrentSKUSetToRenewError,
            4000028 => Self::InvalidStorefront,
            4000167 => Self::InvalidTargetProductID,
            4000127 => Self::InvalidTaxProductCode,
            4000006 => Self::InvalidTransactionId,
            4000194 => Self::ItemCannotBeSpecifiedMultipleTimesError,
            4000179 => Self::ItemLimitExceeded,
            4000173 => Self::MalformedPayload,
            4000147 => Self::MisalignedBillingCycle,
            4000133 => Self::MismatchedStorefront,
            4000134 => Self::MissingPricingConfigForStorefront,
            4000140 => Self::MissingUpdatedItemsWithPeriodChange,
            4000136 => Self::MoreItemsThanAllowed,
            4000137 => Self::MoreOffersThanAllowed,
            4000143 => Self::MultipleOperationsOnSingleSKU,
            4000150 => Self::MultiplePrices,
            4000086 => Self::NegativePrice,
            4000091 => Self::NegativeProratedPrice,
            4000154 => Self::NegativeRefundAmount,
            4000171 => Self::NullAdvancedCommerceData,
            4000098 => Self::NullCurrency,
            4000169 => Self::NullCurrentSKU,
            4000107 => Self::NullDescription,
            4000103 => Self::NullDescriptors,
            4000106 => Self::NullDisplayName,
            4000111 => Self::NullEffective,
            4000102 => Self::NullItem,
            4000101 => Self::NullItems,
            4000112 => Self::NullNewSKU,
            4000092 => Self::NullOfferPeriod,
            4000093 => Self::NullPeriodCount,
            4000104 => Self::NullPeriod,
            4000109 => Self::NullPrice,
            4000095 => Self::NullReason,
            4000153 => Self::NullRefundAmount,
            4000156 => Self::NullRefundReason,
            4000159 => Self::NullRefundRisking,
            4000157 => Self::NullRefundType,
            4000079 => Self::NullRequestInfo,
            4000080 => Self::NullRequestReferenceID,
            4000110 => Self::NullRetainBillingCycle,
            4000105 => Self::NullSKU,
            4000100 => Self::NullStorefront,
            4000166 => Self::NullTargetProductID,
            4000099 => Self::NullTaxCode,
            4000085 => Self::NullTransactionId,
            4000083 => Self::NullVersion,
            4000177 => Self::OfferPreventsItemMidCycleChange,
            4000063 => Self::OneItemNeededInModify,
            4000135 => Self::OperationNotAllowed,
            4000184 => Self::PartialSimulateRefundDecline,
            4000180 => Self::PendingChangesMismatch,
            4000181 => Self::PendingRefund,
            4000142 => Self::PeriodChangeEffectiveConflict,
            4000149 => Self::PeriodChangeImmediateWithEffectiveAtNextBillingCycle,
            4000094 => Self::PeriodCountNotPositive,
            4000141 => Self::PeriodResetWithRetainBillingCycle,
            4000205 => Self::PriceChangeCannotBeIssuedWhenAlreadyCommunicatedError,
            4000178 => Self::PriceChangeNotSupportedThroughModifyItems,
            4000114 => Self::ProductAlreadyExists,
            4030023 => Self::ProductNotEligible,
            4040016 => Self::ProductNotFound,
            4030013 => Self::ProductNotOwned,
            4000182 => Self::ProratedOnlyLatestTransaction,
            4290000 => Self::RateLimitExceeded,
            4000155 => Self::RefundAmountWithoutCustom,
            4000168 => Self::RemovalAllNotAllowed,
            4000145 => Self::RemoveItemNotFound,
            4000144 => Self::RemoveItemsWithoutAddOrChangeItems,
            4000097 => Self::RepeatedRequestReferenceId,
            4000186 => Self::RevokeOnInactiveSubscription,
            4000158 => Self::SimulateRefundDeclineOnlyInSandbox,
            4000087 => Self::SKULengthExceeded,
            4030022 => Self::StorefrontChange,
            4030011 => Self::SubscriptionAlreadyActive,
            4030009 => Self::SubscriptionAlreadyExists,
            4000176 => Self::SubscriptionAlreadyMigrated,
            4030008 => Self::SubscriptionDoesNotExist,
            4030010 => Self::SubscriptionNotEligible,
            4040010 => Self::TransactionIdNotFound,
            4030024 => Self::TransactionNotRefundable,
            4030025 => Self::TransactionCannotBeRefundedContactSupport,
            4010000 => Self::Unauthorized,
            4000084 => Self::UnexpectedVersion,
            _ => Self::Unknown,
        }
    }
}
