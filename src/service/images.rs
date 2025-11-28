//! Images API service.
//!
//! This module provides methods for generating images using DALL-E models.

use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{
    CreateImageEditRequest, CreateImageRequest, CreateImageVariationRequest, ImagesResponse,
};
use reqwest::multipart::{Form, Part};
use std::future::Future;

/// Trait for Images API operations.
pub trait ImagesService {
    /// Generates images based on a text prompt.
    ///
    /// # Arguments
    ///
    /// * `request` - The image generation request containing the prompt and parameters
    ///
    /// # Returns
    ///
    /// Returns an `ImagesResponse` containing the generated images.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use portkey_sdk::{PortkeyConfig, PortkeyClient, ImagesService, AuthMethod};
    /// use portkey_sdk::model::{CreateImageRequest, ImageSize};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .with_auth_method(AuthMethod::VirtualKey {
    ///         virtual_key: "your-virtual-key".to_string()
    ///     })
    ///     .build()?;
    /// let client = PortkeyClient::new(config)?;
    ///
    /// let request = CreateImageRequest {
    ///     prompt: "A cute baby sea otter".to_string(),
    ///     model: Some("dall-e-3".to_string()),
    ///     n: Some(1),
    ///     size: Some(ImageSize::Size1024x1024),
    ///     response_format: None,
    ///     quality: None,
    ///     style: None,
    ///     user: None,
    /// };
    ///
    /// let response = client.generate_image(request).await?;
    /// println!("Generated {} images", response.data.len());
    /// # Ok(())
    /// # }
    /// ```
    fn generate_image(
        &self,
        request: CreateImageRequest,
    ) -> impl Future<Output = Result<ImagesResponse>>;

    /// Edits an image based on a prompt.
    ///
    /// # Arguments
    ///
    /// * `image_data` - The image file data as bytes (PNG, <4MB, square)
    /// * `image_name` - The name of the image file
    /// * `mask_data` - Optional mask image data (transparent areas indicate where to edit)
    /// * `mask_name` - The name of the mask file (required if mask_data is provided)
    /// * `request` - The image edit request containing the prompt and parameters
    ///
    /// # Returns
    ///
    /// Returns an `ImagesResponse` containing the edited images.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use portkey_sdk::{PortkeyConfig, PortkeyClient, ImagesService, AuthMethod};
    /// use portkey_sdk::model::{CreateImageEditRequest, ImageSize};
    /// use std::fs;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .with_auth_method(AuthMethod::VirtualKey {
    ///         virtual_key: "your-virtual-key".to_string()
    ///     })
    ///     .build()?;
    /// let client = PortkeyClient::new(config)?;
    ///
    /// let image_data = fs::read("otter.png")?;
    /// let mask_data = fs::read("mask.png")?;
    ///
    /// let request = CreateImageEditRequest {
    ///     prompt: "A cute baby sea otter wearing a beret".to_string(),
    ///     model: Some("dall-e-2".to_string()),
    ///     n: Some(1),
    ///     size: Some(ImageSize::Size1024x1024),
    ///     response_format: None,
    ///     user: None,
    /// };
    ///
    /// let response = client.edit_image(
    ///     image_data,
    ///     "otter.png",
    ///     Some(mask_data),
    ///     Some("mask.png"),
    ///     request
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn edit_image(
        &self,
        image_data: Vec<u8>,
        image_name: &str,
        mask_data: Option<Vec<u8>>,
        mask_name: Option<&str>,
        request: CreateImageEditRequest,
    ) -> impl Future<Output = Result<ImagesResponse>>;

    /// Creates a variation of an image.
    ///
    /// # Arguments
    ///
    /// * `image_data` - The image file data as bytes (PNG, <4MB, square)
    /// * `image_name` - The name of the image file
    /// * `request` - The image variation request containing parameters
    ///
    /// # Returns
    ///
    /// Returns an `ImagesResponse` containing the image variations.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use portkey_sdk::{PortkeyConfig, PortkeyClient, ImagesService, AuthMethod};
    /// use portkey_sdk::model::{CreateImageVariationRequest, ImageSize};
    /// use std::fs;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .with_auth_method(AuthMethod::VirtualKey {
    ///         virtual_key: "your-virtual-key".to_string()
    ///     })
    ///     .build()?;
    /// let client = PortkeyClient::new(config)?;
    ///
    /// let image_data = fs::read("otter.png")?;
    ///
    /// let request = CreateImageVariationRequest {
    ///     model: Some("dall-e-2".to_string()),
    ///     n: Some(2),
    ///     size: Some(ImageSize::Size1024x1024),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.create_image_variation(
    ///     image_data,
    ///     "otter.png",
    ///     request
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn create_image_variation(
        &self,
        image_data: Vec<u8>,
        image_name: &str,
        request: CreateImageVariationRequest,
    ) -> impl Future<Output = Result<ImagesResponse>>;
}

