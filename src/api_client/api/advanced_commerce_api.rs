pub mod api_error_code;

use http::Method;
use crate::api_client::api::advanced_commerce_api::api_error_code::APIErrorCode;
use crate::api_client::api_client::APIClient;
use crate::api_client::error::APIServiceError;
use crate::api_client::transport::Transport;
use crate::primitives::advanced_commerce::subscription_cancel_request::SubscriptionCancelRequest;
use crate::primitives::advanced_commerce::subscription_cancel_response::SubscriptionCancelResponse;
use crate::primitives::advanced_commerce::subscription_revoke_request::SubscriptionRevokeRequest;
use crate::primitives::advanced_commerce::subscription_revoke_response::SubscriptionRevokeResponse;
use crate::primitives::advanced_commerce::request_refund_request::RequestRefundRequest;
use crate::primitives::advanced_commerce::request_refund_response::RequestRefundResponse;
use crate::primitives::advanced_commerce::subscription_change_metadata_request::SubscriptionChangeMetadataRequest;
use crate::primitives::advanced_commerce::subscription_change_metadata_response::SubscriptionChangeMetadataResponse;
use crate::primitives::advanced_commerce::subscription_migrate_request::SubscriptionMigrateRequest;
use crate::primitives::advanced_commerce::subscription_migrate_response::SubscriptionMigrateResponse;
use crate::primitives::advanced_commerce::subscription_price_change_request::SubscriptionPriceChangeRequest;
use crate::primitives::advanced_commerce::subscription_price_change_response::SubscriptionPriceChangeResponse;

pub struct AdvancedCommerceAPI;
pub type AdvancedCommerceAPIClient<T> = APIClient<T, AdvancedCommerceAPI, APIErrorCode>;
pub type APIError = APIServiceError<APIErrorCode>;

impl<T: Transport> AdvancedCommerceAPIClient<T> {
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
        subscription_cancel_request: &SubscriptionCancelRequest,
    ) -> Result<SubscriptionCancelResponse, APIError> {
        let path = format!("/advancedCommerce/v1/subscription/cancel/{}", transaction_id);
        let req = self.build_request(
            path.as_str(),
            Method::POST,
            Some(subscription_cancel_request),
        )?;
        self.make_request_with_response_body(req).await
    }

    /// Update the SKU, display name, and description associated with a subscription,
    /// without affecting the subscription's billing or its service.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/change-subscription-metadata)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier of the auto-renewable subscription to get changes to its metadata.
    ///                       Use the subscription's original transaction ID or any subsequent transaction ID
    ///                       of a transaction related to the subscription.
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
        subscription_change_metadata_request: &SubscriptionChangeMetadataRequest,
    ) -> Result<SubscriptionChangeMetadataResponse, APIError> {
        let path = format!("/advancedCommerce/v1/subscription/changeMetadata/{}", transaction_id);
        let req = self.build_request(
            path.as_str(),
            Method::POST,
            Some(subscription_change_metadata_request),
        )?;
        self.make_request_with_response_body(req).await
    }

    /// Increase or decrease the price of an auto-renewable subscription, a bundle,
    /// or individual items within a subscription at the next renewal.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/change-subscription-price)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - A transaction identifier of the auto-renewable subscription that is subject to the price change.
    ///                       Use the subscription's original transaction ID or any subsequent transaction ID
    ///                       of a transaction related to the subscription.
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
        subscription_price_change_request: &SubscriptionPriceChangeRequest,
    ) -> Result<SubscriptionPriceChangeResponse, APIError> {
        let path = format!("/advancedCommerce/v1/subscription/changePrice/{}", transaction_id);
        let req = self.build_request(
            path.as_str(),
            Method::POST,
            Some(subscription_price_change_request),
        )?;
        self.make_request_with_response_body(req).await
    }

    /// Migrate a subscription that a customer purchased through In-App Purchase
    /// to a subscription you manage using the Advanced Commerce API.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/migrate-subscription-to-advanced-commerce-api)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier of the auto-renewable subscription to migrate.
    ///                       Use the subscription's original transaction ID or any subsequent transaction ID
    ///                       of a transaction related to the subscription.
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
        subscription_migrate_request: &SubscriptionMigrateRequest,
    ) -> Result<SubscriptionMigrateResponse, APIError> {
        let path = format!("/advancedCommerce/v1/subscription/migrate/{}", transaction_id);
        let req = self.build_request(
            path.as_str(),
            Method::POST,
            Some(subscription_migrate_request),
        )?;
        self.make_request_with_response_body(req).await
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
        request_refund_request: &RequestRefundRequest,
    ) -> Result<RequestRefundResponse, APIError> {
        let path = format!("/advancedCommerce/v1/transaction/requestRefund/{}", transaction_id);
        let req = self.build_request(
            path.as_str(),
            Method::POST,
            Some(request_refund_request),
        )?;
        self.make_request_with_response_body(req).await
    }

    /// Immediately cancel a customer's subscription and all the items that are included in the subscription,
    /// and request a full or prorated refund.
    ///
    /// [Documentation](https://developer.apple.com/documentation/advancedcommerceapi/revoke-subscription)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier of the auto-renewable subscription to revoke.
    ///                       Use the subscription's original transaction ID or any subsequent transaction ID
    ///                       of a transaction related to the subscription.
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
        subscription_revoke_request: &SubscriptionRevokeRequest,
    ) -> Result<SubscriptionRevokeResponse, APIError> {
        let path = format!("/advancedCommerce/v1/subscription/revoke/{}", transaction_id);
        let req = self.build_request(
            path.as_str(),
            Method::POST,
            Some(subscription_revoke_request),
        )?;
        self.make_request_with_response_body(req).await
    }
}