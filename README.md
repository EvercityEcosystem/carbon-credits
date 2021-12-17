# 1. Evercity Carbon Credits Pallet

This repository contains source code of blockchain node, which is a main part of Evercity's Carbon Credits project.

# 2. Introduction

Evercity Carbon Credits Pallet allows to issue and monitor carbon credits - green financial instruments. The main goal of carbon credits is to increase investment in activities that reduce greenhouse gas emissions or remove carbon from the atmosphere. Each carbon credit represents one ton of CO2 (reduced or removed), which was verified by internationally accepted standards using robust methodologies. Main demand for carbon credits is now driven by investors and corporations who want to achieve net zero goals, as well as by blockchain projects that want to offset their carbon emissions. 

The value of the global carbon market increased by 20% in 2020 to $272 billion and is expected to grow further. However, there are currently several challenges that prevent market development: low transparency and fraud risk; low liquidity; double accounting, etc. Representatives of UNFCCC state that blockchain technology could help solve many of these problems, namely:
- strengthen monitoring, reporting and verification of the impacts of climate action
- improve transparency, traceability and cost-effectiveness of climate action
- build trust among climate actors
- make incentive mechanisms for climate action accessible to the poorest
- support mobilization of green finance

(https://unfccc.int/news/un-supports-blockchain-technology-for-climate-action)

At the same time, a challenge preventing rapid blockchain adoption in climate finance still lies in the high carbon footprint of main PoW blockchains including Ethereum. Parity Substrate blockchain has many advantages having a low carbon footprint, as well as enabling interoperability and scalability. 
 

# 3. Overview

Evercity Carbon Credits pallet allows issuing carbon credits according to any standard (or even creating own standard using customizable frameworks) as a result of interaction between various stakeholders: project owners, standard representatives, auditors and registries. We are replicating the globally accepted life cycle of carbon credits on blockchain making it more transparent, efficient and accessible. Key target audience of our product are project owners who issue carbon credits, companies who want to offset their emissions as well as blockchain projects who want to offset the carbon footprint of their transactions. 

# 4. Evercity carbon credits project main entities

Carbon Credits pallet has several main entities: 

### 4.1 Project 

Entity for signing carbon credits project documentation and creating annual reports 

### 4.2 Carbon Standard

Entity which determines the order of signature among three roles: CC_AUDITOR, CC_STANDARD, CC_REGISTRY

### 4.3 Annual Report 

Entity for confirming annual volume of carbon credit issuance

### 4.4 Carbon Credit Passport 

Entity for registering carbon credits as assets  

### 4.5 Carbon Offset Certificate 

Entity for granting certificates for carbon emissions offsetting using carbon credits


# 5. Evercity Roles and Carbon Creditis project scenario

### 5.1 Roles

The system of roles in Evercity is presented in Evercity accounts pallet https://github.com/EvercityEcosystem/evercity-accounts

- CC_PROJECT_OWNER: the role which can create carbon projects, annual reports and issue carbon credits
- CC_STANDARD; CC_AUDITOR; CC_REGISTRY: the roles which sign project documentation and annual reports (the order of signatures is determined by Carbon Standard entity)

### 5.2 Basic scenario

Here is the basic scenario on of carbon credits releasing and offetting:

- Project owner creates document and stores its hash into filesign pallet (extrinsic - pallet_evercity_filesign - create_new_file())

- Project owner creates a Project in Carbon Credits pallet, choosing a Carbon Standard(extrinsic - create_project())

- Project owner can change project file id in a Project in Carbon Credits pallet to a new one. Available before signing starts(extrinsic - change_project_file_id())

- Project owner adds signers and their roles to project(extrinsic - assign_project_signer())

- Then project is signed by different stakeholders, the order depends on Carbon Standard. At the end, the project owner is ready for producing annual report for carbon credits issuance (extrinsic - sign_project())

- Project owner creates document for annual report and stores its hash into filesign pallet(extrinsic - pallet_evercity_filesign - create_new_file())

- Project owner create annual report in project with carbon credits asset_id and asset metadata (extrinsic - create_annual_report())

- Project owner adds signers and their roles to annual report (extrinsic - assign_last_annual_report_signer())

- Then report is signed by different stakeholders, the order depends on Carbon Standard. (extrinsic - sign_last_annual_report())

- At the end, the project owner can release carbon credits (extrinsic - release_carbon_credits())

- User can transfer carbon credits (extrinsic - transfer_carbon_credits())

- User can burn carbon credits (extrinsic - burn_carbon_credits())


Some other functions:

- Project owner can delete last annual report if it is not full signed(extrinsic - delete_last_annual_report())

- Project owner can remove account from project signers if it didnt sign the document (extrinsic - remove_project_signer())

- Project owner can remove account from last annual report signers if it didnt sign the document (extrinsic - remove_last_annual_report_signer())


# 6. Pallet Carbon Credits documentation

### 6.1 Runtime methods

<!-- Methods of pallet-evercity are described in Rust documentation [here](http://51.15.47.43/doc/pallet_evercity/) [TEMP] -->

### 6.2 Build

```bash
git clone https://github.com/EvercityEcosystem/carbon-credits
cd carbon-credits
make build
```
### 6.3 Add to runtime cargo.toml

```toml
pallet-evercity-carbon-credits = { default-features = false, version = '0.1.12', git = 'https://github.com/EvercityEcosystem/carbon-credits' }
#...
[features]
default = ['std']

std = [
    #...
    'pallet-evercity-carbon-credits/std',
    #...
]
```

### 6.4 Add to runtime constructing

```rust
pub use pallet_evercity_carbon_credits;
impl pallet_evercity_carbon_credits::Config for Runtime {
    type Event = Event;
}
...
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        ...
        EvercityCarbonCredits: pallet_evercity_carbon_credits::{ Module, Call, Storage, Event<T>},
        ...
        // Add dependent pallets:
        EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage, Config<T>, Event<T>},
        EvercityFilesign: pallet_evercity_filesign::{ Module, Call, Storage, Event<T> },
        EvercityAssets: pallet_evercity_assets::{ Module, Storage, Event<T> },
        ...
    }
);
```

### 6.5 Run Unit Tests

```bash
make test
```

### 6.6 Launch linter

```bash
make lint
```

### 6.7 Add pallets to substrate node template

1) Add template:

```bash
git clone -b v3.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template.git
cd ./substrate-node-template
```

2) Add pallets to ./runtime/Cargo.toml

