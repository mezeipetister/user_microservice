pub enum AppError {
    InternalError(String),
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&"AppError".to_string())
            .field(self)
            .finish()
    }
}

pub type AppResult<T> = Result<T, AppError>;
