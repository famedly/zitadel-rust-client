// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;
pub use tonic::Code as TonicErrorCode;
use zitadel::credentials::ServiceAccountError;

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum Error {
	#[error("Tonic response error: {0}")]
	TonicResponseError(#[from] tonic::Status),
	#[error("Operation timed out: {0}")]
	TimeoutError(#[from] tokio::time::error::Elapsed),
	#[error("Zitadel service account error: {0}")]
	ZitadelServiceAccountError(String, #[source] FakeError),
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
	#[error("Too many results! Got: '{0}'")]
	TooManyResults(String),
	#[error("Failed to convert event '{0}'")]
	EventConversion(String),
	#[error("Unknown error: '{0}'")]
	Unknown(String),
}

/// A fake source error that allows unwrapping a real error with
/// nested sources into nested errors with messages.
///
/// Exists to circumvent issues with errors that are not Send + Sync.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct FakeError {
	/// The error message of this error
	message: String,
	/// The nested source of this error
	source: Option<Box<FakeError>>,
}

impl FakeError {
	/// Create a FakeError from an error and its sources
	fn from_error_sources<'a>(head: &'a (dyn std::error::Error + 'static)) -> Self {
		let source = anyhow::Chain::new(head).fold(None, |fake, error| {
			Some(Box::new(FakeError { message: error.to_string(), source: fake }))
		});

		FakeError { message: head.to_string(), source }
	}
}

impl From<ServiceAccountError> for Error {
	fn from(error: ServiceAccountError) -> Self {
		Self::ZitadelServiceAccountError(error.to_string(), FakeError::from_error_sources(&error))
	}
}

/// [`Result`] Alias with error set to [`Error`]
pub type Result<R> = std::result::Result<R, Error>;

#[cfg(test)]
mod tests {
	use zitadel::credentials::ServiceAccountError;

	use super::Error;

	#[test]
	fn test_recursive_error() {
		let error = ServiceAccountError::DiscoveryError {
			source: Box::new(ServiceAccountError::DiscoveryError {
				source: Box::new(ServiceAccountError::DiscoveryError {
					source: Box::new(Error::TooManyResults("2".to_owned())),
				}),
			}),
		};

		let message = format!("{}", Error::from(error));

		assert_eq!(
			message,
			"Zitadel service account error: could not discover OIDC document: could not discover OIDC document: could not discover OIDC document: Too many results! Got: '2'"
		);
	}
}
