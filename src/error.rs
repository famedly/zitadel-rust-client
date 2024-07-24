use thiserror::Error;
use zitadel::credentials::ServiceAccountError;

pub use tonic::Code as TonicErrorCode;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Tonic response error: {0}")]
	TonicResponseError(#[from] tonic::Status),
	#[error("Operation timed out: {0}")]
	TimoutError(#[from] tokio::time::error::Elapsed),
	#[error("Zitadel service account error: {0}")]
	ZitadelServiceAccountError(String),
	#[error("Url parsing error (url crate): {0}")]
	UrlParseError(#[from] url::ParseError),
	#[error("Url parsing error (hyper/http crate): {0}")]
	InvalidUri(#[from] hyper::http::uri::InvalidUri),
	#[error("IO error: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Tonic transport error: {0}")]
	TonicTransportError(#[from] tonic::transport::Error),
	#[error("Invalid tonic metadata value: {0}")]
	InvalidMetadataValue(#[from] tonic::metadata::errors::InvalidMetadataValue),
	#[error("No Metadata found in MetadataResponse for key '{0}'")]
	MissingMetadata(String),
	#[error("Failed to parse UTF-8 data: {0}")]
	FromUtf8Error(#[from] std::string::FromUtf8Error),
}

// The `ServiceAccountError` has one error which is exposed as a
// `Box<dyn Error>`, and therefore doesn't implement `Sync` - to allow
// our error to be used in async-heavy crates, that therefore means we
// need to not embed the underlying error, and implement our own
// `From` impl without `thiserror`.
impl From<ServiceAccountError> for Error {
	fn from(error: ServiceAccountError) -> Self {
		Self::ZitadelServiceAccountError(error.to_string())
	}
}

pub type Result<R> = std::result::Result<R, Error>;
