use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiRes<T = String> {
    pub success: bool,
    pub data: T,
}
