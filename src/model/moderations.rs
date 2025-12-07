use serde::{Deserialize, Serialize};

/// Request to create a moderation.
///
/// # Example
///
/// ```rust,ignore
/// use portkey::model::CreateModerationRequest;
///
/// let request = CreateModerationRequest::builder()
///     .input("I want to hurt someone")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateModerationRequest {
    /// The input text to classify.
    pub input: ModerationInput,

    /// The content moderation model you would like to use.
    /// Defaults to "text-moderation-latest".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl Default for CreateModerationRequest {
    fn default() -> Self {
        Self {
            input: ModerationInput::String(String::new()),
            model: Some("text-moderation-latest".to_string()),
        }
    }
}

/// Input for moderation can be a single string or an array of strings.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModerationInput {
    String(String),
    Array(Vec<String>),
}

impl From<String> for ModerationInput {
    fn from(s: String) -> Self {
        ModerationInput::String(s)
    }
}

impl From<&str> for ModerationInput {
    fn from(s: &str) -> Self {
        ModerationInput::String(s.to_string())
    }
}

impl From<Vec<String>> for ModerationInput {
    fn from(v: Vec<String>) -> Self {
        ModerationInput::Array(v)
    }
}

/// Response from the moderation endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModerationResponse {
    /// The unique identifier for the moderation request.
    pub id: String,

    /// The model used for moderation.
    pub model: String,

    /// A list of moderation objects.
    pub results: Vec<ModerationResult>,
}

/// A single moderation result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModerationResult {
    /// Whether any of the below categories are flagged.
    pub flagged: bool,

    /// A list of the categories, and whether they are flagged or not.
    pub categories: ModerationCategories,

    /// A list of the categories along with their scores as predicted by model.
    pub category_scores: ModerationCategoryScores,
}

/// Categories that were checked in the moderation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModerationCategories {
    /// Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste.
    pub hate: bool,

    /// Hateful content that also includes violence or serious harm towards the targeted group.
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,

    /// Content that expresses, incites, or promotes harassing language towards any target.
    pub harassment: bool,

    /// Harassment content that also includes violence or serious harm towards any target.
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,

    /// Content that promotes, encourages, or depicts acts of self-harm.
    #[serde(rename = "self-harm")]
    pub self_harm: bool,

    /// Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm.
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,

    /// Content that encourages performing acts of self-harm.
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,

    /// Content meant to arouse sexual excitement.
    pub sexual: bool,

    /// Sexual content that includes an individual who is under 18 years old.
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,

    /// Content that depicts death, violence, or physical injury.
    pub violence: bool,

    /// Content that depicts death, violence, or physical injury in graphic detail.
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

/// Scores for each moderation category.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModerationCategoryScores {
    /// Score for hate content.
    pub hate: f64,

    /// Score for threatening hate content.
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,

    /// Score for harassment content.
    pub harassment: f64,

    /// Score for threatening harassment content.
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f64,

    /// Score for self-harm content.
    #[serde(rename = "self-harm")]
    pub self_harm: f64,

    /// Score for self-harm intent content.
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f64,

    /// Score for self-harm instructions content.
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f64,

    /// Score for sexual content.
    pub sexual: f64,

    /// Score for sexual content involving minors.
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,

    /// Score for violent content.
    pub violence: f64,

    /// Score for graphic violent content.
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
}
