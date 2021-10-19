use frame_support::sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
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
		EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage },
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
}

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