use crate::Error;
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_noop, dispatch::{
    DispatchResult,
    Vec,
}};
use crate::standard::Standard;
use crate::annual_report::*;
use pallet_evercity_accounts::accounts::*;
use crate::tests::helpers::*;
use sp_std::vec;

type RuntimeError = Error<TestRuntime>;


#[test]
fn it_works_for_create_new_annual_report_with_file() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();

        let file_id = [11, 22, 33, 44, 55, 66, 77, 88, 99, 0, 11, 12, 13, 14, 15, 16];
        let tag = "my_annual_report".to_owned().as_bytes().to_vec();
        let filehash = pallet_evercity_filesign::file::H256::from([0x88; 32]);

        let create_report_result = CarbonCredits::create_annual_report_with_file(
            Origin::signed(owner), project_id, file_id, filehash, tag, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );


        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project.annual_reports.len() + 1, project_with_report.annual_reports.len());
        assert_eq!(REPORT_PROJECT_OWNER_SIGN_PENDING, project_with_report.annual_reports.last().unwrap().state);
        assert_ok!(create_report_result, ());
    });
}

#[test]
fn it_fails_for_create_new_annual_report_with_file_not_project_owner() {
    new_test_ext().execute_with(|| {
        let (project, project_id, _) = get_registerd_project_and_owner_gold_standard();

        let other_owner = create_user_with_owner_role();

        let file_id = [11, 22, 33, 44, 55, 66, 77, 88, 99, 0, 11, 12, 13, 14, 15, 16];
        let tag = "my_annual_report".to_owned().as_bytes().to_vec();
        let filehash = pallet_evercity_filesign::file::H256::from([0x88; 32]);

        let create_report_result = CarbonCredits::create_annual_report_with_file(
            Origin::signed(other_owner), project_id, file_id, filehash, tag, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );

        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());
        assert_noop!(create_report_result, RuntimeError::AccountNotOwner);
    });
}

#[test]
fn it_fails_for_create_new_annual_report_with_file_not_owner_role() {
    new_test_ext().execute_with(|| {
        let (project, project_id, _) = get_registerd_project_and_owner_gold_standard();

        let not_owner = ROLES[2].0;

        let file_id = [11, 22, 33, 44, 55, 66, 77, 88, 99, 0, 11, 12, 13, 14, 15, 16];
        let tag = "my_annual_report".to_owned().as_bytes().to_vec();
        let filehash = pallet_evercity_filesign::file::H256::from([0x88; 32]);

        let create_report_result = CarbonCredits::create_annual_report_with_file(
            Origin::signed(not_owner), project_id, file_id, filehash, tag, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );

        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());
        assert_noop!(create_report_result, RuntimeError::AccountNotOwner);
    });
}

#[test]
fn it_fails_for_create_new_annual_report_with_file_bad_metadata() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();

        let file_id = [11, 22, 33, 44, 55, 66, 77, 88, 99, 0, 11, 12, 13, 14, 15, 16];
        let tag = "my_annual_report".to_owned().as_bytes().to_vec();
        let filehash = pallet_evercity_filesign::file::H256::from([0x88; 32]);

        let create_report_empty_name_result = CarbonCredits::create_annual_report_with_file(
            Origin::signed(owner), project_id, file_id, filehash, tag.clone(), TEST_CARBON_CREDITS_COUNT,
            Vec::new() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );

        let create_report_empty_symbol_result = CarbonCredits::create_annual_report_with_file(
            Origin::signed(owner), project_id, file_id, filehash, tag, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , Vec::new(), TEST_CARBON_CREDITS_DECIMAL
        );


        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());
        assert_noop!(create_report_empty_name_result, RuntimeError::BadMetadataParameters);
        assert_noop!(create_report_empty_symbol_result, RuntimeError::BadMetadataParameters);
    });
}


#[test]
fn it_works_for_create_new_annual_report_gold_standard() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();

        let create_report_result = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, create_annual_report_file(owner), TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project.annual_reports.len() + 1, project_with_report.annual_reports.len());
        assert_eq!(REPORT_PROJECT_OWNER_SIGN_PENDING, project_with_report.annual_reports.last().unwrap().state);
        assert_ok!(create_report_result, ());
    });
}

#[test]
fn it_works_for_create_new_annual_report_multiple_annual_reports_gold_standard() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();

        let report_id = create_annual_report_file(owner);
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let create_second_report_result = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_ne!(create_second_report_result, DispatchResult::Ok(()));
        assert_eq!(project.annual_reports.len() + 1, project_with_report.annual_reports.len());
    });
}

