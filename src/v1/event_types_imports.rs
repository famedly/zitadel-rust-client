#![allow(non_snake_case, missing_docs)]
use super::event_types_manual::*;

pub mod idpconfig {
	use super::*;
	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct IDPConfigAddedEvent {
		#[serde(rename = "idpConfigId")]
		pub ConfigID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "idpType")]
		pub Typ: Option<IDPConfigType>,
		#[serde(rename = "stylingType")]
		pub StylingType: Option<IDPConfigStylingType>,
		#[serde(rename = "autoRegister")]
		pub AutoRegister: Option<bool>,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct IDPConfigChangedEvent {
		#[serde(rename = "idpConfigId")]
		pub ConfigID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "stylingType")]
		pub StylingType: Option<IDPConfigStylingType>,
		#[serde(rename = "autoRegister")]
		pub AutoRegister: Option<bool>,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct IDPConfigDeactivatedEvent {
		#[serde(rename = "idpConfigId")]
		pub ConfigID: String,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct IDPConfigReactivatedEvent {
		#[serde(rename = "idpConfigId")]
		pub ConfigID: String,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct IDPConfigRemovedEvent {
		#[serde(rename = "idpConfigId")]
		pub ConfigID: String,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct JWTConfigAddedEvent {
		#[serde(rename = "idpConfigId")]
		pub IDPConfigID: String,
		#[serde(rename = "jwtEndpoint")]
		pub JWTEndpoint: Option<String>,
		#[serde(rename = "issuer")]
		pub Issuer: Option<String>,
		#[serde(rename = "keysEndpoint")]
		pub KeysEndpoint: Option<String>,
		#[serde(rename = "headerName")]
		pub HeaderName: Option<String>,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct JWTConfigChangedEvent {
		#[serde(rename = "idpConfigId")]
		pub IDPConfigID: String,
		#[serde(rename = "jwtEndpoint")]
		pub JWTEndpoint: Option<String>,
		#[serde(rename = "issuer")]
		pub Issuer: Option<String>,
		#[serde(rename = "keysEndpoint")]
		pub KeysEndpoint: Option<String>,
		#[serde(rename = "headerName")]
		pub HeaderName: Option<String>,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct OIDCConfigAddedEvent {
		#[serde(rename = "idpConfigId")]
		pub IDPConfigID: String,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "issuer")]
		pub Issuer: Option<String>,
		#[serde(rename = "authorizationEndpoint")]
		pub AuthorizationEndpoint: Option<String>,
		#[serde(rename = "tokenEndpoint")]
		pub TokenEndpoint: Option<String>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(rename = "idpDisplayNameMapping")]
		pub IDPDisplayNameMapping: Option<OIDCMappingField>,
		#[serde(rename = "usernameMapping")]
		pub UserNameMapping: Option<OIDCMappingField>,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct OIDCConfigChangedEvent {
		#[serde(rename = "idpConfigId")]
		pub IDPConfigID: String,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "issuer")]
		pub Issuer: Option<String>,
		#[serde(rename = "authorizationEndpoint")]
		pub AuthorizationEndpoint: Option<String>,
		#[serde(rename = "tokenEndpoint")]
		pub TokenEndpoint: Option<String>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(rename = "idpDisplayNameMapping")]
		pub IDPDisplayNameMapping: Option<OIDCMappingField>,
		#[serde(rename = "usernameMapping")]
		pub UserNameMapping: Option<OIDCMappingField>,
	}
}

