#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Value not of type '{0}'")]
    XValueNotOfType(&'static str),

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    MyError(#[from] MyError),
}

#[derive(Debug)]
pub struct MyError {
    msg: String,
}

impl MyError {
    pub fn new(msg: String) -> Self {
        MyError { msg }
    }
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