#[test]
fn it_fails_for_create_new_annual_report_empty_name_gold_standard() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let results = vec![
            CarbonCredits::create_annual_report(
                Origin::signed(owner), project_id, create_annual_report_file(owner), TEST_CARBON_CREDITS_COUNT,
                Vec::new() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
            ),
            CarbonCredits::create_annual_report(
                Origin::signed(owner), project_id, create_annual_report_file(owner), TEST_CARBON_CREDITS_COUNT,
                get_test_carbon_credits_name(), Vec::new(), TEST_CARBON_CREDITS_DECIMAL
            )
        ];

        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());

        results.iter().for_each(|create_report_result| {
            assert_noop!(
                *create_report_result,
                RuntimeError::BadMetadataParameters
            );
        })
    });
}

#[test]
fn it_fails_for_create_new_annual_report_no_file() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let unexisting_file_id = [1,2,3,4,5,6,7,8,9,1,1,1,1,1,1,1];

        let create_report_result = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, unexisting_file_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());
        assert_noop!(
            create_report_result,
            RuntimeError::AccountNotFileOwner
        );
    });
}

#[test]
fn it_fails_for_create_new_annual_report_not_file_owner() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let auditor = ROLES[2].0;
        let unexisting_file_id = create_annual_report_file(auditor);

        let create_report_result = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, unexisting_file_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());
        assert_noop!(
            create_report_result,
            RuntimeError::AccountNotFileOwner
        );
    });
}

#[test]
fn it_fails_for_create_new_annual_report_gold_standard_not_registered() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let standard = Standard::GOLD_STANDARD;
        let project_id = 1;
        let report_id = create_annual_report_file(owner);

        let mut report_results = Vec::with_capacity(4);
        let mut projects = Vec::with_capacity(4);
    
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        report_results.push(CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        ));
        projects.push(CarbonCredits::get_proj_by_id(1).unwrap());

        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
        report_results.push(CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        ));
        projects.push(CarbonCredits::get_proj_by_id(1).unwrap());

        let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);
        report_results.push(CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        ));
        projects.push(CarbonCredits::get_proj_by_id(1).unwrap());

        let _ = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);
        report_results.push(CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        ));
        projects.push(CarbonCredits::get_proj_by_id(1).unwrap());

        // assertion after all steps
        report_results.iter().for_each(|res|{
            assert_ne!(*res, DispatchResult::Ok(()));
        });
        projects.iter().for_each(|proj|{
            assert_eq!(0, proj.annual_reports.len());
        });
    });
}

#[test]
fn it_fails_for_create_new_annual_report_not_an_owner_role_gold_standard() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);

        ROLES.iter()
            .filter(|x| x.1 != CC_PROJECT_OWNER_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let create_report_result = CarbonCredits::create_annual_report(
                    Origin::signed(x), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
                    get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
                );
                let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

                assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());
                assert_ne!(create_report_result, DispatchResult::Ok(()));
                assert_noop!(
                    create_report_result,
                    RuntimeError::AccountNotOwner
                );
            });
    });
}

#[test]
fn it_fails_for_create_new_annual_report_not_an_owner_of_the_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let (project, project_id, _) = get_registerd_project_and_owner_gold_standard();
        

        // Create new acc with owner role
        let new_owner_id = create_user_with_owner_role();
        let report_id = create_annual_report_file(new_owner_id);
        let is_owner = EvercityAccounts::account_is_cc_project_owner(&new_owner_id);

        let create_report_result = CarbonCredits::create_annual_report(
            Origin::signed(new_owner_id), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert!(is_owner);
        assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());
        assert_ne!(create_report_result, DispatchResult::Ok(()));
        assert_noop!(
            create_report_result,
            RuntimeError::AccountNotOwner
        );
    });
}


#[test]
fn it_works_annual_report_assign_signer() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();

        let report_id = create_annual_report_file(owner);
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        ); 

        let assign_results = vec![
            CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id),
            CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id),
            CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id),
            CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id)
        ];

        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assign_results.iter().for_each(|result| {
                assert_ok!(*result, ());
            }
        );
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[1].0, ROLES[1].1)));
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[2].0, ROLES[2].1)));
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[3].0, ROLES[3].1)));
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[5].0, ROLES[5].1)));
    });
}

