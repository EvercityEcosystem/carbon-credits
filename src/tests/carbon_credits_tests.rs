use crate::tests::mock::*;
use frame_support::{assert_ok, 
//     dispatch::{
//     // DispatchResult,
//     // Vec,
// }
};
// use crate::H256;
// use crate::standard::Standard;
// use crate::project;
// use crate::annual_report::*;
use pallet_evercity_accounts::accounts::*;
use crate::tests::helpers::*;


#[test]
fn it_works_for_create_new_cc_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let create_cc_result = CarbonCredits::create_carbon_credits(Origin::signed(owner), asset_id, owner, 1, project_id);

        let passport = CarbonCredits::get_passport_by_assetid(asset_id).unwrap();
        let project = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(passport.get_project_id(), project_id);
        assert_eq!(*passport.get_asset_id_ref(), asset_id);
        assert_eq!(passport.get_annual_report_index(), project.annual_reports.len() as u64);
        assert_ok!(create_cc_result, ());
    });
}

#[test]
fn it_fails_for_create_cc_asset_not_owner_role() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;

        let _ = EvercityAccounts::account_withdraw_role(Origin::signed(ROLES[0].0), owner, CC_PROJECT_OWNER_ROLE_MASK);
        let create_cc_result = CarbonCredits::create_carbon_credits(Origin::signed(owner), asset_id, owner, 1, project_id);
        let passport = CarbonCredits::get_passport_by_assetid(asset_id);
        
        assert!(passport.is_none());
        assert_ne!(create_cc_result, Ok(()));
    });
}

#[test]
fn it_fails_for_create_cc_asset_not_owner_account() {
    new_test_ext().execute_with(|| {
        let (_, project_id, _) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let new_owner_id = 555;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), new_owner_id, CC_PROJECT_OWNER_ROLE_MASK);
        let create_cc_result = CarbonCredits::create_carbon_credits(Origin::signed(new_owner_id), asset_id, new_owner_id, 1, project_id);

        let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
        let last_annual_report = project.annual_reports.last().unwrap();
        let passport = CarbonCredits::get_passport_by_assetid(asset_id);
        
        assert!(passport.is_none());
        assert!(!last_annual_report.is_carbon_credits_released());
        assert_ne!(create_cc_result, Ok(()));
    });
}

#[test]
fn it_fails_for_create_new_cc_not_registered_project() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let asset_id = 1;
        let create_cc_result = CarbonCredits::create_carbon_credits(Origin::signed(owner), asset_id, owner, 1, project_id);

        let passport = CarbonCredits::get_passport_by_assetid(asset_id);
        // let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
        assert!(passport.is_none());
        // assert_eq!(passport.get_project_id(), project_id);
        // assert_eq!(*passport.get_asset_id_ref(), asset_id);
        // assert_eq!(passport.get_annual_report_index(), project.annual_reports.len() as u64);
        assert_ne!(create_cc_result, Ok(()));
    });
}

#[test]
fn it_works_for_set_cc_metadata() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let _ = CarbonCredits::create_carbon_credits(Origin::signed(owner), asset_id, owner, 1, project_id);
        let set_metadata_result = CarbonCredits::set_carbon_credits_metadata(
            Origin::signed(owner), 
            asset_id, 
            "CarbonToken".to_owned().as_bytes().to_vec(), 
            "CT".to_owned().as_bytes().to_vec(), 
            1
        );
        assert_ok!(set_metadata_result, ());
    });
}

#[test]
fn it_fails_for_set_empty_cc_metadata() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let _ = CarbonCredits::create_carbon_credits(Origin::signed(owner), asset_id, owner, 1, project_id);
        let set_metadata_result_empty_name = CarbonCredits::set_carbon_credits_metadata(
            Origin::signed(owner), 
            asset_id, 
            "".to_owned().as_bytes().to_vec(), 
            "CT".to_owned().as_bytes().to_vec(), 
            1
        );

        let set_metadata_result_empty_symbol = CarbonCredits::set_carbon_credits_metadata(
            Origin::signed(owner), 
            asset_id, 
            "CarbonToken".to_owned().as_bytes().to_vec(), 
            "".to_owned().as_bytes().to_vec(), 
            1
        );
        assert_ne!(set_metadata_result_empty_name, Ok(()));
        assert_ne!(set_metadata_result_empty_symbol, Ok(()));
    });
}

#[test]
fn it_works_for_mint_cc() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let _ = CarbonCredits::create_carbon_credits(Origin::signed(owner), asset_id, owner, 1, project_id);
        let mint_result = CarbonCredits::mint_carbon_credits(Origin::signed(owner), asset_id, project_id);
        let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
        let last_annual_report = project.annual_reports.last().unwrap();
        assert!(last_annual_report.is_carbon_credits_released());
        assert_ok!(mint_result, ());
    });
}

#[test]
fn it_fails_for_mint_cc_not_owner_role() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let _ = CarbonCredits::create_carbon_credits(Origin::signed(owner), asset_id, owner, 1, project_id);
        let _ = EvercityAccounts::account_withdraw_role(Origin::signed(ROLES[0].0), owner, CC_PROJECT_OWNER_ROLE_MASK);
        let mint_result = CarbonCredits::mint_carbon_credits(Origin::signed(owner), asset_id, project_id);
        let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
        let last_annual_report = project.annual_reports.last().unwrap();

        assert!(!last_annual_report.is_carbon_credits_released());
        assert_ne!(mint_result, Ok(()));
    });
}

#[test]
fn it_fails_for_mint_cc_not_project_owner() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = full_sign_annual_report_gold_standard();
        let asset_id = 1;
        let _ = CarbonCredits::create_carbon_credits(Origin::signed(owner), asset_id, owner, 1, project_id);

        let new_owner_id = 555;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), new_owner_id, CC_PROJECT_OWNER_ROLE_MASK);
        let mint_result = CarbonCredits::mint_carbon_credits(Origin::signed(new_owner_id), asset_id, project_id);
        let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
        let last_annual_report = project.annual_reports.last().unwrap();

        assert!(!last_annual_report.is_carbon_credits_released());
        assert_ne!(mint_result, Ok(()));
    });
}