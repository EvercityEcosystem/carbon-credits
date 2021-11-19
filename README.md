# 1. Evercity Carbon Credits Pallet

This repositary contains source code of blockchain node, which is a main part of Evercity's Carbon Credits project.


# 2. Introduction


# 6. Evercity documentation

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

    ...

    [features]
default = ['std']

std = [
    ...
    'pallet-evercity-carbon-credits/std',
    ...
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
        EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage, Config<T>, Event<T>},
        EvercityFilesign: pallet_evercity_filesign::{ Module, Call, Storage, Event<T> },
        EvercityAssets: pallet_evercity_assets::{ Module, Storage, Event<T> },
        ...
    }
);
```


