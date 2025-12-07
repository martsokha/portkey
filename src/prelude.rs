//! Prelude module for convenient imports.
//!
//! The prelude re-exports the most commonly used types and traits from the Portkey SDK,
//! allowing you to import everything you need with a single glob import.

pub use crate::builder::{PortkeyBuilder, PortkeyBuilderError};
pub use crate::model::*;
pub use crate::service::{
    AssistantsService, AudioService, BatchesService, ChatService, CompletionsService,
    EmbeddingsService, FeedbackService, FilesService, FineTuningService, ImagesService,
    LogsService, MessagesService, ModelsService, ModerationsService, PromptsService,
    ResponsesService, RunsService, ThreadsService,
};
pub use crate::{Error, PortkeyClient, PortkeyConfig, Result};
