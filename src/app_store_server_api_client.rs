use crate::api_client::api_client::ApiClient;
use crate::api_client::error::{ApiClientError, ConfigurationError};
use crate::api_client::transport::Transport;
use crate::models::app_store_environment::Environment;
use crate::models::app_transaction_info_response::AppTransactionInfoResponse;
use crate::models::check_test_notification_response::CheckTestNotificationResponse;
use crate::models::consumption_request::ConsumptionRequest;
use crate::models::consumption_request_v1::ConsumptionRequestV1;
use crate::models::default_configuration_request::DefaultConfigurationRequest;
use crate::models::default_configuration_response::DefaultConfigurationResponse;
use crate::models::extend_renewal_date_request::ExtendRenewalDateRequest;
use crate::models::extend_renewal_date_response::ExtendRenewalDateResponse;
use crate::models::get_image_list_response::GetImageListResponse;
use crate::models::get_message_list_response::GetMessageListResponse;
use crate::models::history_response::HistoryResponse;
use crate::models::mass_extend_renewal_date_request::MassExtendRenewalDateRequest;
use crate::models::mass_extend_renewal_date_status_response::MassExtendRenewalDateStatusResponse;
use crate::models::notification_history_request::NotificationHistoryRequest;
use crate::models::notification_history_response::NotificationHistoryResponse;
use crate::models::order_lookup_response::OrderLookupResponse;
use crate::models::performance_test_request::PerformanceTestRequest;
use crate::models::performance_test_response::PerformanceTestResponse;
use crate::models::performance_test_result_response::PerformanceTestResultResponse;
use crate::models::realtime_url_request::RealtimeUrlRequest;
use crate::models::realtime_url_response::RealtimeUrlResponse;
use crate::models::refund_history_response::RefundHistoryResponse;
use crate::models::send_test_notification_response::SendTestNotificationResponse;
use crate::models::status::Status;
use crate::models::status_response::StatusResponse;
use crate::models::transaction_history_request::TransactionHistoryRequest;
use crate::models::transaction_info_response::TransactionInfoResponse;
use crate::models::update_app_account_token_request::UpdateAppAccountTokenRequest;
use crate::models::upload_message_request_body::UploadMessageRequestBody;
use crate::models::image_size::ImageSize;
use crate::utils::percent_encode_query_value;
use http::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;
use uuid::Uuid;

/// The error returned by [`AppStoreServerApiClient`].
#[derive(Debug, Clone)]
pub struct AppStoreServerApiClientError {
    inner: ApiClientError,
    api_error: ApiErrorCode,
}

impl AppStoreServerApiClientError {
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

impl std::error::Error for AppStoreServerApiClientError {}

impl fmt::Display for AppStoreServerApiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, API Error: {:?}", self.inner, self.api_error)
    }
}

impl From<ApiClientError> for AppStoreServerApiClientError {
    fn from(inner: ApiClientError) -> Self {
        let api_error = inner.raw_code().map(ApiErrorCode::from_code).unwrap_or(ApiErrorCode::Unknown);
        Self { inner, api_error }
    }
}

pub struct AppStoreServerApiClient<T: Transport> {
    inner: ApiClient<T>,
}

