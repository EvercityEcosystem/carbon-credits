use frame_support::sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use frame_support::parameter_types;
use sp_core::H256;
use crate as pallet_carbon_credits;
use pallet_evercity_accounts::accounts::*;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{ Module, Call, Config, Storage, Event<T> },
		CarbonCredits: pallet_carbon_credits::{ Module, Call, Storage, Event<T> },
		EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage, Event<T> },
		Timestamp: pallet_timestamp::{ Module, Call, Storage, Inherent},
	}
);

impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl pallet_carbon_credits::Config for TestRuntime {
	type Event = Event;
}

impl pallet_evercity_accounts::Config for TestRuntime {
	type Event = Event;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 6000 / 2;
}

impl pallet_timestamp::Config for TestRuntime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}


// use sp_core::u32_trait::{_1, _2};

// pub type Balance = u128;

// pub const U_MITO: Balance = 1_000_000;
// pub const MITO: Balance = 1_000_000 * U_MITO;
// pub const fn deposit(items: u32, bytes: u32) -> Balance {
// 	items as Balance * 15 * MITO / 100 + (bytes as Balance) * 6 * MITO / 100
// }

// parameter_types! {
//     pub const AssetDeposit: Balance = 1_000 * MITO; // 1000 MITO deposit to create asset
//     pub const ApprovalDeposit: Balance = 1 * U_MITO;
//     pub const StringLimit: u32 = 50;
//     /// Key = 32 bytes, Value = 36 bytes (32+1+1+1+1)
//     /// https://github.com/paritytech/substrate/blob/069917b/frame/assets/src/lib.rs#L257L271
//     pub const MetadataDepositBase: Balance = deposit(1, 68);
//     pub const MetadataDepositPerByte: Balance = deposit(0, 1);
// }

// // pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// impl pallet_assets::Config for Runtime {
//     type Event = Event;
//     type Balance = Balance;
//     type AssetId = u32;
//     type Currency = Balances;
//     // type ForceOrigin = MoreThanHalfTechnicals;//frame_system::EnsureRoot<AccountId>
//     type ForceOrigin = frame_system::EnsureSigned<AccountId>;
//     // type ForceOrigin = frame_system::EnsureRoot<AccountId>;
//     // type ForceOrigin = pallet_dogs::EnsureAllowedAcc<AccountId>;
//     type AssetDeposit = AssetDeposit;
//     type MetadataDepositBase = MetadataDepositBase;
//     type MetadataDepositPerByte = MetadataDepositPerByte;
//     type ApprovalDeposit = ApprovalDeposit;
//     type StringLimit = StringLimit;
//     type Freezer = ();
//     type Extra = ();
//     type WeightInfo = ();
// }


// (AccountId, role)
pub static ROLES: [(u64, RoleMask); 6] = [
    (1_u64, MASTER_ROLE_MASK),
    (2_u64, CC_PROJECT_OWNER_ROLE_MASK),
    (3_u64, CC_AUDITOR_ROLE_MASK),
    (4_u64, CC_STANDARD_ROLE_MASK),
    (5_u64, CC_INVESTOR_ROLE_MASK),
    (6_u64, CC_REGISTRY_ROLE_MASK),
];

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> frame_support::sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();

	pallet_evercity_accounts::GenesisConfig::<TestRuntime> {
        // Accounts for tests
        genesis_account_registry: ROLES
            .iter()
            .map(|(acc, role)| {
                (
                    *acc,
                    AccountStruct {
                        roles: *role
                    },
                )
            })
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}

// Build genesis storage for event testing
pub fn new_test_ext_with_event() -> frame_support::sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();

	pallet_evercity_accounts::GenesisConfig::<TestRuntime> {
        // Accounts for tests
        genesis_account_registry: ROLES
            .iter()
            .map(|(acc, role)| {
                (
                    *acc,
                    AccountStruct {
                        roles: *role
                    },
                )
            })
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

// get and cut last event
pub fn last_event() -> Result<Event, ()> {
	match System::events().pop() {
		Some(ev) => Ok(ev.event),
		None => Err(())
	}
}

// Get events list
fn events() -> Vec<Event> {
    let evt = System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>();
    System::reset_events();
    evt
}