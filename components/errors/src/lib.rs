use std::convert::Into;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    Msg(String),
    Tera(tera::Error),
    Io(::std::io::Error),
    Toml(toml::de::Error),
    Image(image::ImageError),
    Syntect(syntect::LoadingError),
    Lua(mlua::Error),
}

/// The Error type
#[derive(Debug)]
pub struct Error {
    /// Kind of error
    pub kind: ErrorKind,
    pub source: Option<Box<dyn StdError + Send + Sync>>,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.source {
            Some(ref err) => Some(&**err),
            None => match self.kind {
                ErrorKind::Tera(ref err) => err.source(),
                _ => None,
            },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Msg(ref message) => write!(f, "{}", message),
            ErrorKind::Tera(ref e) => write!(f, "{}", e),
            ErrorKind::Io(ref e) => write!(f, "{}", e),
            ErrorKind::Toml(ref e) => write!(f, "{}", e),
            ErrorKind::Image(ref e) => write!(f, "{}", e),
            ErrorKind::Syntect(ref e) => write!(f, "{}", e),
            ErrorKind::Lua(ref e) => write!(f, "{}", e),
        }
    }
}

impl Error {
    /// Creates generic error
    pub fn msg(value: impl ToString) -> Self {
        Self { kind: ErrorKind::Msg(value.to_string()), source: None }
    }

    /// Creates generic error with a cause
    pub fn chain(value: impl ToString, source: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        Self { kind: ErrorKind::Msg(value.to_string()), source: Some(source.into()) }
    }

    /// Create an error from a list of path collisions, formatting the output
    pub fn from_collisions(collisions: Vec<(String, Vec<String>)>) -> Self {
        let mut msg = String::from("Found path collisions:\n");

        for (path, filepaths) in collisions {
            let row = format!("- `{}` from files {:?}\n", path, filepaths);
            msg.push_str(&row);
        }

        Self { kind: ErrorKind::Msg(msg), source: None }
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::msg(e)
    }
}
impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::msg(e)
    }
}
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self { kind: ErrorKind::Toml(e), source: None }
    }
}
impl From<syntect::LoadingError> for Error {
    fn from(e: syntect::LoadingError) -> Self {
        Self { kind: ErrorKind::Syntect(e), source: None }
    }
}
impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Self { kind: ErrorKind::Tera(e), source: None }
    }
}
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Self { kind: ErrorKind::Io(e), source: None }
    }
}
impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Self { kind: ErrorKind::Image(e), source: None }
    }
}

impl From<mlua::Error> for Error {
    fn from(e: mlua::Error) -> Self {
        Self { kind: ErrorKind::Lua(e), source: None }
    }
}

/// Convenient wrapper around std::Result.
pub type Result<T> = ::std::result::Result<T, Error>;

// So we can use bail! in all other crates
#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into());
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(format!($fmt, $($arg)+).into());
    };
}
