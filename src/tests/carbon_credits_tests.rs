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
fn it_works_for_create_new_cc_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        full_sign_annual_report_gold_standard();
        let create_cc_result = CarbonCredits::create_carbon_credits(Origin::signed(owner), 1, owner, 1, project_id);
        assert_ok!(create_cc_result, ());
    });
}

#[test]
fn it_fails_for_create_cc_asset_not_owner_role() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        full_sign_annual_report_gold_standard();

        let _ = EvercityAccounts::account_withdraw_role(Origin::signed(ROLES[0].0), owner, CC_PROJECT_OWNER_ROLE_MASK);
        let create_cc_result = CarbonCredits::create_carbon_credits(Origin::signed(owner), 1, owner, 1, project_id);
        assert_ne!(create_cc_result, Ok(()));
    });
}

#[test]
fn it_fails_for_create_cc_asset_not_owner_account() {
    new_test_ext().execute_with(|| {
        let (_, project_id, _) = get_registerd_project_and_owner_gold_standard();
        full_sign_annual_report_gold_standard();
        let new_owner_id = 555;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), new_owner_id, CC_PROJECT_OWNER_ROLE_MASK);
        let create_cc_result = CarbonCredits::create_carbon_credits(Origin::signed(new_owner_id), 1, new_owner_id, 1, project_id);
        assert_ne!(create_cc_result, Ok(()));
    });
}