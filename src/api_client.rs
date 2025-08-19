pub mod reqwest_transport;
pub mod transport;
pub mod error;

use crate::primitives::check_test_notification_response::CheckTestNotificationResponse;
use crate::primitives::consumption_request::ConsumptionRequest;
use crate::primitives::environment::Environment;
use crate::primitives::extend_renewal_date_request::ExtendRenewalDateRequest;
use crate::primitives::extend_renewal_date_response::ExtendRenewalDateResponse;
use crate::primitives::history_response::HistoryResponse;
use crate::primitives::mass_extend_renewal_date_request::MassExtendRenewalDateRequest;
use crate::primitives::mass_extend_renewal_date_status_response::MassExtendRenewalDateStatusResponse;
use crate::primitives::notification_history_request::NotificationHistoryRequest;
use crate::primitives::notification_history_response::NotificationHistoryResponse;
use crate::primitives::order_lookup_response::OrderLookupResponse;
use crate::primitives::refund_history_response::RefundHistoryResponse;
use crate::primitives::send_test_notification_response::SendTestNotificationResponse;
use crate::primitives::status::Status;
use crate::primitives::status_response::StatusResponse;
use crate::primitives::transaction_history_request::TransactionHistoryRequest;
use crate::primitives::transaction_info_response::TransactionInfoResponse;
use crate::primitives::update_app_account_token_request::UpdateAppAccountTokenRequest;
use crate::primitives::error_payload::ErrorPayload;
use crate::api_client::transport::Transport;
use crate::api_client::error::{APIException, ConfigurationError};

use chrono::Utc;
use http::Method;
use http::{Request, Response};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

pub struct AppStoreServerAPIClient<T: Transport> {
    base_url: String,
    signing_key: Vec<u8>,
    key_id: String,
    issuer_id: String,
    bundle_id: String,
    transport: T,
}

unsafe impl<T: Transport> Send for AppStoreServerAPIClient<T> {}
unsafe impl<T: Transport> Sync for AppStoreServerAPIClient<T> {}

impl<T: Transport> AppStoreServerAPIClient<T> {
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
        // Xcode environment is only for local receipt validation and cannot be used with the API
        if matches!(environment, Environment::Xcode) {
            return Err(ConfigurationError::InvalidEnvironment(
                "Xcode environment is not supported for App Store Server API calls. Use Sandbox or Production instead."
                    .to_string(),
            ));
        }

