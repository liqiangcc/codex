use std::collections::BTreeMap;
use std::fmt;

use codex_protocol::account::PlanType as AccountPlanType;

/// A concrete auth snapshot supplied by an external auth provider.
///
/// The caller owns credential validation, rotation, and persistence.
/// Codex keeps this snapshot in memory and uses its headers for backend requests.
#[derive(Clone, PartialEq, Eq)]
pub struct ExternalAuthSnapshot {
    headers: BTreeMap<String, String>,
    account_id: Option<String>,
    user_id: String,
    account_email: Option<String>,
    account_plan_type: Option<AccountPlanType>,
    is_fedramp_account: bool,
    capabilities: ExternalAuthSnapshotCapabilities,
}

/// Behavior explicitly enabled by the provider that supplies an
/// external auth snapshot.
///
/// All capabilities default to disabled. Callers should only enable behavior
/// that is supported by the credentials they provide.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExternalAuthSnapshotCapabilities {
    /// Whether these credentials can authenticate requests to Codex backend
    /// services.
    pub uses_codex_backend: bool,
    /// Whether these credentials represent an authenticated human ChatGPT
    /// account.
    pub has_chatgpt_account: bool,
}

impl fmt::Debug for ExternalAuthSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExternalAuthSnapshot")
            .field("headers", &"<redacted>")
            .field("account_id", &self.account_id)
            .field("user_id", &self.user_id)
            .field(
                "account_email",
                &self.account_email.as_ref().map(|_| "<redacted>"),
            )
            .field("account_plan_type", &self.account_plan_type)
            .field("is_fedramp_account", &self.is_fedramp_account)
            .field("capabilities", &self.capabilities)
            .finish()
    }
}

impl ExternalAuthSnapshot {
    /// Creates externally provided auth with backend request headers and a stable user identity.
    pub fn new(
        headers: impl IntoIterator<Item = (String, String)>,
        user_id: impl Into<String>,
    ) -> Self {
        Self {
            headers: headers.into_iter().collect(),
            account_id: None,
            user_id: user_id.into(),
            account_email: None,
            account_plan_type: Some(AccountPlanType::Unknown),
            is_fedramp_account: false,
            capabilities: ExternalAuthSnapshotCapabilities::default(),
        }
    }

    /// Adds the account selected by the caller.
    pub fn with_account_id(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = Some(account_id.into());
        self
    }

    /// Adds the account email supplied by the caller.
    pub fn with_account_email(mut self, account_email: impl Into<String>) -> Self {
        self.account_email = Some(account_email.into());
        self
    }

    /// Adds the account plan classification supplied by the caller.
    pub fn with_account_plan_type(mut self, account_plan_type: AccountPlanType) -> Self {
        self.account_plan_type = Some(account_plan_type);
        self
    }

    /// Sets whether the supplied account is a FedRAMP account.
    pub fn with_fedramp_account(mut self, is_fedramp_account: bool) -> Self {
        self.is_fedramp_account = is_fedramp_account;
        self
    }

    /// Sets the behavior supported by these externally provided credentials.
    pub fn with_capabilities(mut self, capabilities: ExternalAuthSnapshotCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Returns the request headers supplied by the caller.
    pub fn headers(&self) -> &BTreeMap<String, String> {
        &self.headers
    }

    /// Returns the selected account ID, when the caller supplied one.
    pub fn account_id(&self) -> Option<&str> {
        self.account_id.as_deref()
    }

    /// Returns the account email supplied by the caller.
    pub fn account_email(&self) -> Option<&str> {
        self.account_email.as_deref()
    }

    /// Returns the account plan classification supplied by the caller.
    pub fn account_plan_type(&self) -> Option<AccountPlanType> {
        self.account_plan_type
    }

    /// Returns whether the supplied account is a FedRAMP account.
    pub fn is_fedramp_account(&self) -> bool {
        self.is_fedramp_account
    }

    /// Returns the stable user ID supplied by the caller.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// Returns whether these credentials represent an authenticated human ChatGPT account.
    pub fn has_chatgpt_account(&self) -> bool {
        self.capabilities.has_chatgpt_account
    }

    /// Returns whether these credentials can authenticate requests to Codex backend services.
    pub fn uses_codex_backend(&self) -> bool {
        self.capabilities.uses_codex_backend
    }
}