impl<T: Transport> AppStoreServerApiClient<T> {
    /// Creates a new App Store Server API client.
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
            inner: ApiClient::new(signing_key, key_id, issuer_id, bundle_id, environment, transport)?,
        })
    }

    async fn request<Res, B>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
    ) -> Result<Res, AppStoreServerApiClientError>
    where
        Res: DeserializeOwned,
        B: Serialize,
    {
        let req = self.inner.build_request(path, method, body)?;
        self.inner.make_request_with_response_body(req).await.map_err(Into::into)
    }

    async fn request_no_body<B: Serialize>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
    ) -> Result<(), AppStoreServerApiClientError> {
        let req = self.inner.build_request(path, method, body)?;
        self.inner
            .make_request_without_response_body(req)
            .await
            .map_err(Into::into)
    }

    async fn request_custom_content(
        &self,
        path: &str,
        method: Method,
        body: Vec<u8>,
        content_type: &str,
    ) -> Result<(), AppStoreServerApiClientError> {
        let req = self.inner.build_request_with_custom_content(path, method, body, content_type)?;
        self.inner
            .make_request_without_response_body(req)
            .await
            .map_err(Into::into)
    }

    /// Uses a subscription's product identifier to extend the renewal date for all of its eligible active subscribers.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/extend_subscription_renewal_dates_for_all_active_subscribers)
    ///
    /// # Arguments
    ///
    /// * `mass_extend_renewal_date_request` - The request body for extending a subscription renewal date for all of its active subscribers.
    ///
    /// # Returns
    ///
    /// A response that indicates the server successfully received the subscription-renewal-date extension request.
    ///
    /// # Errors
    ///
    /// Throws an `APIException` if a response was returned indicating the request could not be processed.
    pub async fn extend_renewal_date_for_all_active_subscribers(
        &self,
        mass_extend_renewal_date_request: &MassExtendRenewalDateRequest,
    ) -> Result<MassExtendRenewalDateStatusResponse, AppStoreServerApiClientError> {
        self.request(
            "/inApps/v1/subscriptions/extend/mass",
            Method::POST,
            Some(mass_extend_renewal_date_request),
        )
        .await
    }

    /// Extends the renewal date of a customer's active subscription using the original transaction identifier.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/extend_a_subscription_renewal_date)
    ///
    /// # Arguments
    ///
    /// * `original_transaction_id` - The original transaction identifier of the subscription receiving a renewal date extension.
    /// * `extend_renewal_date_request` - The request body containing subscription-renewal-extension data.
    ///
    /// # Returns
    ///
    /// A response that indicates whether an individual renewal-date extension succeeded, and related details.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn extend_subscription_renewal_date(
        &self,
        original_transaction_id: &str,
        extend_renewal_date_request: &ExtendRenewalDateRequest,
    ) -> Result<ExtendRenewalDateResponse, AppStoreServerApiClientError> {
        let path = format!(
            "/inApps/v1/subscriptions/extend/{}",
            original_transaction_id
        );
        self.request(path.as_str(), Method::PUT, Some(extend_renewal_date_request))
            .await
    }

    /// Get the statuses for all of a customer's auto-renewable subscriptions in your app.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/get_all_subscription_statuses)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The identifier of a transaction that belongs to the customer, and which may be an original transaction identifier.
    /// * `status` - An optional filter that indicates the status of subscriptions to include in the response.
    ///
    /// # Returns
    ///
    /// A response that contains status information for all of a customer's auto-renewable subscriptions in your app.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn get_all_subscription_statuses(
        &self,
        transaction_id: &str,
        status: Option<&Vec<Status>>,
    ) -> Result<StatusResponse, AppStoreServerApiClientError> {
        let mut path = format!("/inApps/v1/subscriptions/{}", transaction_id);

        if let Some(status) = status {
            let query_params: Vec<String> = status
                .iter()
                .map(|item| format!("status={}", item.raw_value()))
                .collect();
            if !query_params.is_empty() {
                path.push('?');
                path.push_str(&query_params.join("&"));
            }
        }

        self.request::<StatusResponse, ()>(path.as_str(), Method::GET, None)
            .await
    }

    /// Get a paginated list of all of a customer's refunded in-app purchases for your app.
    ///
    /// [Apple Documentation](https://developer.apple.com/documentation/appstoreserverapi/get_refund_history)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The identifier of a transaction that belongs to the customer, and which may be an original transaction identifier.
    /// * `revision` - A token you provide to get the next set of up to 20 transactions. All responses include a revision token. Use the revision token from the previous `RefundHistoryResponse`.
    ///
    /// # Returns
    ///
    /// A result containing either the response that contains status information for all of a customer's auto-renewable subscriptions in your app, or an `APIError` if the request could not be processed.
    ///
    /// # Errors
    ///
    /// * `RefundHistoryNotFoundError` (Status Code: 4040008) - An error that indicates that the test notification token is expired or the test notification status isn’t available.
    /// * `RefundHistoryRequestNotFoundError` (Status Code: 4040009) - An error that indicates the server didn't find a subscription-renewal-date extension request for the request identifier and product identifier you provided.
    /// * `RefundHistoryServerError` (Status Code: 5000000) - An error that indicates a server error occurred during the request processing.
    ///
    pub async fn get_refund_history(
        &self,
        transaction_id: &str,
        revision: &str,
    ) -> Result<RefundHistoryResponse, AppStoreServerApiClientError> {
        let mut path = format!("/inApps/v2/refund/lookup/{}", transaction_id);
        if !revision.is_empty() {
            path.push_str(&format!("?revision={}", percent_encode_query_value(revision)));
        }
        self.request::<RefundHistoryResponse, ()>(path.as_str(), Method::GET, None)
            .await
    }

    /// Checks whether a renewal date extension request completed, and provides the final count of successful or failed extensions.
    ///
    /// [Apple Documentation](https://developer.apple.com/documentation/appstoreserverapi/get_status_of_subscription_renewal_date_extensions)
    ///
    /// # Arguments
    ///
    /// * `request_identifier` - The UUID that represents your request to the Extend Subscription Renewal Dates for All Active Subscribers endpoint.
    /// * `product_id` - The product identifier of the auto-renewable subscription that you request a renewal-date extension for.
    ///
    /// # Returns
    ///
    /// A result containing either the response that indicates the current status of a request to extend the subscription renewal date to all eligible subscribers, or an `APIError` if the request could not be processed.
    ///
    /// # Errors
    ///
    /// * `SubscriptionRenewalDateStatusNotFoundError` (Status Code: 4040009) - An error that indicates the server didn't find a subscription-renewal-date extension request for the request identifier and product identifier you provided.
    /// * `SubscriptionRenewalDateStatusServerError` (Status Code: 5000000) - An error that indicates a server error occurred during the request processing.
    ///
    pub async fn get_status_of_subscription_renewal_date_extensions(
        &self,
        request_identifier: &str,
        product_id: &str,
    ) -> Result<MassExtendRenewalDateStatusResponse, AppStoreServerApiClientError> {
        let path = format!(
            "/inApps/v1/subscriptions/extend/mass/{}/{}",
            product_id, request_identifier
        );
        self.request::<MassExtendRenewalDateStatusResponse, ()>(path.as_str(), Method::GET, None)
            .await
    }

    /// Check the status of the test App Store server notification sent to your server.
    ///
    /// [Apple Documentation](https://developer.apple.com/documentation/appstoreserverapi/get_test_notification_status)
    ///
    /// # Arguments
    ///
    /// * `test_notification_token` - The test notification token received from the Request a Test Notification endpoint.
    ///
    /// # Returns
    ///
    /// A result containing either the response that contains the contents of the test notification sent by the App Store server and the result from your server, or an `APIError` if the request could not be processed.
    ///
    /// # Errors
    ///
    /// * `TestNotificationNotFoundError` (Status Code: 4040008) - An error that indicates that the test notification token is expired or the test notification status isn’t available.
    /// * `TestNotificationServerError` (Status Code: 5000000) - An error that indicates a server error occurred during the request processing.
    ///
    pub async fn get_test_notification_status(
        &self,
        test_notification_token: &str,
    ) -> Result<CheckTestNotificationResponse, AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/notifications/test/{}", test_notification_token);
        self.request::<CheckTestNotificationResponse, ()>(path.as_str(), Method::GET, None)
            .await
    }

    /// Get the transaction history for a given transaction ID.
    ///
    /// This method is deprecated. Please use `get_transaction_history` instead.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The identifier of the transaction to retrieve the history for.
    /// * `revision` - An optional revision string to specify the starting point of the transaction history.
    /// * `transaction_history_request` - The request object containing additional parameters for the transaction history.
    ///
    /// # Returns
    ///
    /// A response that contains the transaction history for the given transaction ID.
    ///
    /// # Errors
    ///
    /// Returns an `APIException` if the request could not be processed.
    #[deprecated(note = "Use `get_transaction_history` instead.")]
    #[allow(deprecated)]
    pub async fn get_transaction_history_v1(
        &self,
        transaction_id: &str,
        revision: Option<&str>,
        transaction_history_request: TransactionHistoryRequest,
    ) -> Result<HistoryResponse, AppStoreServerApiClientError> {
        self.get_transaction_history(
            transaction_id,
            revision,
            &transaction_history_request,
            GetTransactionHistoryVersion::V1,
        )
        .await
    }

    /// Get a list of notifications that the App Store server attempted to send to your server.
    ///
    /// [Apple Documentation](https://developer.apple.com/documentation/appstoreserverapi/get_notification_history)
    ///
    /// # Arguments
    ///
    /// * `pagination_token` - An optional token you use to get the next set of up to 20 notification history records. All responses that have more records available include a paginationToken. Omit this parameter the first time you call this endpoint.
    /// * `notification_history_request` - The request body that includes the start and end dates, and optional query constraints.
    ///
    /// # Returns
    ///
    /// A response that contains the App Store Server Notifications history for your app.
    ///
    /// # Errors
    ///
    /// * `NotificationHistoryNotFoundError` (Status Code: 4040008) - An error that indicates that the notification history is not found.
    /// * `NotificationHistoryServerError` (Status Code: 5000000) - An error that indicates a server error occurred during the request processing.
    ///
    pub async fn get_notification_history(
        &self,
        pagination_token: &str,
        notification_history_request: &NotificationHistoryRequest,
    ) -> Result<NotificationHistoryResponse, AppStoreServerApiClientError> {
        let mut path = "/inApps/v1/notifications/history".to_string();
        if !pagination_token.is_empty() {
            path.push_str(&format!("?paginationToken={}", percent_encode_query_value(pagination_token)));
        }

        self.request(path.as_str(), Method::POST, Some(notification_history_request))
            .await
    }

    /// Get a customer's in-app purchase transaction history for your app.
    ///
    /// [Apple Documentation](https://developer.apple.com/documentation/appstoreserverapi/get_transaction_history)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The identifier of a transaction that belongs to the customer, and which may be an original transaction identifier.
    /// * `revision` - A token you provide to get the next set of up to 20 transactions. All responses include a revision token. Note: For requests that use the revision token, include the same query parameters from the initial request. Use the revision token from the previous HistoryResponse.
    /// * `transaction_history_request` - The request body that includes the start and end dates, and optional query constraints.
    ///
    /// # Returns
    ///
    /// A response that contains the customer's transaction history for an app.
    ///
    /// # Errors
    ///
    /// * `TransactionHistoryNotFoundError` (Status Code: 4040010) - An error that indicates a transaction identifier wasn't found.
    /// * `TransactionHistoryServerError` (Status Code: 5000000) - An error that indicates a server error occurred during the request processing.
    ///
    pub async fn get_transaction_history(
        &self,
        transaction_id: &str,
        revision: Option<&str>,
        transaction_history_request: &TransactionHistoryRequest,
        version: GetTransactionHistoryVersion,
    ) -> Result<HistoryResponse, AppStoreServerApiClientError> {
        let mut query_strings: Vec<String> = vec![];

        if let Some(rev) = revision {
            query_strings.push(format!("revision={}", percent_encode_query_value(rev)));
        }

        if let Some(start_date) = transaction_history_request.start_date {
            query_strings.push(format!("startDate={}", start_date.timestamp_millis()));
        }

        if let Some(end_date) = transaction_history_request.end_date {
            query_strings.push(format!("endDate={}", end_date.timestamp_millis()));
        }

        if let Some(product_ids) = &transaction_history_request.product_ids {
            for item in product_ids {
                query_strings.push(format!("productId={}", percent_encode_query_value(item)));
            }
        }

        if let Some(product_types) = &transaction_history_request.product_types {
            for item in product_types {
                query_strings.push(format!("productType={}", percent_encode_query_value(item.raw_value())));
            }
        }

        if let Some(sort) = &transaction_history_request.sort {
            query_strings.push(format!("sort={}", percent_encode_query_value(sort.raw_value())));
        }

        if let Some(subscription_group_ids) = &transaction_history_request.subscription_group_identifiers {
            for item in subscription_group_ids {
                query_strings.push(format!(
                    "subscriptionGroupIdentifier={}",
                    percent_encode_query_value(item)
                ));
            }
        }

        if let Some(ownership_type) = &transaction_history_request.in_app_ownership_type {
            query_strings.push(format!(
                "inAppOwnershipType={}",
                percent_encode_query_value(ownership_type.raw_value())
            ));
        }

        if let Some(revoked) = &transaction_history_request.revoked {
            query_strings.push(format!("revoked={}", revoked));
        }

        let mut path = format!("/inApps/{}/history/{}", version.as_str(), transaction_id);

        if !query_strings.is_empty() {
            path.push('?');
            path.push_str(&query_strings.join("&"));
        }

        self.request::<HistoryResponse, ()>(path.as_str(), Method::GET, None)
            .await
    }

    /// Get information about a single transaction for your app.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/get_transaction_info)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The identifier of a transaction that belongs to the customer, and which may be an original transaction identifier.
    ///
    /// # Returns
    ///
    /// A response that contains signed transaction information for a single transaction.
    ///
    /// # Errors
    ///
    /// Returns an `APIException` if the request could not be processed.
    pub async fn get_transaction_info(
        &self,
        transaction_id: &str,
    ) -> Result<TransactionInfoResponse, AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/transactions/{}", transaction_id);
        self.request::<TransactionInfoResponse, ()>(path.as_str(), Method::GET, None)
            .await
    }

    /// Get a customer's app transaction information for your app.
    ///
    /// [Get App Transaction Info](https://developer.apple.com/documentation/appstoreserverapi/get-app-transaction-info)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - Any originalTransactionId, transactionId or appTransactionId that belongs to the customer for your app.
    ///
    /// # Returns
    ///
    /// A response that contains signed app transaction information for a customer.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn get_app_transaction_info(
        &self,
        transaction_id: &str,
    ) -> Result<AppTransactionInfoResponse, AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/transactions/appTransactions/{}", transaction_id);
        self.request::<AppTransactionInfoResponse, ()>(path.as_str(), Method::GET, None)
            .await
    }

    /// Get a customer's in-app purchases from a receipt using the order ID.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/look_up_order_id)
    ///
    /// # Arguments
    ///
    /// * `order_id` - The order ID for in-app purchases that belong to the customer.
    ///
    /// # Returns
    ///
    /// A response that includes the order lookup status and an array of signed transactions for the in-app purchases in the order.
    ///
    /// # Errors
    ///
    /// Returns an `APIException` if the request could not be processed.
    pub async fn look_up_order_id(&self, order_id: &str) -> Result<OrderLookupResponse, AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/lookup/{}", order_id);
        self.request::<OrderLookupResponse, ()>(path.as_str(), Method::GET, None)
            .await
    }

    /// Ask App Store Server Notifications to send a test notification to your server.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/request_a_test_notification)
    ///
    /// # Returns
    ///
    /// A response that contains the test notification token.
    ///
    /// # Errors
    ///
    /// Returns an `APIException` if the request could not be processed.
    pub async fn request_test_notification(
        &self,
    ) -> Result<SendTestNotificationResponse, AppStoreServerApiClientError> {
        self.request::<SendTestNotificationResponse, ()>("/inApps/v1/notifications/test", Method::POST, None)
            .await
    }

    /// Send consumption information about an In-App Purchase to the App Store after your server receives a consumption request notification.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/send-consumption-information)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier for which you're providing consumption information.
    /// * `consumption_request` - The request body containing consumption information.
    ///
    /// # Errors
    ///
    /// Returns an `APIException` if the request could not be processed.
    pub async fn send_consumption_information(
        &self,
        transaction_id: &str,
        consumption_request: &ConsumptionRequest,
    ) -> Result<(), AppStoreServerApiClientError> {
        let path = format!("/inApps/v2/transactions/consumption/{}", transaction_id);
        self.request_no_body(path.as_str(), Method::PUT, Some(consumption_request))
            .await
    }

    /// Indicate that the app delivered the consumable in-app purchase to the customer.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/finish-transaction)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier of the consumable in-app purchase.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn finish_transaction(&self, transaction_id: &str) -> Result<(), AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/transactions/{}/finish", transaction_id);
        self.request_no_body::<()>(&path, Method::POST, None).await
    }

    /// Send consumption information about a consumable in-app purchase to the App Store after your server receives a consumption request notification.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/send-consumption-information-v1)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier for which you're providing consumption information.
    /// * `consumption_request` - The request body containing consumption information.
    ///
    /// # Errors
    ///
    /// Returns an `APIException` if the request could not be processed.
    #[deprecated(note = "Use `send_consumption_information` instead.")]
    pub async fn send_consumption_data(
        &self,
        transaction_id: &str,
        consumption_request: &ConsumptionRequestV1,
    ) -> Result<(), AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/transactions/consumption/{}", transaction_id);
        self.request_no_body(path.as_str(), Method::PUT, Some(consumption_request))
            .await
    }

    /// Sets the app account token value for a purchase the customer makes outside your app,
    /// or updates its value in an existing transaction.
    ///
    /// [Set App Account Token](https://developer.apple.com/documentation/appstoreserverapi/set-app-account-token)
    ///
    /// # Arguments
    ///
    /// * `original_transaction_id` - The original transaction identifier of the transaction to receive the app account token update.
    /// * `update_app_account_token_request` - The request body that contains a valid app account token value.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The request was successful.
    ///
    /// # Errors
    ///
    /// Returns an `APIException` if the request could not be processed.
    pub async fn set_app_account_token(
        &self,
        original_transaction_id: &str,
        update_app_account_token_request: &UpdateAppAccountTokenRequest,
    ) -> Result<(), AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/transactions/{}/appAccountToken", original_transaction_id);
        self.request_no_body(path.as_str(), Method::PUT, Some(update_app_account_token_request))
            .await
    }

    /// Upload an image to use for retention messaging.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/upload-image)
    ///
    /// # Arguments
    ///
    /// * `image_identifier` - A UUID you provide to uniquely identify the image you upload.
    /// * `image` - The PNG image data to upload.
    /// * `image_size` - The size of the image you upload.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn upload_image(
        &self,
        image_identifier: Uuid,
        image: Vec<u8>,
        image_size: Option<ImageSize>,
    ) -> Result<(), AppStoreServerApiClientError> {
        let mut path = format!("/inApps/v1/messaging/image/{}", image_identifier);
        if let Some(image_size) = image_size {
            path.push_str(&format!("?imageSize={}", percent_encode_query_value(image_size.raw_value())));
        }
        self.request_custom_content(&path, Method::PUT, image, "image/png").await
    }

    /// Delete a previously uploaded image.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/delete-image)
    ///
    /// # Arguments
    ///
    /// * `image_identifier` - The identifier of the image to delete.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn delete_image(&self, image_identifier: Uuid) -> Result<(), AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/messaging/image/{}", image_identifier);
        self.request_no_body::<()>(&path, Method::DELETE, None).await
    }

    /// Get the image identifier and state for all uploaded images.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/get-image-list)
    ///
    /// # Returns
    ///
    /// A response that contains status information for all images.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn get_image_list(&self) -> Result<GetImageListResponse, AppStoreServerApiClientError> {
        self.request::<GetImageListResponse, ()>("/inApps/v1/messaging/image/list", Method::GET, None)
            .await
    }

    /// Upload a message to use for retention messaging.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/upload-message)
    ///
    /// # Arguments
    ///
    /// * `message_identifier` - A UUID you provide to uniquely identify the message you upload.
    /// * `upload_message_request_body` - The message text to upload.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn upload_message(
        &self,
        message_identifier: Uuid,
        upload_message_request_body: &UploadMessageRequestBody,
    ) -> Result<(), AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/messaging/message/{}", message_identifier);
        self.request_no_body(&path, Method::PUT, Some(upload_message_request_body))
            .await
    }

    /// Delete a previously uploaded message.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/delete-message)
    ///
    /// # Arguments
    ///
    /// * `message_identifier` - The identifier of the message to delete.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn delete_message(&self, message_identifier: Uuid) -> Result<(), AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/messaging/message/{}", message_identifier);
        self.request_no_body::<()>(&path, Method::DELETE, None).await
    }

    /// Get the message identifier and state of all uploaded messages.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/get-message-list)
    ///
    /// # Returns
    ///
    /// A response that contains status information for all messages.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn get_message_list(&self) -> Result<GetMessageListResponse, AppStoreServerApiClientError> {
        self.request::<GetMessageListResponse, ()>("/inApps/v1/messaging/message/list", Method::GET, None)
            .await
    }

    /// Configure a default message for a specific product in a specific locale.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/set-default-configuration)
    ///
    /// # Arguments
    ///
    /// * `product_id` - The product identifier for the default configuration.
    /// * `locale` - The locale for the default configuration.
    /// * `default_configuration_request` - The request body that includes the message identifier to configure as the default message.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn configure_default_message(
        &self,
        product_id: &str,
        locale: &str,
        default_configuration_request: &DefaultConfigurationRequest,
    ) -> Result<(), AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/messaging/default/{}/{}", product_id, locale);
        self.request_no_body(&path, Method::PUT, Some(default_configuration_request))
            .await
    }

    /// Delete the default message configuration for a specific product in a specific locale.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/delete-default-configuration)
    ///
    /// # Arguments
    ///
    /// * `product_id` - The product identifier for the default configuration.
    /// * `locale` - The locale for the default configuration.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn delete_default_message(
        &self,
        product_id: &str,
        locale: &str,
    ) -> Result<(), AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/messaging/default/{}/{}", product_id, locale);
        self.request_no_body::<()>(&path, Method::DELETE, None).await
    }

    /// Get the default message configuration for a specific product in a specific locale.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/get-default-configuration)
    ///
    /// # Arguments
    ///
    /// * `product_id` - The product identifier for the default configuration.
    /// * `locale` - The locale for the default configuration.
    ///
    /// # Returns
    ///
    /// The default configuration for the specified product and locale.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn get_default_message(
        &self,
        product_id: &str,
        locale: &str,
    ) -> Result<DefaultConfigurationResponse, AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/messaging/default/{}/{}", product_id, locale);
        self.request::<DefaultConfigurationResponse, ()>(&path, Method::GET, None)
            .await
    }

    /// Configure the URL of your Get Retention Message endpoint.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/configure-realtime-url)
    ///
    /// # Arguments
    ///
    /// * `realtime_url_request` - The request body that includes your endpoint's URL.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn configure_realtime_url(
        &self,
        realtime_url_request: &RealtimeUrlRequest,
    ) -> Result<(), AppStoreServerApiClientError> {
        self.request_no_body("/inApps/v1/messaging/realtime/url", Method::PUT, Some(realtime_url_request))
            .await
    }

    /// Delete the URL of your Get Retention Message endpoint.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/delete-realtime-url)
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn delete_realtime_url(&self) -> Result<(), AppStoreServerApiClientError> {
        self.request_no_body::<()>("/inApps/v1/messaging/realtime/url", Method::DELETE, None)
            .await
    }

    /// Get the URL of your Get Retention Message endpoint.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/get-realtime-url)
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn get_realtime_url(&self) -> Result<RealtimeUrlResponse, AppStoreServerApiClientError> {
        self.request::<RealtimeUrlResponse, ()>("/inApps/v1/messaging/realtime/url", Method::GET, None)
            .await
    }

    /// Initiate a performance test for retention messaging notifications.
    ///
    /// This endpoint only works in the sandbox environment.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/initiate-performance-test)
    ///
    /// # Arguments
    ///
    /// * `performance_test_request` - The request body containing the original transaction identifier.
    ///
    /// # Returns
    ///
    /// A response containing the performance test configuration and request identifier.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn initiate_performance_test(
        &self,
        performance_test_request: &PerformanceTestRequest,
    ) -> Result<PerformanceTestResponse, AppStoreServerApiClientError> {
        self.request(
            "/inApps/v1/messaging/performanceTest",
            Method::POST,
            Some(performance_test_request),
        )
        .await
    }

    /// Get the results of a performance test.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/get-performance-test-results)
    ///
    /// # Arguments
    ///
    /// * `request_id` - The ID of the performance test to return.
    ///
    /// # Returns
    ///
    /// A response containing the performance test results.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn get_performance_test_results(
        &self,
        request_id: Uuid,
    ) -> Result<PerformanceTestResultResponse, AppStoreServerApiClientError> {
        let path = format!("/inApps/v1/messaging/performanceTest/result/{}", request_id);
        self.request::<PerformanceTestResultResponse, ()>(&path, Method::GET, None)
            .await
    }
}

