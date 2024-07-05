use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Tonic response error: {0}")]
	TonicResponseError(#[from] tonic::Status),
	#[error("Operation timed out: {0}")]
	TimoutError(#[from] tokio::time::error::Elapsed),
	#[error("Zitadel service account error: {0}")]
	ZitadelServiceAccountError(#[from] zitadel::credentials::ServiceAccountError),
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
}

pub type Result<R> = std::result::Result<R, Error>;
