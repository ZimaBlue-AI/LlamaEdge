pub mod error;
pub mod llama;
pub mod mistral;

use std::str::FromStr;

use error::Result;
use xin::chat::ChatCompletionRequestMessage;

pub trait BuildPrompt: Send {
    fn build(&self, messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<String>;
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum PromptTemplateType {
    Llama2Chat,
    MistralInstructV01,
    CodeLlama,
}
impl FromStr for PromptTemplateType {
    type Err = error::PromptError;

    fn from_str(template: &str) -> std::result::Result<Self, Self::Err> {
        match template {
            "llama-2-chat" => Ok(PromptTemplateType::Llama2Chat),
            "mistral-instruct-v0.1" => Ok(PromptTemplateType::MistralInstructV01),
            "codellama-instruct" => Ok(PromptTemplateType::CodeLlama),
            _ => Err(error::PromptError::UnknownPromptTemplateType(
                template.to_string(),
            )),
        }
    }
}
impl std::fmt::Display for PromptTemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptTemplateType::Llama2Chat => write!(f, "llama-2-chat"),
            PromptTemplateType::MistralInstructV01 => write!(f, "mistral-instruct-v0.1"),
            PromptTemplateType::CodeLlama => write!(f, "codellama-instruct"),
        }
    }
}