/// Represents the version of the Get Transaction History endpoint to use.
#[derive(Debug)]
pub enum GetTransactionHistoryVersion {
    #[deprecated(note = "Version v1 is deprecated, use v2 instead.")]
    V1,
    V2,
}

impl GetTransactionHistoryVersion {
    /// Converts the enum variant to its corresponding string representation.
    #[allow(deprecated)]
    pub fn as_str(&self) -> &str {
        match self {
            GetTransactionHistoryVersion::V1 => "v1",
            GetTransactionHistoryVersion::V2 => "v2",
        }
    }
}

/// Enum representing different API errors with associated status codes.
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
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

impl ApiErrorCode {
    /// Maps a raw error code returned by the App Store Server API into a known
    /// `ApiErrorCode` variant, falling back to `ApiErrorCode::Unknown`.
    #[allow(deprecated)]
    pub fn from_code(code: i64) -> Self {
        match code {
            4000000 => Self::GeneralBadRequest,
            4000002 => Self::InvalidAppIdentifier,
            4000005 => Self::InvalidRequestRevision,
            4000006 => Self::InvalidTransactionId,
            4000008 => Self::InvalidOriginalTransactionId,
            4000009 => Self::InvalidExtendByDays,
            4000010 => Self::InvalidExtendReasonCode,
            4000011 => Self::InvalidRequestIdentifier,
            4000012 => Self::StartDateTooFarInPast,
            4000013 => Self::StartDateAfterEndDate,
            4000014 => Self::InvalidPaginationToken,
            4000015 => Self::InvalidStartDate,
            4000016 => Self::InvalidEndDate,
            4000017 => Self::PaginationTokenExpired,
            4000018 => Self::InvalidNotificationType,
            4000019 => Self::MultipleFiltersSupplied,
            4000020 => Self::InvalidTestNotificationToken,
            4000021 => Self::InvalidSort,
            4000022 => Self::InvalidProductType,
            4000023 => Self::InvalidProductId,
            4000024 => Self::InvalidSubscriptionGroupIdentifier,
            4000025 => Self::InvalidExcludeRevoked,
            4000026 => Self::InvalidInAppOwnershipType,
            4000027 => Self::InvalidEmptyStorefrontCountryCodeList,
            4000028 => Self::InvalidStorefrontCountryCode,
            4000030 => Self::InvalidRevoked,
            4000031 => Self::InvalidStatus,
            4000032 => Self::InvalidAccountTenure,
            4000033 => Self::InvalidAppAccountToken,
            4000034 => Self::InvalidConsumptionStatus,
            4000035 => Self::InvalidCustomerConsented,
            4000036 => Self::InvalidDeliveryStatus,
            4000037 => Self::InvalidLifetimeDollarsPurchased,
            4000038 => Self::InvalidLifetimeDollarsRefunded,
            4000039 => Self::InvalidPlatform,
            4000040 => Self::InvalidPlayTime,
            4000041 => Self::InvalidSampleContentProvided,
            4000042 => Self::InvalidUserStatus,
            4000043 => Self::InvalidTransactionNotConsumable,
            4000047 => Self::InvalidTransactionTypeNotSupported,
            4000048 => Self::AppTransactionIdNotSupportedError,
            4000161 => Self::InvalidImage,
            4000162 => Self::HeaderTooLong,
            4000163 => Self::BodyTooLong,
            4000164 => Self::InvalidLocale,
            4000175 => Self::AltTextTooLong,
            4000183 => Self::InvalidAppAccountTokenUUID,
            4000185 => Self::FamilyTransactionNotSupported,
            4000187 => Self::TransactionIdNotOriginalTransactionId,
            4000211 => Self::InvalidPerformanceTestRequest,
            4000212 => Self::InvalidRequestId,
            4000213 => Self::ExistingPerformanceTestRun,
            4000215 => Self::BadRequestRealtimeUrl,
            4000216 => Self::BadRequestImageSize,
            4000218 => Self::BadRequestTooManyBulletPoints,
            4000219 => Self::BadRequestBulletPointTextTooLong,
            4000224 => Self::BadRequestAboveImageRequiresAnImage,
            4030004 => Self::SubscriptionExtensionIneligible,
            4030005 => Self::SubscriptionMaxExtension,
            4030007 => Self::FamilySharedSubscriptionExtensionIneligible,
            4030014 => Self::MaximumNumberOfImagesReached,
            4030016 => Self::MaximumNumberOfMessagesReached,
            4030017 => Self::MessageNotApproved,
            4030018 => Self::ImageNotApproved,
            4030019 => Self::ImageInUse,
            4030026 => Self::ForbiddenNoPassingTest,
            4040001 => Self::AccountNotFound,
            4040002 => Self::AccountNotFoundRetryable,
            4040003 => Self::AppNotFound,
            4040004 => Self::AppNotFoundRetryable,
            4040005 => Self::OriginalTransactionIdNotFound,
            4040006 => Self::OriginalTransactionIdNotFoundRetryable,
            4040007 => Self::ServerNotificationUrlNotFound,
            4040008 => Self::TestNotificationNotFound,
            4040009 => Self::StatusRequestNotFound,
            4040010 => Self::TransactionIdNotFound,
            4040014 => Self::ImageNotFound,
            4040015 => Self::MessageNotFound,
            4040018 => Self::PerformanceTestRunNotFound,
            4040019 => Self::AppTransactionDoesNotExist,
            4040020 => Self::DefaultMessageNotFound,
            4040021 => Self::RealtimeUrlNotFound,
            4090000 => Self::ImageAlreadyExists,
            4090001 => Self::MessageAlreadyExists,
            4290000 => Self::RateLimitExceeded,
            5000000 => Self::GeneralInternal,
            5000001 => Self::GeneralInternalRetryable,
            _ => Self::Unknown,
        }
    }
}
