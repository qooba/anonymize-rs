#[derive(thiserror::Error, Debug)]
pub enum LLMError {
    #[error("Session not exists")]
    SessionNotExists,

    #[error("User exists")]
    UserExists,

     #[error("Login error")]
    UserLoginError,

}