#[test]
fn it_works_annual_report_remove_signer() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();

        let report_id = create_annual_report_file(owner);
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        ); 

        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);

        let delete_result = CarbonCredits::remove_last_annual_report_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_ok!(delete_result, ());
        assert!(!project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[5].0, ROLES[5].1)));

        // Assert that others are not deleted:
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[1].0, ROLES[1].1)));
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[2].0, ROLES[2].1)));
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[3].0, ROLES[3].1)));
    });
}

#[test]
fn it_fails_annual_report_remove_unexisting_signer() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();

        let report_id = create_annual_report_file(owner);
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        ); 

        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);

        let delete_result = CarbonCredits::remove_last_annual_report_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_noop!(delete_result, RuntimeError::AccountNotGivenRoleSigner);

        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[1].0, ROLES[1].1)));
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[2].0, ROLES[2].1)));
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[3].0, ROLES[3].1)));
    });
}

#[test]
fn it_fails_annual_report_remove_signed_signer() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let auditor = ROLES[2].0;

        let report_id = create_annual_report_file(owner);
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        ); 

        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);

        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(owner), project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(auditor), project_id);

        let delete_result = CarbonCredits::remove_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, project_id);
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_noop!(delete_result, RuntimeError::AccountAlreadySigned);

        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[1].0, ROLES[1].1)));
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[2].0, ROLES[2].1)));
        assert!(project_with_report.annual_reports.last().unwrap().is_required_signer((ROLES[3].0, ROLES[3].1)));
    });
}


// Project Owner sends report for verification =>  Auditor provides and submits verification report => 
// Standard Approves carbon credit issuance => Registry issues carbon credits
#[test]
fn it_works_for_full_cycle_sign_annual_report_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;
        let report_id = create_annual_report_file(owner);

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        crate::tests::helpers::assign_annual_report_mock_users_required_signers_gold_standard(project_id);

        let tuple_vec = vec![
            (owner, REPORT_AUDITOR_SIGN_PENDING),
            (auditor, REPORT_STANDARD_SIGN_PENDING),
            (standard_acc, REPORT_REGISTRY_SIGN_PENDING),
            (registry, REPORT_ISSUED)
        ];

        tuple_vec.iter()
            .map(|account_state_tuple| {
                let acc = account_state_tuple.0;
                let state = account_state_tuple.1;
                let result = CarbonCredits::sign_last_annual_report(Origin::signed(acc), project_id);

                (acc, state, result)
            })
            .for_each(|account_state_result_tuple|{
                let acc = account_state_result_tuple.0;
                let state = account_state_result_tuple.1;
                let result = account_state_result_tuple.2;
                let project = CarbonCredits::get_proj_by_id(project_id).unwrap();

                assert!(EvercityFilesign::address_has_signed_the_file(report_id, &acc));
                assert_ok!(result, ());
                assert_eq!(state, project.annual_reports.last().unwrap().state);
            })
    });
}

#[test]
fn it_fails_sign_annual_report_not_an_owner_of_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL 
        );
        crate::tests::helpers::assign_annual_report_mock_users_required_signers_gold_standard(project_id);
        
        // Create new acc with owner role
        let new_owner_id = create_user_with_owner_role();
        let is_owner = EvercityAccounts::account_is_cc_project_owner(&new_owner_id);
        let owner_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(new_owner_id), 1);

        assert!(is_owner);
        assert_ne!(owner_sign_result, DispatchResult::Ok(()));
    });
}

#[test]
fn it_fails_sign_annual_report_not_an_owner_role_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );

        ROLES.iter()
        .filter(|x| x.1 != CC_PROJECT_OWNER_ROLE_MASK)
        .map(|x| x.0)
        .for_each(|x| {
            let owner_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(x), 1);
            assert_ne!(owner_sign_result, DispatchResult::Ok(()));
        });
    });
}

#[test]
fn it_fails_sign_annual_report_owner_not_in_signers_gold_standard() {
    // OWNER NOT IN SIGNERS
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let owner_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(owner), 1);
        assert_noop!(
            owner_sign_result,
            RuntimeError::IncorrectAnnualReportSigner
        );
    });
}

#[test]
fn it_fails_sign_annual_report_auditor_not_in_signers_gold_standard() {
    // AUDITOR NOT IN SIGNERS
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, project_id);
        let _owner_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(owner), 1);
        let auditor = ROLES[2].0;

        let auditor_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(auditor), 1);
        assert_noop!(
            auditor_sign_result,
            RuntimeError::IncorrectAnnualReportSigner
        );
    });
}

