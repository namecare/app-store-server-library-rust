pub mod api_error_code;

use http::Method;
use uuid::Uuid;
use crate::api_client::api::retention_messaging_api::api_error_code::ApiErrorCode;
use crate::api_client::api_client::ApiClient;
use crate::api_client::error::ApiServiceError;
use crate::api_client::transport::Transport;
use crate::primitives::retention_messaging::default_configuration_request::DefaultConfigurationRequest;
use crate::primitives::retention_messaging::get_image_list_response::GetImageListResponse;
use crate::primitives::retention_messaging::get_message_list_response::GetMessageListResponse;
use crate::primitives::retention_messaging::performance_test_request::PerformanceTestRequest;
use crate::primitives::retention_messaging::performance_test_response::PerformanceTestResponse;
use crate::primitives::retention_messaging::performance_test_result_response::PerformanceTestResultResponse;
use crate::primitives::retention_messaging::upload_message_request_body::UploadMessageRequestBody;

pub struct RetentionMessagingApi;
pub type RetentionMessagingApiClient<T> = ApiClient<T, RetentionMessagingApi, ApiErrorCode>;
pub type ApiError = ApiServiceError<ApiErrorCode>;

impl<T: Transport> RetentionMessagingApiClient<T> {
    /// Upload an image to use for retention messaging.
    ///
    /// [Documentation](https://developer.apple.com/documentation/retentionmessaging/upload-image)
    ///
    /// # Arguments
    ///
    /// * `image_identifier` - A UUID you provide to uniquely identify the image you upload.
    /// * `image` - The PNG image data to upload.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if the request could not be processed.
    pub async fn upload_image(
        &self,
        image_identifier: Uuid,
        image: Vec<u8>,
    ) -> Result<(), ApiError> {
        let path = format!("/inApps/v1/messaging/image/{}", image_identifier);
        let req = self.build_request_with_custom_content(
            &path,
            Method::PUT,
            image,
            "image/png",
        )?;
        self.make_request_without_response_body(req).await
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
    pub async fn delete_image(
        &self,
        image_identifier: Uuid,
    ) -> Result<(), ApiError> {
        let path = format!("/inApps/v1/messaging/image/{}", image_identifier);
        let req = self.build_request::<()>(
            &path,
            Method::DELETE,
            None,
        )?;
        self.make_request_without_response_body(req).await
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
    pub async fn image_list(&self) -> Result<GetImageListResponse, ApiError> {
        let req = self.build_request::<()>(
            "/inApps/v1/messaging/image/list",
            Method::GET,
            None,
        )?;
        self.make_request_with_response_body(req).await
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
    ) -> Result<(), ApiError> {
        let path = format!("/inApps/v1/messaging/message/{}", message_identifier);
        let req = self.build_request(
            &path,
            Method::PUT,
            Some(upload_message_request_body),
        )?;
        self.make_request_without_response_body(req).await
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
    pub async fn delete_message(
        &self,
        message_identifier: Uuid,
    ) -> Result<(), ApiError> {
        let path = format!("/inApps/v1/messaging/message/{}", message_identifier);
        let req = self.build_request::<()>(
            &path,
            Method::DELETE,
            None,
        )?;
        self.make_request_without_response_body(req).await
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
    pub async fn message_list(&self) -> Result<GetMessageListResponse, ApiError> {
        let req = self.build_request::<()>(
            "/inApps/v1/messaging/message/list",
            Method::GET,
            None,
        )?;
        self.make_request_with_response_body(req).await
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
    pub async fn set_default_configuration(
        &self,
        product_id: &str,
        locale: &str,
        default_configuration_request: &DefaultConfigurationRequest,
    ) -> Result<(), ApiError> {
        let path = format!(
            "/inApps/v1/messaging/default/{}/{}",
            product_id,
            locale
        );
        let req = self.build_request(
            &path,
            Method::PUT,
            Some(default_configuration_request),
        )?;
        self.make_request_without_response_body(req).await
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
    pub async fn default_configuration(
        &self,
        product_id: &str,
        locale: &str,
    ) -> Result<DefaultConfigurationRequest, ApiError> {
        let path = format!(
            "/inApps/v1/messaging/default/{}/{}",
            product_id,
            locale
        );
        let req = self.build_request::<()>(
            &path,
            Method::GET,
            None,
        )?;
        self.make_request_with_response_body(req).await
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
    pub async fn delete_default_configuration(
        &self,
        product_id: &str,
        locale: &str,
    ) -> Result<(), ApiError> {
        let path = format!(
            "/inApps/v1/messaging/default/{}/{}",
            product_id,
            locale
        );
        let req = self.build_request::<()>(
            &path,
            Method::DELETE,
            None,
        )?;
        self.make_request_without_response_body(req).await
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
    ) -> Result<PerformanceTestResponse, ApiError> {
        let req = self.build_request(
            "/inApps/v1/messaging/performanceTest",
            Method::POST,
            Some(performance_test_request),
        )?;
        self.make_request_with_response_body(req).await
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
    pub async fn performance_test_result(
        &self,
        request_id: Uuid,
    ) -> Result<PerformanceTestResultResponse, ApiError> {
        let path = format!("/inApps/v1/messaging/performanceTest/result/{}", request_id);
        let req = self.build_request::<()>(
            &path,
            Method::GET,
            None,
        )?;
        self.make_request_with_response_body(req).await
    }
}
