use serde::{Deserialize, Serialize};

/// Image quality options for DALL-E 3
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageQuality {
    /// Standard quality
    #[default]
    Standard,

    /// High definition quality with finer details
    Hd,
}

/// Image response format
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    /// URL to the generated image (valid for 60 minutes)
    #[default]
    Url,

    /// Base64-encoded JSON of the image
    B64Json,
}

/// Image size options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum ImageSize {
    /// 256x256 pixels (DALL-E 2 only)
    #[serde(rename = "256x256")]
    Size256x256,

    /// 512x512 pixels (DALL-E 2 only)
    #[serde(rename = "512x512")]
    Size512x512,

    /// 1024x1024 pixels (DALL-E 2 and 3)
    #[serde(rename = "1024x1024")]
    #[default]
    Size1024x1024,

    /// 1792x1024 pixels (DALL-E 3 only)
    #[serde(rename = "1792x1024")]
    Size1792x1024,

    /// 1024x1792 pixels (DALL-E 3 only)
    #[serde(rename = "1024x1792")]
    Size1024x1792,
}

/// Image style for DALL-E 3
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageStyle {
    /// Hyper-real and dramatic images
    #[default]
    Vivid,

    /// More natural, less hyper-real images
    Natural,
}

/// Request to generate an image.
///
/// # Example
///
/// ```rust
/// use portkey_sdk::model::{CreateImageRequest, ImageSize, ImageQuality, ImageStyle};
///
/// let request = CreateImageRequest {
///     prompt: "A cute baby sea otter".to_string(),
///     model: Some("dall-e-3".to_string()),
///     n: Some(1),
///     quality: Some(ImageQuality::Hd),
///     response_format: None,
///     size: Some(ImageSize::Size1024x1024),
///     style: Some(ImageStyle::Vivid),
///     user: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageRequest {
    /// A text description of the desired image(s).
    ///
    /// Maximum length is 1000 characters for DALL-E 2 and 4000 characters for DALL-E 3.
    pub prompt: String,

    /// The model to use for image generation (default: dall-e-2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Number of images to generate (1-10).
    ///
    /// For DALL-E 3, only n=1 is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,

    /// Image quality (DALL-E 3 only).
    ///
    /// `hd` creates images with finer details and greater consistency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,

    /// Format for the generated images.
    ///
    /// URLs are only valid for 60 minutes after generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// Size of the generated images.
    ///
    /// For DALL-E 2: 256x256, 512x512, or 1024x1024.
    /// For DALL-E 3: 1024x1024, 1792x1024, or 1024x1792.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// Style of the generated images (DALL-E 3 only).
    ///
    /// `vivid` generates hyper-real and dramatic images.
    /// `natural` generates more natural, less hyper-real images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// A single generated image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Base64-encoded JSON of the image (if response_format is b64_json)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,

    /// URL of the generated image (if response_format is url)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The revised prompt that was used to generate the image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revised_prompt: Option<String>,
}

/// Response from image generation.
///
/// # Example
///
/// ```rust
/// use portkey_sdk::model::{ImagesResponse, Image};
///
/// let response = ImagesResponse {
///     created: 1677652288,
///     data: vec![
///         Image {
///             url: Some("https://...".to_string()),
///             b64_json: None,
///             revised_prompt: Some("A cute baby sea otter swimming".to_string()),
///         }
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagesResponse {
    /// Unix timestamp of when the images were created
    pub created: i64,

    /// Array of generated images
    pub data: Vec<Image>,
}

/// Request for editing an image with a prompt.
///
/// # Example
///
/// ```
/// use portkey_sdk::model::{CreateImageEditRequest, ImageSize, ImageResponseFormat};
///
/// let request = CreateImageEditRequest {
///     prompt: "A cute baby sea otter wearing a beret".to_string(),
///     model: Some("dall-e-2".to_string()),
///     n: Some(1),
///     size: Some(ImageSize::Size1024x1024),
///     response_format: Some(ImageResponseFormat::Url),
///     user: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageEditRequest {
    /// A text description of the desired image(s).
    ///
    /// Maximum length is 1000 characters.
    pub prompt: String,

    /// The model to use for image generation.
    ///
    /// Only `dall-e-2` is supported for edits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Number of images to generate (1-10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,

    /// Size of the generated images.
    ///
    /// Must be one of `256x256`, `512x512`, or `1024x1024`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// Format for the generated images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Request for creating a variation of an image.
///
/// # Example
///
/// ```
/// use portkey_sdk::model::{CreateImageVariationRequest, ImageSize, ImageResponseFormat};
///
/// let request = CreateImageVariationRequest {
///     model: Some("dall-e-2".to_string()),
///     n: Some(2),
///     size: Some(ImageSize::Size1024x1024),
///     response_format: Some(ImageResponseFormat::Url),
///     user: None,
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateImageVariationRequest {
    /// The model to use for image generation.
    ///
    /// Only `dall-e-2` is supported for variations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Number of images to generate (1-10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,

    /// Size of the generated images.
    ///
    /// Must be one of `256x256`, `512x512`, or `1024x1024`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// Format for the generated images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