#[test]
fn it_fails_sign_annual_report_standard_not_in_signers_gold_standard() {
    // STANDARD NOT IN SIGNERS
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, project_id);

        let _owner_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(owner), 1);
        let _auditor_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(auditor), 1);

        let standard_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(standard_acc), 1);

        assert_noop!(
            standard_sign_result,
            RuntimeError::IncorrectAnnualReportSigner
        );
    });
}

#[test]
fn it_fails_sign_annual_report_registry_not_in_signers_gold_standard() {
    // REGISTRY NOT IN SIGNERS
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;
        
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), standard_acc, ROLES[3].1, project_id);

        let _owner_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(owner), 1);
        let _auditor_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(auditor), 1);
        let _standard_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(standard_acc), 1);

        let registry_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(registry), 1);
        assert_noop!(
            registry_sign_result,
            RuntimeError::IncorrectAnnualReportSigner
        );
    });
}

#[test]
fn it_fails_sign_annual_report_not_an_auditor_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        crate::tests::helpers::assign_annual_report_mock_users_required_signers_gold_standard(project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(owner), project_id);


        ROLES.iter()
            .filter(|x| x.1 != CC_AUDITOR_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let auditor_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(x), project_id);
                assert_ne!(auditor_sign_result, DispatchResult::Ok(()));
            });

        let signatures_len = EvercityFilesign::get_file_by_id(report_id)
                .unwrap()
                .versions.last()
                .unwrap()
                .signatures.len();

        assert_eq!(1, signatures_len);
    });
}

#[test]
fn it_fails_sign_annual_report_not_a_standard_role_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let auditor = ROLES[2].0;

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        crate::tests::helpers::assign_annual_report_mock_users_required_signers_gold_standard(project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(owner), project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(auditor), project_id);

        ROLES.iter()
            .filter(|x| x.1 != CC_STANDARD_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let standard_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(x), project_id);
                assert_ne!(standard_sign_result, DispatchResult::Ok(()));
            });

        let signatures_len = EvercityFilesign::get_file_by_id(report_id)
                .unwrap()
                .versions.last()
                .unwrap()
                .signatures.len();

        assert_eq!(2, signatures_len);
    });
}

#[test]
fn it_fails_sign_annual_report_not_an_registry_role_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;

        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), standard_acc, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), registry, ROLES[5].1, project_id);

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        crate::tests::helpers::assign_annual_report_mock_users_required_signers_gold_standard(project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(owner), project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(auditor), project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(standard_acc), project_id);

        ROLES.iter()
            .filter(|x| x.1 != CC_REGISTRY_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(x), project_id);
                assert_ne!(sign_result, DispatchResult::Ok(()));
            });

        let signatures_len = EvercityFilesign::get_file_by_id(report_id)
                .unwrap()
                .versions.last()
                .unwrap()
                .signatures.len();

        assert_eq!(3, signatures_len);
    });
}

#[test]
fn it_fails_sign_annual_report_already_issued_gold_standard() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );

        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), standard_acc, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), registry, ROLES[5].1, project_id);

        let result_vec = vec![
            CarbonCredits::sign_last_annual_report(Origin::signed(owner), project_id),
            CarbonCredits::sign_last_annual_report(Origin::signed(auditor), project_id),
            CarbonCredits::sign_last_annual_report(Origin::signed(standard_acc), project_id),
            CarbonCredits::sign_last_annual_report(Origin::signed(registry), project_id)
        ];

        result_vec.iter().for_each(|res|{
            assert_ok!(res);
        });

        ROLES.iter()
            .map(|x| x.0)
            .for_each(|x| {
                let sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(x), project_id);
                assert_ne!(sign_result, DispatchResult::Ok(()));
            });
    });
}

#[test]
fn it_works_change_report_carbon_credits_count() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let new_carbon_credits_count = 555666;
        let change_count_result = CarbonCredits::change_report_carbon_credits_count(Origin::signed(owner), project_id, new_carbon_credits_count);

        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project_with_report.annual_reports.last().unwrap().carbon_credits_count(), new_carbon_credits_count);
        assert_ok!(change_count_result, ());
    });
}

