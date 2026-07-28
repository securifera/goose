use crate::routes::errors::ErrorResponse;
use axum::{
    extract::Path,
    routing::{delete, get, put},
    Json, Router,
};
use goose::prompt_template::{
    get_template, list_templates, reset_template, save_template, Template,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PromptsListResponse {
    pub prompts: Vec<Template>,
}

#[derive(Serialize)]
pub struct PromptContentResponse {
    pub name: String,
    pub content: String,
    pub default_content: String,
    pub is_customized: bool,
}

#[derive(Deserialize)]
pub struct SavePromptRequest {
    pub content: String,
}

pub async fn get_prompts() -> Json<PromptsListResponse> {
    Json(PromptsListResponse {
        prompts: list_templates(),
    })
}

pub async fn get_prompt(
    Path(name): Path<String>,
) -> Result<Json<PromptContentResponse>, ErrorResponse> {
    let template = get_template(&name)
        .ok_or_else(|| ErrorResponse::not_found(format!("Prompt template '{}' not found", name)))?;

    let content = template
        .user_content
        .as_ref()
        .unwrap_or(&template.default_content);

    Ok(Json(PromptContentResponse {
        name: template.name,
        content: content.clone(),
        default_content: template.default_content,
        is_customized: template.is_customized,
    }))
}

pub async fn save_prompt(
    Path(name): Path<String>,
    Json(request): Json<SavePromptRequest>,
) -> Result<Json<String>, ErrorResponse> {
    save_template(&name, &request.content).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ErrorResponse::not_found(format!("Prompt template '{}' not found", name))
        } else {
            ErrorResponse::internal(format!("Failed to save prompt '{}': {}", name, e))
        }
    })?;

    Ok(Json(format!("Saved prompt: {}", name)))
}

pub async fn reset_prompt(Path(name): Path<String>) -> Result<Json<String>, ErrorResponse> {
    reset_template(&name).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ErrorResponse::not_found(format!("Prompt template '{}' not found", name))
        } else {
            ErrorResponse::internal(format!("Failed to reset prompt '{}': {}", name, e))
        }
    })?;

    Ok(Json(format!("Reset prompt to default: {}", name)))
}

pub fn routes() -> Router {
    Router::new()
        .route("/config/prompts", get(get_prompts))
        .route("/config/prompts/{name}", get(get_prompt))
        .route("/config/prompts/{name}", put(save_prompt))
        .route("/config/prompts/{name}", delete(reset_prompt))
}
