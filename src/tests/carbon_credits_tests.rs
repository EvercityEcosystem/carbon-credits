use crate::tests::mock::*;
use frame_support::{assert_ok, dispatch::{
    DispatchResult,
    Vec,
}};
use crate::H256;
use crate::standard::Standard;
use crate::project;
use crate::annual_report::*;
use pallet_evercity_accounts::accounts::*;
use crate::tests::helpers::*;


#[test]
fn it_works_for_create_new_annual_report_gold_standard() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_hash = H256::from([0x69; 32]);

        full_sign_annual_report_gold_standard();
        let create_cc_result = CarbonCredits::create_carbon_credits(Origin::signed(owner), 1, owner, 1, project_id);
        assert_ok!(create_cc_result, ());
    });
}