#[test]
fn it_fails_change_report_carbon_credits_count_last_annual_report_not_owner() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);

        // Create new acc with owner role
        let new_owner_id = create_user_with_owner_role();

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let new_carbon_credits_count = 555666;
        let change_count_result = CarbonCredits::change_report_carbon_credits_count(Origin::signed(new_owner_id), project_id, new_carbon_credits_count);
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project_with_report.annual_reports.last().unwrap().carbon_credits_count(), TEST_CARBON_CREDITS_COUNT);
        assert_noop!(change_count_result, RuntimeError::AccountNotOwner);
    });
}


#[test]
fn it_fails_change_report_carbon_credits_count_last_annual_report_not_correct_state() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), standard_acc, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), registry, ROLES[5].1, project_id);

        let new_carbon_credits_count = 555666;
        let mut change_count_results = Vec::new();

        let _owner_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(owner), project_id);
        change_count_results.push(CarbonCredits::change_report_carbon_credits_count(Origin::signed(owner), project_id, new_carbon_credits_count));
        let _auditor_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(auditor), project_id);
        change_count_results.push(CarbonCredits::change_report_carbon_credits_count(Origin::signed(owner), project_id, new_carbon_credits_count));
        let _auditor_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(standard_acc), project_id);
        change_count_results.push(CarbonCredits::change_report_carbon_credits_count(Origin::signed(owner), project_id, new_carbon_credits_count));
        let _auditor_sign_result = CarbonCredits::sign_last_annual_report(Origin::signed(registry), project_id);
        change_count_results.push(CarbonCredits::change_report_carbon_credits_count(Origin::signed(owner), project_id, new_carbon_credits_count));
        
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project_with_report.annual_reports.last().unwrap().carbon_credits_count(), TEST_CARBON_CREDITS_COUNT);
        change_count_results.iter().for_each(|res|{
            assert_noop!(*res, RuntimeError::InvalidState);
        });
        
    });
}

// Delete tests:
#[test]
fn it_works_delete_last_annual_report() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), standard_acc, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), registry, ROLES[5].1, project_id);

        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(owner), project_id);
        let result = CarbonCredits::delete_last_annual_report(Origin::signed(owner), project_id);

        assert_ok!(result, ());
    });
}

#[test]
fn it_fails_delete_issued_annual_report() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );

        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), standard_acc, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), registry, ROLES[5].1, project_id);

        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(owner), project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(auditor), project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(standard_acc), project_id);
        let _ = CarbonCredits::sign_last_annual_report(Origin::signed(registry), project_id);

        let result = CarbonCredits::delete_last_annual_report(Origin::signed(owner), project_id);
        assert_noop!(result, RuntimeError::InvalidState);
    });
}

#[test]
fn it_fails_delete_last_annual_report_not_project_owner() {
    new_test_ext().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);

        // Create new acc with owner role
        let new_owner_id = create_user_with_owner_role();

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let result = CarbonCredits::delete_last_annual_report(Origin::signed(new_owner_id), project_id);

        assert_noop!(result, RuntimeError::AccountNotOwner);
    });
}

#[test]
fn it_works_for_create_new_annual_report_deposit_event_gold_standard() {
    new_test_ext_with_event().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_id = create_annual_report_file(owner);
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        let last_event = last_event().unwrap();
        let check_event = Event::pallet_carbon_credits(crate::RawEvent::AnnualReportCreated(owner, project_id));

        assert_eq!(check_event, last_event);
    });
}

#[test]
fn it_works_sign_annual_report_deposit_events_gold_standard() {
    new_test_ext_with_event().execute_with(|| {
        let (_, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;
        let report_id = create_annual_report_file(owner);

        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), project_id, report_id, TEST_CARBON_CREDITS_COUNT,
            get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
        );
        crate::tests::helpers::assign_annual_report_mock_users_required_signers_gold_standard(project_id);

        let tuple_vec = vec![
            (owner, Event::pallet_carbon_credits(crate::RawEvent::AnnualReportSubmited(owner, 1))),
            (auditor, Event::pallet_carbon_credits(crate::RawEvent::AnnualReportSignedByAuditor(auditor, 1))),
            (standard_acc, Event::pallet_carbon_credits(crate::RawEvent::AnnualReportSignedByStandard(standard_acc, 1))),
            (registry, Event::pallet_carbon_credits(crate::RawEvent::AnnualReportSignedByRegistry(registry, 1))),
        ];

        tuple_vec.iter()
            .for_each(|(acc, check_event)|{
                let _ = CarbonCredits::sign_last_annual_report(Origin::signed(*acc), project_id);
                let last_event = last_event().unwrap();
    
                assert_eq!(*check_event, last_event);
            });
    });
}