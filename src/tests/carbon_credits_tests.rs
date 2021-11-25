// use crate::Error;
// use crate::tests::mock::*;
// use frame_support::{assert_ok, assert_noop};
// use pallet_evercity_accounts::accounts::*;
// use crate::tests::helpers::*;

// type RuntimeError = Error<TestRuntime>;

// // CC create token test:
// #[test]
// fn it_works_for_create_new_cc_gold_standard() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let create_cc_result = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);

//         let passport = CarbonCredits::get_passport_by_assetid(asset_id).unwrap();
//         let project = CarbonCredits::get_proj_by_id(project_id).unwrap();

//         assert_eq!(passport.get_project_id(), project_id);
//         assert_eq!(*passport.get_asset_id_ref(), asset_id);
//         assert_eq!(passport.get_annual_report_index(), project.annual_reports.len() as u64);
//         assert_ok!(create_cc_result, ());
//     });
// }

// #[test]
// fn it_fails_for_create_cc_asset_not_owner_role() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;

//         let _ = EvercityAccounts::account_withdraw_role(Origin::signed(ROLES[0].0), owner, CC_PROJECT_OWNER_ROLE_MASK);
//         let create_cc_result = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let passport = CarbonCredits::get_passport_by_assetid(asset_id);
        
//         assert!(passport.is_none());
//         assert_ne!(create_cc_result, Ok(()));
//     });
// }

// #[test]
// fn it_fails_for_create_cc_asset_not_owner_account() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, _) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let new_owner_id = 555;
//         let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), new_owner_id, CC_PROJECT_OWNER_ROLE_MASK);
//         let create_cc_result = CarbonCredits::set_carbon_credit_asset(Origin::signed(new_owner_id), asset_id, new_owner_id, 1, project_id);

//         let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
//         let last_annual_report = project.annual_reports.last().unwrap();
//         let passport = CarbonCredits::get_passport_by_assetid(asset_id);
        
//         assert!(passport.is_none());
//         assert!(!last_annual_report.is_carbon_credits_released());
//         assert_ne!(create_cc_result, Ok(()));
//     });
// }

// #[test]
// fn it_fails_for_create_new_cc_not_registered_project() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
//         let asset_id = 1;
//         let create_cc_result = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);

//         let passport = CarbonCredits::get_passport_by_assetid(asset_id);
//         assert!(passport.is_none());
//         assert_ne!(create_cc_result, Ok(()));
//     });
// }

// // CC Metadata test:
// #[test]
// fn it_works_for_set_cc_metadata() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let set_metadata_result = CarbonCredits::set_carbon_credits_metadata(
//             Origin::signed(owner), 
//             asset_id, 
//             "CarbonToken".to_owned().as_bytes().to_vec(), 
//             "CT".to_owned().as_bytes().to_vec(), 
//             1
//         );
//         assert_ok!(set_metadata_result, ());
//     });
// }

// #[test]
// fn it_fails_for_set_empty_cc_metadata() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let set_metadata_result_empty_name = CarbonCredits::set_carbon_credits_metadata(
//             Origin::signed(owner), 
//             asset_id, 
//             "".to_owned().as_bytes().to_vec(), 
//             "CT".to_owned().as_bytes().to_vec(), 
//             1
//         );

//         let set_metadata_result_empty_symbol = CarbonCredits::set_carbon_credits_metadata(
//             Origin::signed(owner), 
//             asset_id, 
//             "CarbonToken".to_owned().as_bytes().to_vec(), 
//             "".to_owned().as_bytes().to_vec(), 
//             1
//         );
//         assert_ne!(set_metadata_result_empty_name, Ok(()));
//         assert_ne!(set_metadata_result_empty_symbol, Ok(()));
//     });
// }

// #[test]
// fn it_fails_for_set_cc_metadata_not_existing_asset() {
//     new_test_ext().execute_with(|| {
//         let (_, _, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let set_metadata_result = CarbonCredits::set_carbon_credits_metadata(
//             Origin::signed(owner), 
//             asset_id, 
//             "CarbonToken".to_owned().as_bytes().to_vec(), 
//             "CT".to_owned().as_bytes().to_vec(), 
//             1
//         );
//         assert_ne!(set_metadata_result, Ok(()));
//     });
// }

// // CC MINT TESTS:
// #[test]
// fn it_works_for_mint_cc() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let mint_result = CarbonCredits::release_carbon_credits(Origin::signed(owner), asset_id);
//         let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
//         let last_annual_report = project.annual_reports.last().unwrap();
//         assert!(last_annual_report.is_carbon_credits_released());
//         assert_ok!(mint_result, ());
//     });
// }

