use crate::Error;
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_noop};
use pallet_evercity_accounts::accounts::*;
use crate::tests::helpers::*;

type RuntimeError = Error<TestRuntime>;


// CC create token test:
#[test]
fn it_works_for_relase_new_cc_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let release_call = CarbonCredits::release_carbon_credits(Origin::signed(owner), project_id, asset_id, owner, 1);
        let passport = CarbonCredits::get_passport_by_assetid(asset_id).unwrap();
        let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
        let balance = Assets::balance(asset_id, owner);

        assert_ok!(release_call, ());
        assert_eq!(passport.get_project_id(), project_id);
        assert_eq!(TEST_CARBON_CREDITS_COUNT, balance);
        assert_eq!(*passport.get_asset_id_ref(), asset_id);
        assert_eq!(passport.get_annual_report_index(), project.annual_reports.len() as u64);
        assert!(project.annual_reports.last().unwrap().is_carbon_credits_released());
    });
}

#[test]
fn it_fails_for_relase_cc_not_owner_role() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;

        let _ = EvercityAccounts::account_withdraw_role(Origin::signed(ROLES[0].0), owner, CC_PROJECT_OWNER_ROLE_MASK);
        let release_call = CarbonCredits::release_carbon_credits(Origin::signed(owner), project_id, asset_id, owner, 1);
        let passport = CarbonCredits::get_passport_by_assetid(asset_id);
        let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
        
        assert!(passport.is_none());
        assert!(!project.annual_reports.last().unwrap().is_carbon_credits_released());
        assert_noop!(release_call, RuntimeError::AccountNotOwner);
    });
}

#[test]
fn it_fails_for_relase_cc_not_owner_account() {
    new_test_ext().execute_with(|| {
        let (_, project_id, _) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let new_owner_id = create_user_with_owner_role();
        let release_call = CarbonCredits::release_carbon_credits(Origin::signed(new_owner_id), project_id, asset_id, new_owner_id, 1);

        let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
        let last_annual_report = project.annual_reports.last().unwrap();
        let passport = CarbonCredits::get_passport_by_assetid(asset_id);
        
        assert!(passport.is_none());
        assert!(!last_annual_report.is_carbon_credits_released());
        assert_noop!(release_call, RuntimeError::AccountNotOwner);
    });
}

#[test]
fn it_fails_for_release_cc_no_annual_reports() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let asset_id = 1;
        let release_call = CarbonCredits::release_carbon_credits(Origin::signed(owner), project_id, asset_id, owner, 1);

        let passport = CarbonCredits::get_passport_by_assetid(asset_id);
        assert!(passport.is_none());
        assert_noop!(release_call, RuntimeError::NoAnnualReports);
    });
}

// cc transfer tests
#[test]
fn it_works_for_ransfer_cc() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), project_id, asset_id, owner, 1);
        let investor = ROLES[4].0;

        let tranfer_amount = 30;
        let transfer_result = CarbonCredits::transfer_carbon_credits(Origin::signed(owner), asset_id, investor, tranfer_amount);

        let balance = Assets::balance(asset_id, investor);

        assert_eq!(tranfer_amount, balance);
        assert_ok!(transfer_result, ());
    });
}

// // CC burn tests:
#[test]
fn it_works_for_burn_cc() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let burn_amount = 20;
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), project_id, asset_id, owner, 1);

        let burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(owner), asset_id, burn_amount);

        let burn_cert_value = CarbonCredits::get_certificates_by_account(owner)[0].burn_amount;

        assert_ok!(burn_result, ());
        assert_eq!(Assets::balance(asset_id, owner), TEST_CARBON_CREDITS_COUNT - burn_amount);
        assert_eq!(burn_amount, burn_cert_value);
    });
}

#[test]
fn it_works_for_burn_cc_after_transfer() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), project_id, asset_id, owner, 1);

        let investor = ROLES[4].0;
        let transfer_amount = 300;
        let _ = CarbonCredits::transfer_carbon_credits(Origin::signed(owner), asset_id, investor, transfer_amount);

        let first_burn_amount = 20;
        let first_burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(investor), asset_id, first_burn_amount);
        let first_burn_cert_value = CarbonCredits::get_certificates_by_account(investor)[0].burn_amount;

        let second_burn_amount = 15;
        let second_burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(investor), asset_id, second_burn_amount);
        let second_burn_cert_value = CarbonCredits::get_certificates_by_account(investor)[0].burn_amount;

        assert_ok!(first_burn_result, ());
        assert_ok!(second_burn_result, ());
        assert_eq!(first_burn_amount, first_burn_cert_value);
        assert_eq!(second_burn_amount + first_burn_amount, second_burn_cert_value);
        assert_eq!(Assets::balance(asset_id, investor), transfer_amount - first_burn_amount - second_burn_amount);
    });
}


#[test]
fn it_fails_for_burn_cc_no_assets () {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), project_id, asset_id, owner, 1);

        // Doesnt have assets
        let burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(ROLES[4].0), asset_id, 20);

        assert_noop!(burn_result, RuntimeError::InsufficientCarbonCredits);
        assert_eq!(Assets::balance(asset_id, owner), TEST_CARBON_CREDITS_COUNT);
    });
}

#[test]
fn it_fails_for_burn_cc_not_enough() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), project_id, asset_id, owner, 1);
        let burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(owner), asset_id, TEST_CARBON_CREDITS_COUNT + 666);

        assert_noop!(burn_result, RuntimeError::InsufficientCarbonCredits);
        assert_eq!(Assets::balance(asset_id, owner), TEST_CARBON_CREDITS_COUNT);
    });
}