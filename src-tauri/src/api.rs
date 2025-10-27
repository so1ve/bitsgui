use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize, E: Serialize = ()> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<E>,
}

impl<T: Serialize, E: Serialize> ApiResponse<T, E> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: E) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}
