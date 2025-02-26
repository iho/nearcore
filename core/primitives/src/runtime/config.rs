//! Settings of the parameters of the runtime.
use crate::runtime::config_store::INITIAL_TESTNET_CONFIG;
use crate::runtime::fees::RuntimeFeesConfig;
use crate::runtime::parameter_table::ParameterTable;
use crate::types::AccountId;
use near_primitives_core::types::Balance;
use near_primitives_core::version::PROTOCOL_VERSION;

use super::parameter_table::InvalidConfigError;

/// The structure that holds the parameters of the runtime, mostly economics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// Action gas costs, storage fees, and economic constants around them.
    ///
    /// This contains parameters that are required by the WASM runtime and the
    /// transaction runtime.
    pub fees: RuntimeFeesConfig,
    /// Config of wasm operations, also includes wasm gas costs.
    ///
    /// This contains all the configuration parameters that are only required by
    /// the WASM runtime.
    pub wasm_config: near_vm_runner::logic::Config,
    /// Config that defines rules for account creation.
    pub account_creation_config: AccountCreationConfig,
}

impl RuntimeConfig {
    pub(crate) fn new(params: &ParameterTable) -> Result<Self, InvalidConfigError> {
        RuntimeConfig::try_from(params)
    }

    pub fn initial_testnet_config() -> RuntimeConfig {
        INITIAL_TESTNET_CONFIG
            .parse()
            .and_then(|params| RuntimeConfig::new(&params))
            .expect("Failed parsing initial testnet config")
    }

    pub fn test() -> Self {
        let config_store = super::config_store::RuntimeConfigStore::new(None);
        let mut wasm_config = near_vm_runner::logic::Config::clone(
            &config_store.get_config(PROTOCOL_VERSION).wasm_config,
        );
        wasm_config.vm_kind = wasm_config.vm_kind.normalize();
        RuntimeConfig {
            fees: RuntimeFeesConfig::test(),
            wasm_config,
            account_creation_config: AccountCreationConfig::default(),
        }
    }

    pub fn free() -> Self {
        let config_store = super::config_store::RuntimeConfigStore::new(None);
        let mut wasm_config = near_vm_runner::logic::Config::clone(
            &config_store.get_config(PROTOCOL_VERSION).wasm_config,
        );
        wasm_config.vm_kind = wasm_config.vm_kind.normalize();
        wasm_config.make_free();
        Self {
            fees: RuntimeFeesConfig::free(),
            wasm_config,
            account_creation_config: AccountCreationConfig::default(),
        }
    }

    pub fn storage_amount_per_byte(&self) -> Balance {
        self.fees.storage_usage_config.storage_amount_per_byte
    }
}

/// The structure describes configuration for creation of new accounts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountCreationConfig {
    /// The minimum length of the top-level account ID that is allowed to be created by any account.
    pub min_allowed_top_level_account_length: u8,
    /// The account ID of the account registrar. This account ID allowed to create top-level
    /// accounts of any valid length.
    pub registrar_account_id: AccountId,
}

impl Default for AccountCreationConfig {
    fn default() -> Self {
        Self {
            min_allowed_top_level_account_length: 0,
            registrar_account_id: "registrar".parse().unwrap(),
        }
    }
}
