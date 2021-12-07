# 1. Evercity Carbon Credits Pallet

This repositary contains source code of blockchain node, which is a main part of Evercity's Carbon Credits project.

# 2. Introduction

Evercity Carbon Credits Pallet provides Substrate-based Sustainable Finance Protocol to issue and monitor carbon credits, as well as to integrate carbon credits with green and sustainability-linked bonds. 
Carbon credits are financial instruments, tradable certificates that represent a right to emit one ton of CO2. Carbon markets can be divided into three main parts: UNFCCC-led international carbon market which is compulsory for countries operating under Kyoto protocol (CDM), national compulsory carbon markets (e.g. in China) and voluntary carbon markets which are led by voluntary standards (VCS, Gold Standard, and others). 

# 3. Overview

Powered by Parity Substrate blockchain engine, Pallet Carbon Credits is an open-source software which allows participants to issue, transfer and monitor carbon credits.

# 4. Evercity carbon credits project main entities

Carbon Credits pallet has several main entities: 

### 4.1 Project 

Is the main entity for carbon credits project documentation signing and annual reports creating 

### 4.2 Annual Report 

Is the main entity for signing carbon credits annual gain 

### 4.3 Annual Report Burn Certificate 

Is the main entity for granting certificates for carbon credits offceting 

### 4.4 Carbon Credits Passport 

Is the entity for registering carbon credits as assets  


# 5. Evercity Roles and Carbon Creditis project scenario

### 5.1 Roles

The evercity role model presented in evercity accounts pallet https://github.com/EvercityEcosystem/evercity-accounts

- CC_PROJECT_OWNER: the role which can create carbon projects, annual report and issue caebon credits
- CC_AUDITOR: the role to sign project documentation and annual reports according to carbon credits standard
- CC_STANDARD: the role to sign project documentation and annual reports according to carbon credits standard
- CC_REGISTRY: the role to sign project documentation and annual reports according to carbon credits standard
- CC_INVESTOR: carbon credits investor

### 5.2 Basic scenario

Here is the basic scenario on of carbon credits releasing and offetting:

- Project owner creates document and stores its hash into filesign pallet (extrinsic - pallet_evercity_filesign - create_new_file())

- Project owner creates a Project in Carbon Credits pallet, choosing a carbon credits standard(extrinsic - create_project())

- Project owner can change project file id in a Project in Carbon Credits pallet to a new ine(extrinsic - change_project_file_id())

- Project owner adds signers and their roles to project(extrinsic - assign_project_signer())

- Then starts project signing, the sign order depends on carbon credits standard. 
At the end, the project owner is ready for producing annual report for carbon credits production (extrinsic - sign_project())

- Project owner creates document for annual report and stores its hash into filesign pallet(extrinsic - pallet_evercity_filesign - create_new_file())

- Project owner create annual report in project with carbon credits asset_id and asset metadata (extrinsic - create_annual_report())

- Project owner adds signers and their roles to annual report (extrinsic - assign_last_annual_report_signer())

- Then starts report signing, the sign order depends on carbon credits standard (extrinsic - sign_last_annual_report())

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

### 6.5 Check on smart sustainable bond node

```bash
git clone https://github.com/EvercityEcosystem/smart-sustainable-bond.git
cd smart-sustainable-bond
git checkout add_carbon_credits #temporary
make run
```

### 6.6 Run Unit Tests

```bash
make test
```

### 6.7 Launch linter

```bash
make lint
```

# 7. Dependent evercity pallets

Check documentation on dependent pallets:

### 7.1 Evercity Accounts - main accounting pallet with evercity role model

https://github.com/EvercityEcosystem/evercity-accounts

### 7.2 Evercity Filesing - pallet for storing file hashes and signing

https://github.com/EvercityEcosystem/filesign

### 7.3 Evercity Assets - pallet for assets

https://github.com/EvercityEcosystem/evercity-assets