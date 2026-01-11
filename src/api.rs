use crate::bot::BotState;
use crate::models::SendResponse;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serenity::model::id::ChannelId;
use serenity::builder::CreateMessage;
use std::sync::Arc;
use tracing::{error, info};

pub async fn run_server(bot_state: BotState, port: u16) {
    let app = Router::new()
        .route("/", post(send_message))
        .with_state(Arc::new(bot_state));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind port");

    info!(port, "HTTP server listening");

    axum::serve(listener, app).await.expect("HTTP server error");
}

async fn send_message(
    State(state): State<Arc<BotState>>,
    mut multipart: Multipart,
) -> Result<Json<SendResponse>, StatusCode> {
    let mut channel_id = None;
    let mut content = None;
    let mut files = Vec::new();

    // Parse multipart form
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "channel_id" => {
                channel_id = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "content" => {
                content = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "files" => {
                let file_name = field.file_name().unwrap_or("unknown").to_string();
                let content_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST)?
                    .to_vec();

                files.push((file_name, content_type, bytes));
            }
            _ => {}
        }
    }

    // Validate required fields
    let channel_id = match channel_id {
        Some(id) => id,
        None => {
            error!("Missing channel_id");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Parse channel_id to u64
    let channel_id_u64 = match channel_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            error!("Invalid channel_id format");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    info!(
        channel_id,
        ?content,
        file_count = files.len(),
        "Received send request"
    );

    // Validate file count
    if files.len() > 10 {
        error!("Too many files: {}", files.len());
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate file sizes (25MB limit)
    for (name, _, bytes) in &files {
        if bytes.len() > 25 * 1024 * 1024 {
            error!(file = %name, size = bytes.len(), "File too large");
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // Send message to Discord
    match send_to_discord(state.http.clone(), channel_id_u64, content, files).await {
        Ok(_) => {
            info!(channel_id, "Message sent successfully");
            Ok(Json(SendResponse {
                success: true,
                message: "Message sent".to_string(),
            }))
        }
        Err(e) => {
            error!(channel_id, error = %e, "Failed to send message");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn send_to_discord(
    http: Arc<serenity::http::Http>,
    channel_id: u64,
    content: Option<String>,
    files: Vec<(String, String, Vec<u8>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = ChannelId::new(channel_id);

    // Send the message using a closure
    let mut create_message = CreateMessage::new();

    if let Some(text) = content {
        create_message = create_message.content(text);
    }

    // Add files if any
    if !files.is_empty() {
        for (file_name, _content_type, bytes) in files {
            let file = serenity::all::CreateAttachment::bytes(bytes, file_name.clone());
            create_message = create_message.add_file(file);
        }
    }

    channel.send_message(&http, create_message).await?;

    Ok(())
}