```toml
[dependencies]
pallet-evercity-carbon-credits = { default-features = false, version = '0.1.13', git = 'https://github.com/EvercityEcosystem/carbon-credits' }
pallet-evercity-filesign = { default-features = false, version = '0.1.3', git = 'https://github.com/EvercityEcosystem/filesign'}
pallet-evercity-assets = { default-features = false, version = '0.1.0', git = 'https://github.com/EvercityEcosystem/evercity-assets' }
pallet-evercity-accounts = { default-features = false, version = '0.1.7', git = 'https://github.com/EvercityEcosystem/evercity-accounts' }

[features]
std = [
    #...
    'pallet-evercity-carbon-credits/std',
    'pallet-evercity-filesign/std',
    'pallet-evercity-assets/std',
    'pallet-evercity-accounts/std',
    #...
]
```
3) Add pallets configs to runtime/src/lib.rs

```rust
use pallet_evercity_filesign;

impl pallet_evercity_filesign::Config for Runtime {
    type Event = Event;
}

use pallet_evercity_assets;

parameter_types! {
    pub const AssetDepositBase: Balance = 0;
    pub const AssetDepositPerZombie: Balance = 0;
    pub const ApprovalDeposit: Balance = 0;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_evercity_assets::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type AssetId = u64;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;
    type AssetDepositBase = AssetDepositBase;
    type AssetDepositPerZombie = AssetDepositPerZombie;
    type StringLimit = StringLimit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type WeightInfo = pallet_evercity_assets::weights::SubstrateWeight<Runtime>;
}

pub use pallet_evercity_carbon_credits;
impl pallet_evercity_carbon_credits::Config for Runtime {
    type Event = Event;
}

pub use pallet_evercity_accounts;
impl pallet_evercity_accounts::Config for Runtime {
    type Event = Event;
}
```

4) Add pallets to Runtime

```rust
    construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...

        EvercityCarbonCredits: pallet_evercity_carbon_credits::{ Module, Call, Storage, Event<T>},
        EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage, Config<T>, Event<T>},
        EvercityFilesign: pallet_evercity_filesign::{ Module, Call, Storage, Event<T> },
        EvercityAssets: pallet_evercity_assets::{ Module, Storage, Event<T> },
	}
);
```

5) Change content of node/src/chain_spec.rs to:

```rust
use sp_core::{Pair, Public, sr25519, crypto::Ss58Codec};
use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature, EvercityAccountsConfig
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;

use node_template_runtime::pallet_evercity_accounts;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],

			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			vec![
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					pallet_evercity_accounts::accounts::MASTER_ROLE_MASK,
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					pallet_evercity_accounts::accounts::CC_PROJECT_OWNER_ROLE_MASK,
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					pallet_evercity_accounts::accounts::CC_AUDITOR_ROLE_MASK,
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					pallet_evercity_accounts::accounts::CC_STANDARD_ROLE_MASK,
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					pallet_evercity_accounts::accounts::CC_REGISTRY_ROLE_MASK,
				),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			let master_account_id: AccountId =
			Ss58Codec::from_ss58check("5DJBx8EcrJqWqDQDe3xPd7Bw2zL3obvHigdLZKVGDHx7GRwW")
				.unwrap();
			testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			vec![(master_account_id.clone(), pallet_evercity_accounts::accounts::MASTER_ROLE_MASK)],
			true,
		)},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	evercity_accounts: Vec<(AccountId, pallet_evercity_accounts::accounts::RoleMask)>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),

		pallet_evercity_accounts: Some(EvercityAccountsConfig {
            // set roles for each pre-set accounts (set role)
            genesis_account_registry: evercity_accounts
                .iter()
                .map(|(acc, role)| {
                    (
                        acc.clone(),
                        pallet_evercity_accounts::accounts::AccountStruct {
                            roles: *role,
                        },
                    )
                })
                .collect(),
        }),
	}
}
```

6) Run

```bash
cargo build --release
./target/release/node-template --dev --tmp
```
or 

```bash
cargo build --release -- --dev --tmp
```

6.1) If you face a compile problems with parity-db and wasm-bindgen, run

```bash
cargo update -p parity-db
cargo update -p wasm-bindgen
```

# 7. Dependent evercity pallets

Check documentation on dependent pallets:

### 7.1 Evercity Accounts - main accounting pallet with evercity role model

https://github.com/EvercityEcosystem/evercity-accounts

### 7.2 Evercity Filesing - pallet for storing file hashes and signing

https://github.com/EvercityEcosystem/filesign

### 7.3 Evercity Assets - pallet for assets

https://github.com/EvercityEcosystem/evercity-assets
