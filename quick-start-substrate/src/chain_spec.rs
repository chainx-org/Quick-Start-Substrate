use node_template_runtime::{
    AccountId, BalancesConfig, ConsensusConfig, FeesConfig, GenesisConfig, IndicesConfig,
    SudoConfig, TimestampConfig,
};
use primitives::{ed25519, Ed25519AuthorityId};
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialised `ChainSpec`. This is a specialisation of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
    /// Whatever the current runtime is, with just Alice as an auth.
    Development,
    /// Whatever the current runtime is, with simple Alice/Bob auths.
    LocalTestnet,
}

impl Alternative {
    /// Get an actual chain config from one of the alternatives.
    pub(crate) fn load(self) -> Result<ChainSpec, String> {
        Ok(match self {
            Alternative::Development => ChainSpec::from_genesis(
                "Quick Start Substrate Development",
                "quick_start_substrate_dev",
                || {
                    testnet_genesis(
                        vec![
                            ed25519::Pair::from_seed(b"Alice                           ")
                                .public()
                                .into(),
                        ],
                        vec![
                            ed25519::Pair::from_seed(b"Alice                           ")
                                .public()
                                .0
                                .into(),
                        ],
                        ed25519::Pair::from_seed(b"Alice                           ")
                            .public()
                            .0
                            .into(),
                    )
                },
                vec![],
                Some(TelemetryEndpoints::new(vec![(
                    STAGING_TELEMETRY_URL.to_string(),
                    0,
                )])),
                None,
                None,
                None,
            ),
            Alternative::LocalTestnet => ChainSpec::from_genesis(
                "Quick Start Substrate Local",
                "quick_start_substrate_local",
                || {
                    testnet_genesis(
                        vec![
                            ed25519::Pair::from_seed(b"Alice                           ")
                                .public()
                                .into(),
                            ed25519::Pair::from_seed(b"Bob                             ")
                                .public()
                                .into(),
                        ],
                        vec![
                            ed25519::Pair::from_seed(b"Alice                           ")
                                .public()
                                .0
                                .into(),
                            ed25519::Pair::from_seed(b"Bob                             ")
                                .public()
                                .0
                                .into(),
                            ed25519::Pair::from_seed(b"Charlie                         ")
                                .public()
                                .0
                                .into(),
                            ed25519::Pair::from_seed(b"Dave                            ")
                                .public()
                                .0
                                .into(),
                            ed25519::Pair::from_seed(b"Eve                             ")
                                .public()
                                .0
                                .into(),
                            ed25519::Pair::from_seed(b"Ferdie                          ")
                                .public()
                                .0
                                .into(),
                        ],
                        ed25519::Pair::from_seed(b"Alice                           ")
                            .public()
                            .0
                            .into(),
                    )
                },
                vec![],
                Some(TelemetryEndpoints::new(vec![(
                    STAGING_TELEMETRY_URL.to_string(),
                    0,
                )])),
                None,
                None,
                None,
            ),
        })
    }

    pub(crate) fn from(s: &str) -> Option<Self> {
        match s {
            "dev" => Some(Alternative::Development),
            "" | "local" => Some(Alternative::LocalTestnet),
            _ => None,
        }
    }
}

fn testnet_genesis(
    initial_authorities: Vec<Ed25519AuthorityId>,
    endowed_accounts: Vec<AccountId>,
    root_key: AccountId,
) -> GenesisConfig {
    GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/node_template_runtime_wasm.compact.wasm").to_vec(),
			authorities: initial_authorities.clone(),
		}),
		system: None,
		timestamp: Some(TimestampConfig {
			period: 5,					// 5 second block time.
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			existential_deposit: 500,
			transfer_fee: 0,
			creation_fee: 0,
			balances: endowed_accounts.iter().map(|&k|(k, (100_000_000))).collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		fees: Some(FeesConfig {
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
		})
	}
}
