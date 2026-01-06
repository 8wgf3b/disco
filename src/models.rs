use serde::Serialize;

#[derive(Serialize)]
pub struct SendResponse {
    pub success: bool,
    pub message: String,
}