pub mod idp {
	use super::*;
	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct AppleIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: String,
		#[serde(rename = "teamId")]
		pub TeamID: String,
		#[serde(rename = "keyId")]
		pub KeyID: String,
		#[serde(rename = "privateKey")]
		pub PrivateKey: CryptoValue,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct AppleIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "teamId")]
		pub TeamID: Option<String>,
		#[serde(rename = "keyId")]
		pub KeyID: Option<String>,
		#[serde(rename = "privateKey")]
		pub PrivateKey: Option<CryptoValue>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct AzureADIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "client_id")]
		pub ClientID: Option<String>,
		#[serde(rename = "client_secret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(rename = "tenant")]
		pub Tenant: Option<String>,
		#[serde(rename = "isEmailVerified")]
		pub IsEmailVerified: Option<bool>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct AzureADIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "client_id")]
		pub ClientID: Option<String>,
		#[serde(rename = "client_secret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(rename = "tenant")]
		pub Tenant: Option<String>,
		#[serde(rename = "isEmailVerified")]
		pub IsEmailVerified: Option<bool>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GitHubIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GitHubIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GitHubEnterpriseIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "authorizationEndpoint")]
		pub AuthorizationEndpoint: Option<String>,
		#[serde(rename = "tokenEndpoint")]
		pub TokenEndpoint: Option<String>,
		#[serde(rename = "userEndpoint")]
		pub UserEndpoint: Option<String>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GitHubEnterpriseIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "authorizationEndpoint")]
		pub AuthorizationEndpoint: Option<String>,
		#[serde(rename = "tokenEndpoint")]
		pub TokenEndpoint: Option<String>,
		#[serde(rename = "userEndpoint")]
		pub UserEndpoint: Option<String>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GitLabIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "client_id")]
		pub ClientID: String,
		#[serde(rename = "client_secret")]
		pub ClientSecret: CryptoValue,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GitLabIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "client_id")]
		pub ClientID: Option<String>,
		#[serde(rename = "client_secret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GitLabSelfHostedIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: String,
		#[serde(rename = "issuer")]
		pub Issuer: String,
		#[serde(rename = "client_id")]
		pub ClientID: String,
		#[serde(rename = "client_secret")]
		pub ClientSecret: CryptoValue,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GitLabSelfHostedIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "issuer")]
		pub Issuer: Option<String>,
		#[serde(rename = "client_id")]
		pub ClientID: Option<String>,
		#[serde(rename = "client_secret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GoogleIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: String,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: CryptoValue,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct GoogleIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct JWTIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "issuer")]
		pub Issuer: Option<String>,
		#[serde(rename = "jwtEndpoint")]
		pub JWTEndpoint: Option<String>,
		#[serde(rename = "keysEndpoint")]
		pub KeysEndpoint: Option<String>,
		#[serde(rename = "headerName")]
		pub HeaderName: Option<String>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct JWTIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "issuer")]
		pub Issuer: Option<String>,
		#[serde(rename = "jwtEndpoint")]
		pub JWTEndpoint: Option<String>,
		#[serde(rename = "keysEndpoint")]
		pub KeysEndpoint: Option<String>,
		#[serde(rename = "headerName")]
		pub HeaderName: Option<String>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct LDAPIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: String,
		#[serde(rename = "servers")]
		pub Servers: Vec<String>,
		#[serde(rename = "startTLS")]
		pub StartTLS: bool,
		#[serde(rename = "baseDN")]
		pub BaseDN: String,
		#[serde(rename = "bindDN")]
		pub BindDN: String,
		#[serde(rename = "bindPassword")]
		pub BindPassword: CryptoValue,
		#[serde(rename = "userBase")]
		pub UserBase: String,
		#[serde(rename = "userObjectClasses")]
		pub UserObjectClasses: Vec<String>,
		#[serde(rename = "userFilters")]
		pub UserFilters: Vec<String>,
		#[serde(rename = "timeout")]
		pub Timeout: Duration,
		#[serde(flatten)]
		pub LDAPAttributes: LDAPAttributes,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct LDAPIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "servers")]
		pub Servers: Option<Vec<String>>,
		#[serde(rename = "startTLS")]
		pub StartTLS: Option<bool>,
		#[serde(rename = "baseDN")]
		pub BaseDN: Option<String>,
		#[serde(rename = "bindDN")]
		pub BindDN: Option<String>,
		#[serde(rename = "bindPassword")]
		pub BindPassword: Option<CryptoValue>,
		#[serde(rename = "userBase")]
		pub UserBase: Option<String>,
		#[serde(rename = "userObjectClasses")]
		pub UserObjectClasses: Option<Vec<String>>,
		#[serde(rename = "userFilters")]
		pub UserFilters: Option<Vec<String>>,
		#[serde(rename = "timeout")]
		pub Timeout: Option<Duration>,
		#[serde(flatten)]
		pub LDAPAttributeChanges: LDAPAttributeChanges,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct OAuthIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "authorizationEndpoint")]
		pub AuthorizationEndpoint: Option<String>,
		#[serde(rename = "tokenEndpoint")]
		pub TokenEndpoint: Option<String>,
		#[serde(rename = "userEndpoint")]
		pub UserEndpoint: Option<String>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(rename = "idAttribute")]
		pub IDAttribute: Option<String>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct OAuthIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "authorizationEndpoint")]
		pub AuthorizationEndpoint: Option<String>,
		#[serde(rename = "tokenEndpoint")]
		pub TokenEndpoint: Option<String>,
		#[serde(rename = "userEndpoint")]
		pub UserEndpoint: Option<String>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(rename = "idAttribute")]
		pub IDAttribute: Option<String>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct OIDCIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: String,
		#[serde(rename = "issuer")]
		pub Issuer: String,
		#[serde(rename = "clientId")]
		pub ClientID: String,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: CryptoValue,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(rename = "idTokenMapping")]
		pub IsIDTokenMapping: Option<bool>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct OIDCIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "issuer")]
		pub Issuer: Option<String>,
		#[serde(rename = "clientId")]
		pub ClientID: Option<String>,
		#[serde(rename = "clientSecret")]
		pub ClientSecret: Option<CryptoValue>,
		#[serde(rename = "scopes")]
		pub Scopes: Option<Vec<String>>,
		#[serde(rename = "idTokenMapping")]
		pub IsIDTokenMapping: Option<bool>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct OIDCIDPMigratedAzureADEvent {
		#[serde(flatten)]
		pub AzureADIDPAddedEvent: AzureADIDPAddedEvent,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct OIDCIDPMigratedGoogleEvent {
		#[serde(flatten)]
		pub GoogleIDPAddedEvent: GoogleIDPAddedEvent,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct RemovedEvent {
		#[serde(rename = "id")]
		pub ID: String,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct SAMLIDPAddedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "metadata")]
		pub Metadata: Option<Vec<u8>>,
		#[serde(rename = "key")]
		pub Key: Option<CryptoValue>,
		#[serde(rename = "certificate")]
		pub Certificate: Option<Vec<u8>>,
		#[serde(rename = "binding")]
		pub Binding: Option<String>,
		#[serde(rename = "withSignedRequest")]
		pub WithSignedRequest: Option<bool>,
		#[serde(rename = "nameIDFormat")]
		pub NameIDFormat: Option<SAMLNameIDFormat>,
		#[serde(rename = "transientMappingAttributeName")]
		pub TransientMappingAttributeName: Option<String>,
		#[serde(flatten)]
		pub Options: Options,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct SAMLIDPChangedEvent {
		#[serde(rename = "id")]
		pub ID: String,
		#[serde(rename = "name")]
		pub Name: Option<String>,
		#[serde(rename = "metadata")]
		pub Metadata: Option<Vec<u8>>,
		#[serde(rename = "key")]
		pub Key: Option<CryptoValue>,
		#[serde(rename = "certificate")]
		pub Certificate: Option<Vec<u8>>,
		#[serde(rename = "binding")]
		pub Binding: Option<String>,
		#[serde(rename = "withSignedRequest")]
		pub WithSignedRequest: Option<bool>,
		#[serde(rename = "nameIDFormat")]
		pub NameIDFormat: Option<SAMLNameIDFormat>,
		#[serde(rename = "transientMappingAttributeName")]
		pub TransientMappingAttributeName: Option<String>,
		#[serde(flatten)]
		pub OptionChanges: OptionChanges,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct Options {
		#[serde(rename = "isCreationAllowed")]
		pub IsCreationAllowed: Option<bool>,
		#[serde(rename = "isLinkingAllowed")]
		pub IsLinkingAllowed: Option<bool>,
		#[serde(rename = "isAutoCreation")]
		pub IsAutoCreation: Option<bool>,
		#[serde(rename = "isAutoUpdate")]
		pub IsAutoUpdate: Option<bool>,
		#[serde(rename = "autoLinkingOption")]
		pub AutoLinkingOption: Option<AutoLinkingOption>,
	}

	#[derive(Debug, Clone, Deserialize, Serialize)]
	pub struct OptionChanges {
		#[serde(rename = "isCreationAllowed")]
		pub IsCreationAllowed: Option<bool>,
		#[serde(rename = "isLinkingAllowed")]
		pub IsLinkingAllowed: Option<bool>,
		#[serde(rename = "isAutoCreation")]
		pub IsAutoCreation: Option<bool>,
		#[serde(rename = "isAutoUpdate")]
		pub IsAutoUpdate: Option<bool>,
		#[serde(rename = "autoLinkingOption")]
		pub AutoLinkingOption: Option<AutoLinkingOption>,
	}
}