// #[test]
// fn it_fails_for_mint_cc_not_existing_asset() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let mint_result = CarbonCredits::release_carbon_credits(Origin::signed(owner), asset_id);
//         let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
//         let last_annual_report = project.annual_reports.last().unwrap();
//         assert!(!last_annual_report.is_carbon_credits_released());
//         assert_ne!(mint_result, Ok(()));
//     });
// }

// #[test]
// fn it_fails_for_mint_cc_not_owner_role() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let _ = EvercityAccounts::account_withdraw_role(Origin::signed(ROLES[0].0), owner, CC_PROJECT_OWNER_ROLE_MASK);
//         let mint_result = CarbonCredits::release_carbon_credits(Origin::signed(owner), asset_id);
//         let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
//         let last_annual_report = project.annual_reports.last().unwrap();

//         assert!(!last_annual_report.is_carbon_credits_released());
//         assert_ne!(mint_result, Ok(()));
//     });
// }

// #[test]
// fn it_fails_for_mint_cc_not_project_owner() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);

//         let new_owner_id = 555;
//         let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), new_owner_id, CC_PROJECT_OWNER_ROLE_MASK);
//         let mint_result = CarbonCredits::release_carbon_credits(Origin::signed(new_owner_id), asset_id);
//         let project = CarbonCredits::get_proj_by_id(project_id).unwrap();
//         let last_annual_report = project.annual_reports.last().unwrap();

//         assert!(!last_annual_report.is_carbon_credits_released());
//         assert_ne!(mint_result, Ok(()));
//     });
// }

// // cc transfer tests
// #[test]
// fn it_works_for_ransfer_cc() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), asset_id);
//         let investor = ROLES[4].0;
//         let transfer_result = CarbonCredits::transfer_carbon_credits(Origin::signed(owner), asset_id, investor, 30);
//         assert_ok!(transfer_result, ());
//     });
// }

// // CC burn tests:
// #[test]
// fn it_works_for_burn_cc() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let burn_amount = 20;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), asset_id);
//         let burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(owner), asset_id, burn_amount);

//         let burn_cert_value = CarbonCredits::get_certificates_by_account(owner)[0].burned_amount;

//         assert_ok!(burn_result, ());
//         assert_eq!(Assets::balance(asset_id, owner), TEST_CARBON_CREDITS_COUNT - burn_amount);
//         assert_eq!(burn_amount, burn_cert_value);
//     });
// }

// #[test]
// fn it_works_for_burn_cc_after_transfer() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), asset_id);

//         let investor = ROLES[4].0;
//         let transfer_amount = 300;
//         let _ = CarbonCredits::transfer_carbon_credits(Origin::signed(owner), asset_id, investor, transfer_amount);

//         let first_burn_amount = 20;
//         let first_burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(investor), asset_id, first_burn_amount);
//         let first_burn_cert_value = CarbonCredits::get_certificates_by_account(investor)[0].burned_amount;

//         let second_burn_amount = 15;
//         let second_burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(investor), asset_id, second_burn_amount);
//         let second_burn_cert_value = CarbonCredits::get_certificates_by_account(investor)[0].burned_amount;

//         assert_ok!(first_burn_result, ());
//         assert_ok!(second_burn_result, ());
//         assert_eq!(first_burn_amount, first_burn_cert_value);
//         assert_eq!(second_burn_amount + first_burn_amount, second_burn_cert_value);
//         assert_eq!(Assets::balance(asset_id, investor), transfer_amount - first_burn_amount - second_burn_amount);
//     });
// }


// #[test]
// fn it_fails_for_burn_cc_not_owner() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), asset_id);

//         // Doesnt have assets
//         let burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(ROLES[4].0), asset_id, 20);

//         assert_ne!(burn_result, Ok(()));
//         assert_eq!(Assets::balance(asset_id, owner), TEST_CARBON_CREDITS_COUNT);
//     });
// }

// #[test]
// fn it_fails_for_burn_cc_not_enough() {
//     new_test_ext().execute_with(|| {
//         let (_, project_id, owner) = full_sign_annual_report_gold_standard();
//         let asset_id = 1;
//         let _ = CarbonCredits::set_carbon_credit_asset(Origin::signed(owner), asset_id, owner, 1, project_id);
//         let _ = CarbonCredits::release_carbon_credits(Origin::signed(owner), asset_id);
//         let burn_result = CarbonCredits::burn_carbon_credits(Origin::signed(owner), asset_id, TEST_CARBON_CREDITS_COUNT + 666);

//         assert_noop!(burn_result, RuntimeError::InsufficientCarbonCredits);
//         assert_eq!(Assets::balance(asset_id, owner), TEST_CARBON_CREDITS_COUNT);
//     });
// }