impl ImagesService for PortkeyClient {
    async fn generate_image(&self, request: CreateImageRequest) -> Result<ImagesResponse> {
        let response = self
            .post("/images/generations")
            .json(&request)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let images_response: ImagesResponse = response.json().await?;
        Ok(images_response)
    }

    async fn edit_image(
        &self,
        image_data: Vec<u8>,
        image_name: &str,
        mask_data: Option<Vec<u8>>,
        mask_name: Option<&str>,
        request: CreateImageEditRequest,
    ) -> Result<ImagesResponse> {
        // Build multipart form
        let image_part = Part::bytes(image_data).file_name(image_name.to_string());

        let mut form = Form::new()
            .part("image", image_part)
            .text("prompt", request.prompt.clone());

        // Add optional mask
        if let (Some(mask_bytes), Some(mask_filename)) = (mask_data, mask_name) {
            let mask_part = Part::bytes(mask_bytes).file_name(mask_filename.to_string());
            form = form.part("mask", mask_part);
        }

        // Add optional parameters
        if let Some(model) = request.model {
            form = form.text("model", model);
        }

        if let Some(n) = request.n {
            form = form.text("n", n.to_string());
        }

        if let Some(size) = request.size {
            let size_str = match size {
                crate::model::ImageSize::Size256x256 => "256x256",
                crate::model::ImageSize::Size512x512 => "512x512",
                crate::model::ImageSize::Size1024x1024 => "1024x1024",
                crate::model::ImageSize::Size1792x1024 => "1792x1024",
                crate::model::ImageSize::Size1024x1792 => "1024x1792",
            };
            form = form.text("size", size_str);
        }

        if let Some(response_format) = request.response_format {
            let format_str = match response_format {
                crate::model::ImageResponseFormat::Url => "url",
                crate::model::ImageResponseFormat::B64Json => "b64_json",
            };
            form = form.text("response_format", format_str);
        }

        if let Some(user) = request.user {
            form = form.text("user", user);
        }

        let response = self.post("/images/edits").multipart(form).send().await?;

        let response = response.error_for_status()?;
        let images_response: ImagesResponse = response.json().await?;
        Ok(images_response)
    }

    async fn create_image_variation(
        &self,
        image_data: Vec<u8>,
        image_name: &str,
        request: CreateImageVariationRequest,
    ) -> Result<ImagesResponse> {
        // Build multipart form
        let image_part = Part::bytes(image_data).file_name(image_name.to_string());

        let mut form = Form::new().part("image", image_part);

        // Add optional parameters
        if let Some(model) = request.model {
            form = form.text("model", model);
        }

        if let Some(n) = request.n {
            form = form.text("n", n.to_string());
        }

        if let Some(size) = request.size {
            let size_str = match size {
                crate::model::ImageSize::Size256x256 => "256x256",
                crate::model::ImageSize::Size512x512 => "512x512",
                crate::model::ImageSize::Size1024x1024 => "1024x1024",
                crate::model::ImageSize::Size1792x1024 => "1792x1024",
                crate::model::ImageSize::Size1024x1792 => "1024x1792",
            };
            form = form.text("size", size_str);
        }

        if let Some(response_format) = request.response_format {
            let format_str = match response_format {
                crate::model::ImageResponseFormat::Url => "url",
                crate::model::ImageResponseFormat::B64Json => "b64_json",
            };
            form = form.text("response_format", format_str);
        }

        if let Some(user) = request.user {
            form = form.text("user", user);
        }

        let response = self
            .post("/images/variations")
            .multipart(form)
            .send()
            .await?;

        let response = response.error_for_status()?;
        let images_response: ImagesResponse = response.json().await?;
        Ok(images_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ImageQuality, ImageSize, ImageStyle};

    #[test]
    fn test_create_image_request() {
        let request = CreateImageRequest {
            prompt: "A cute baby sea otter".to_string(),
            model: Some("dall-e-3".to_string()),
            n: Some(1),
            quality: Some(ImageQuality::Hd),
            size: Some(ImageSize::Size1024x1024),
            style: Some(ImageStyle::Vivid),
            response_format: None,
            user: None,
        };

        assert_eq!(request.prompt, "A cute baby sea otter");
        assert_eq!(request.model, Some("dall-e-3".to_string()));
        assert_eq!(request.n, Some(1));
    }
}