        let base_url = environment.base_url();
        Ok(Self {
            base_url,
            signing_key,
            key_id: key_id.to_string(),
            issuer_id: issuer_id.to_string(),
            bundle_id: bundle_id.to_string(),
            transport,
        })
    }

    fn generate_token(&self) -> String {
        let future_time = Utc::now() + chrono::Duration::minutes(5);
        let key_id = (&self.key_id).to_string();

        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(key_id);

        let claims = Claims {
            bid: &self.bundle_id,
            iss: &self.issuer_id,
            aud: "appstoreconnect-v1",
            exp: future_time.timestamp(),
        };

        encode(
            &header,
            &claims,
            &EncodingKey::from_ec_pem(self.signing_key.as_slice()).unwrap(),
        )
        .unwrap()
    }

    fn build_request<B: serde::Serialize>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
    ) -> Result<Request<Vec<u8>>, APIException> {
        let url = format!("{}{}", self.base_url, path);

        let mut request_builder = Request::builder()
            .method(method)
            .uri(url)
            .header("User-Agent", "app-store-server-library/rust/1.0.0")
            .header("Authorization", format!("Bearer {}", self.generate_token()))
            .header("Accept", "application/json");

        let body_bytes = if let Some(body_data) = body {
            request_builder = request_builder.header("Content-Type", "application/json");
            serde_json::to_vec(body_data).map_err(|_| APIException {
                http_status_code: 400,
                api_error: None,
                raw_api_error: None,
                error_message: Some("Failed to serialize request body".to_string()),
            })?
        } else {
            Vec::new()
        };

        request_builder
            .body(body_bytes)
            .map_err(|e| APIException {
                http_status_code: 500,
                api_error: None,
                raw_api_error: None,
                error_message: Some(format!("Failed to build request: {}", e)),
            })
    }

    async fn make_request_with_response_body<Res>(&self, request: Request<Vec<u8>>) -> Result<Res, APIException>
    where
        Res: for<'de> Deserialize<'de>,
    {
        let response = self.make_request(request).await?;
        let body = response.into_body();
        let json_result = serde_json::from_slice::<Res>(&body).map_err(|_| APIException {
            http_status_code: 500,
            api_error: None,
            raw_api_error: None,
            error_message: Some("Failed to deserialize response JSON".to_string()),
        })?;
        Ok(json_result)
    }

    async fn make_request_without_response_body(&self, request: Request<Vec<u8>>) -> Result<(), APIException> {
        let _ = self.make_request(request).await?;
        Ok(())
    }

    async fn make_request(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, APIException> {
        let response = self
            .transport
            .send(request).await?;

        let status_code = response.status().as_u16();

        if status_code >= 200 && status_code < 300 {
            Ok(response)
        } else {
            Err(self.extract_error(&response))
        }
    }

    fn extract_error(&self, response: &Response<Vec<u8>>) -> APIException {
        let status_code = response.status().as_u16();

        serde_json::from_slice::<ErrorPayload>(response.body())
            .map(|payload| {
                let raw_api_error = payload.raw_error_code();
                APIException {
                    http_status_code: status_code,
                    api_error: payload.error_code,
                    raw_api_error,
                    error_message: payload.error_message,
                }
            })
            .unwrap_or_else(|_| APIException {
                http_status_code: status_code,
                api_error: None,
                raw_api_error: None,
                error_message: None,
            })
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
    ) -> Result<MassExtendRenewalDateStatusResponse, APIException> {
        let req = self.build_request(
            "/inApps/v1/subscriptions/extend/mass",
            Method::POST,
            Some(mass_extend_renewal_date_request),
        )?;
        self.make_request_with_response_body(req).await
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
    ) -> Result<ExtendRenewalDateResponse, APIException> {
        let path = format!(
            "/inApps/v1/subscriptions/extend/{}",
            original_transaction_id
        );
        let req = self.build_request(
            path.as_str(),
            Method::PUT,
            Some(extend_renewal_date_request),
        )?;
        self.make_request_with_response_body(req).await
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
    ) -> Result<StatusResponse, APIException> {
        let mut path = format!("/inApps/v1/subscriptions/{}", transaction_id);

        if let Some(status) = status {
            let query_params: Vec<String> = status
                .iter()
                .map(|item| format!("status={}", item.raw_value()))
                .collect();
            if !query_params.is_empty() {
                path.push_str("?");
                path.push_str(&query_params.join("&"));
            }
        }

        let req = self.build_request::<()>(path.as_str(), Method::GET, None)?;
        self.make_request_with_response_body(req).await
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
    ) -> Result<RefundHistoryResponse, APIException> {
        let mut path = format!("/inApps/v2/refund/lookup/{}", transaction_id);
        if !revision.is_empty() {
            path.push_str(&format!("?revision={}", revision));
        }
        let req = self.build_request::<()>(path.as_str(), Method::GET, None)?;
        self.make_request_with_response_body(req).await
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
    ) -> Result<MassExtendRenewalDateStatusResponse, APIException> {
        let path = format!(
            "/inApps/v1/subscriptions/extend/mass/{}/{}",
            product_id, request_identifier
        );
        let req = self.build_request::<()>(path.as_str(), Method::GET, None)?;
        self.make_request_with_response_body(req).await
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
    ) -> Result<CheckTestNotificationResponse, APIException> {
        let path = format!("/inApps/v1/notifications/test/{}", test_notification_token);
        let req = self.build_request::<()>(path.as_str(), Method::GET, None)?;
        self.make_request_with_response_body(req).await
    }

    /// Get the transaction history for a given transaction ID.
    ///
    /// This method is deprecated. Please use `get_transaction_history_with_version` instead.
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
    #[deprecated(note = "Use `get_transaction_history_with_version` instead.")]
    pub async fn get_transaction_history(
        &self,
        transaction_id: &str,
        revision: Option<&str>,
        transaction_history_request: TransactionHistoryRequest,
    ) -> Result<HistoryResponse, APIException> {
        self.get_transaction_history_with_version(
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
    ) -> Result<NotificationHistoryResponse, APIException> {
        let mut query_parameters: HashMap<&str, &str> = HashMap::new();
        if !pagination_token.is_empty() {
            query_parameters.insert("paginationToken", pagination_token);
        }

        let mut path = "/inApps/v1/notifications/history".to_string();
        if !pagination_token.is_empty() {
            path.push_str(&format!("?paginationToken={}", pagination_token));
        }

        let req = self.build_request(
            path.as_str(),
            Method::POST,
            Some(notification_history_request),
        )?;
        self.make_request_with_response_body(req).await
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
    pub async fn get_transaction_history_with_version(
        &self,
        transaction_id: &str,
        revision: Option<&str>,
        transaction_history_request: &TransactionHistoryRequest,
        version: GetTransactionHistoryVersion,
    ) -> Result<HistoryResponse, APIException> {
        let mut query_parameters: Vec<(&str, Value)> = vec![];

        if let Some(rev) = revision {
            query_parameters.push(("revision", rev.into()));
        }

        if let Some(start_date) = transaction_history_request.start_date {
            let start_date = start_date
                .timestamp_millis()
                .to_string();
            query_parameters.push(("startDate", start_date.into()));
        }

        if let Some(end_date) = transaction_history_request.end_date {
            let end_date = end_date.timestamp_millis().to_string();
            query_parameters.push(("endDate", end_date.into()));
        }

        if let Some(product_ids) = &transaction_history_request.product_ids {
            for item in product_ids {
                query_parameters.push(("productId", item.as_str().into()));
            }
        }

        if let Some(product_types) = &transaction_history_request.product_types {
            for item in product_types {
                query_parameters.push(("productType", item.raw_value().to_string().into()));
            }
        }

        if let Some(sort) = &transaction_history_request.sort {
            query_parameters.push(("sort", sort.raw_value().to_string().into()));
        }

        if let Some(subscription_group_ids) = &transaction_history_request.subscription_group_identifiers {
            for item in subscription_group_ids {
                query_parameters.push(("subscriptionGroupIdentifier", item.as_str().into()));
            }
        }

        if let Some(ownership_type) = &transaction_history_request.in_app_ownership_type {
            query_parameters.push((
                "inAppOwnershipType",
                ownership_type
                    .raw_value()
                    .to_string()
                    .into(),
            ));
        }

        if let Some(revoked) = &transaction_history_request.revoked {
            query_parameters.push(("revoked", revoked.to_string().into()));
        }

        let mut path = format!("/inApps/{}/history/{}", version.as_str(), transaction_id);

        let mut query_strings: Vec<String> = vec![];
        for (key, value) in query_parameters {
            if let Value::String(s) = value {
                query_strings.push(format!("{}={}", key, s));
            }
        }

        if !query_strings.is_empty() {
            path.push_str("?");
            path.push_str(&query_strings.join("&"));
        }

        let req = self.build_request::<()>(path.as_str(), Method::GET, None)?;
        self.make_request_with_response_body(req).await
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
    pub async fn get_transaction_info(&self, transaction_id: &str) -> Result<TransactionInfoResponse, APIException> {
        let path = format!("/inApps/v1/transactions/{}", transaction_id);
        let req = self.build_request::<()>(path.as_str(), Method::GET, None)?;
        self.make_request_with_response_body(req).await
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
    pub async fn look_up_order_id(&self, order_id: &str) -> Result<OrderLookupResponse, APIException> {
        let path = format!("/inApps/v1/lookup/{}", order_id);
        let req = self.build_request::<()>(path.as_str(), Method::GET, None)?;
        self.make_request_with_response_body(req).await
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
    pub async fn request_test_notification(&self) -> Result<SendTestNotificationResponse, APIException> {
        let path = "/inApps/v1/notifications/test";
        let req = self.build_request::<()>(path, Method::POST, None)?;
        self.make_request_with_response_body(req).await
    }

    /// Send consumption information about a consumable in-app purchase to the App Store after your server receives a consumption request notification.
    ///
    /// [Documentation](https://developer.apple.com/documentation/appstoreserverapi/send_consumption_information)
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction identifier for which you're providing consumption information.
    /// * `consumption_request` - The request body containing consumption information.
    ///
    /// # Errors
    ///
    /// Returns an `APIException` if the request could not be processed.
    pub async fn send_consumption_data(
        &self,
        transaction_id: &str,
        consumption_request: &ConsumptionRequest,
    ) -> Result<(), APIException> {
        let path = format!("/inApps/v1/transactions/consumption/{}", transaction_id);
        let req = self.build_request(path.as_str(), Method::PUT, Some(consumption_request))?;
        self.make_request_without_response_body(req).await
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
    ) -> Result<(), APIException> {
        let path = format!(
            "/inApps/v1/transactions/{}/appAccountToken",
            original_transaction_id
        );
        let req = self.build_request(
            path.as_str(),
            Method::PUT,
            Some(update_app_account_token_request),
        )?;
        self.make_request_without_response_body(req).await
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
    pub fn as_str(&self) -> &str {
        match self {
            GetTransactionHistoryVersion::V1 => "v1",
            GetTransactionHistoryVersion::V2 => "v2",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    bid: &'a str,
    iss: &'a str,
    aud: &'a str,
    exp: i64,
}
