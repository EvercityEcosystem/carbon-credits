use crate::tests::mock::*;
use frame_support::{assert_ok, assert_noop, dispatch::{
    DispatchResult,
}};
use crate::standard::Standard;
use pallet_evercity_accounts::accounts::*;
use crate::project::*;
use crate::tests::helpers::*;
use crate::Error;
use sp_std::vec;

type RuntimeError = Error<TestRuntime>;

#[test]
fn it_works_get_unexisting_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        let option = CarbonCredits::get_proj_by_id(2);
        assert!(option.is_none())
    });
}

#[test]
fn it_works_for_create_new_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard.clone(), create_project_documentation_file(owner));
        let project = CarbonCredits::get_proj_by_id(1).unwrap();

        assert_eq!(owner, project.owner);
        assert_eq!(standard, *project.get_standard());
        assert_eq!(1, project.id);
        assert_ok!(create_project_result, ());
    });
}

#[test]
fn it_works_for_create_new_project_file_not_specified_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard.clone(), None);
        let project = CarbonCredits::get_proj_by_id(1).unwrap();

        assert_eq!(owner, project.owner);
        assert_eq!(standard, *project.get_standard());
        assert_eq!(1, project.id);
        assert_eq!(None, project.file_id);
        assert_ok!(create_project_result, ());
    });
}

#[test]
fn it_fails_for_create_new_project_not_owner_role_gold_standard() {
    new_test_ext().execute_with(|| {
        let auditor = ROLES[3].0;
        let standard = Standard::default();
        let create_project_result = CarbonCredits::create_project(Origin::signed(auditor), standard, create_project_documentation_file(auditor));
        let project_opt = CarbonCredits::get_proj_by_id(1);

        assert!(project_opt.is_none());
        assert_noop!(
            create_project_result,
            RuntimeError::AccountNotOwner
        );
    });
}

#[test]
fn it_fails_for_create_new_project_no_file_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[3].0;
        let standard = Standard::default();
        let other_owner_file_id = create_project_documentation_file(auditor);
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard, other_owner_file_id);
        let project_opt = CarbonCredits::get_proj_by_id(1);

        assert_ne!(create_project_result, DispatchResult::Ok(()));
        assert!(project_opt.is_none());
        assert_noop!(
            create_project_result,
            RuntimeError::AccountNotFileOwner
        );
    });
}

#[test]
fn it_fails_for_create_new_project_other_owner_file_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let not_existing_file_id = Some([1,2,3,4,5,6,7,8,9,1,1,1,1,1,1,1]);
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard, not_existing_file_id);
        let project_opt = CarbonCredits::get_proj_by_id(1);

        assert_ne!(create_project_result, DispatchResult::Ok(()));
        assert!(project_opt.is_none());
        assert_noop!(
            create_project_result,
            RuntimeError::AccountNotFileOwner
        );
    });
}

#[test]
fn it_works_for_change_file_id() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let not_existing_file_id = None;
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard, not_existing_file_id);
        let project_before_change = CarbonCredits::get_proj_by_id(1);
        let file_id = create_project_documentation_file(owner);
        let change_id_result = CarbonCredits::change_project_file_id(Origin::signed(owner), 1, file_id.unwrap());
        let project_after_change = CarbonCredits::get_proj_by_id(1);

        assert_ok!(create_project_result, ());
        assert_ok!(change_id_result, ());
        assert_eq!(None, project_before_change.unwrap().file_id);
        assert_eq!(file_id, project_after_change.unwrap().file_id);
    });
}

#[test]
fn it_fails_for_change_file_id_not_file_owner() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard = Standard::default();
        let not_existing_file_id = None;
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, not_existing_file_id);
        let project_before_change = CarbonCredits::get_proj_by_id(1);
        let file_id = create_project_documentation_file(auditor);
        let change_id_result = CarbonCredits::change_project_file_id(Origin::signed(owner), 1, file_id.unwrap());
        let project_after_change = CarbonCredits::get_proj_by_id(1);

        assert_noop!(change_id_result, RuntimeError::AccountNotFileOwner);
        assert_eq!(None, project_before_change.unwrap().file_id);
        assert_eq!(None, project_after_change.unwrap().file_id);
    });
}

#[test]
fn it_fails_for_change_file_id_not_owner() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let new_owner_id = create_user_with_owner_role();
        let standard = Standard::default();
        let not_existing_file_id = None;
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, not_existing_file_id);
        let project_before_change = CarbonCredits::get_proj_by_id(1);
        let file_id = create_project_documentation_file(new_owner_id);
        let change_id_result = CarbonCredits::change_project_file_id(Origin::signed(new_owner_id), 1, file_id.unwrap());
        let project_after_change = CarbonCredits::get_proj_by_id(1);

        assert_noop!(change_id_result, RuntimeError::AccountNotOwner);
        assert_eq!(None, project_before_change.unwrap().file_id);
        assert_eq!(None, project_after_change.unwrap().file_id);
    });
}

#[test]
fn it_works_project_assign_signer() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        let project_id = 1;

        let assign_results = vec![
            CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id),
            CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id),
            CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id),
            CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id)
        ];

        let project = CarbonCredits::get_proj_by_id(1).unwrap();

        assign_results.iter().for_each(|result| {
                assert_ok!(*result, ());
            }
        );
        assert!(project.is_required_signer((ROLES[1].0, ROLES[1].1)));
        assert!(project.is_required_signer((ROLES[2].0, ROLES[2].1)));
        assert!(project.is_required_signer((ROLES[3].0, ROLES[3].1)));
        assert!(project.is_required_signer((ROLES[5].0, ROLES[5].1)));
    });
}

#[test]
fn it_works_remove_signer() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        let project_id = 1;

        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);

        let delete_result = CarbonCredits::remove_project_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);
        let project = CarbonCredits::get_proj_by_id(1).unwrap();

        assert_ok!(delete_result, ());

        // assert that deleted:
        assert!(!project.is_required_signer((ROLES[5].0, ROLES[5].1)));

        // Assert other are not deleted:
        assert!(project.is_required_signer((ROLES[1].0, ROLES[1].1)));
        assert!(project.is_required_signer((ROLES[2].0, ROLES[2].1)));
        assert!(project.is_required_signer((ROLES[3].0, ROLES[3].1)));
    });
}

#[test]
fn it_fails_remove_unexisting_signer() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        let project_id = 1;

        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);

        let delete_result = CarbonCredits::remove_project_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);
        let project = CarbonCredits::get_proj_by_id(1).unwrap();

        assert_noop!(delete_result, RuntimeError::AccountNotGivenRoleSigner);

        // Assert other are not deleted:
        assert!(project.is_required_signer((ROLES[1].0, ROLES[1].1)));
        assert!(project.is_required_signer((ROLES[2].0, ROLES[2].1)));
        assert!(project.is_required_signer((ROLES[3].0, ROLES[3].1)));
    });
}


#[test]
fn it_fails_remove_signed_signer() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard = Standard::default();
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        let project_id = 1;

        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);

        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);

        let delete_result = CarbonCredits::remove_project_signer(Origin::signed(owner), auditor, ROLES[2].1, project_id);
        let project = CarbonCredits::get_proj_by_id(1).unwrap();

        assert_noop!(delete_result, RuntimeError::AccountAlreadySigned);

        // Assert signer is not deleted:
        assert!(project.is_required_signer((ROLES[2].0, ROLES[2].1)));
    });
}

// Main flow test
// Project Owner submits PDD (changing status to Registration) => 
// => Auditor Approves PDD => Standard Certifies PDD => Registry Registers PDD (changing status to Issuance)
#[test]
fn it_works_for_full_cycle_sign_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;

        let standard = Standard::GOLD_STANDARD;

        let project_doc_id = create_project_documentation_file(owner);
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, project_doc_id);
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);

        let tuple_vec = vec![
            (owner, AUDITOR_SIGN_PENDING, ProjectStatus::REGISTRATION),
            (auditor, STANDARD_SIGN_PENDING, ProjectStatus::REGISTRATION),
            (standard_acc, REGISTRY_SIGN_PENDING, ProjectStatus::REGISTRATION),
            (registry, REGISTERED, ProjectStatus::ISSUANCE)
        ];

        let file_id = [1,2,3,4,5,6,7,8,9,1,1,1,1,1,1,1];
        let _ = EvercityFilesign::create_new_file(Origin::signed(owner), 
                "my_some_other_file".to_owned().as_bytes().to_vec(),
             pallet_evercity_filesign::file::H256::from([0x96; 32]),
                     Some(file_id)
            );

        // sign here:
        tuple_vec.iter()
            .map(|account_state_tuple| {
                let acc = account_state_tuple.0;
                let state = account_state_tuple.1;
                let result = CarbonCredits::sign_project(Origin::signed(acc), 1);
                let status = account_state_tuple.2.clone();

                // Check, that file id could not be changed
                let change_id_result = CarbonCredits::change_project_file_id(Origin::signed(owner), 1, file_id);
            
                (acc, state, result, status, change_id_result)
            })
            .for_each(|account_state_result_status_tuple|{
                let acc = account_state_result_status_tuple.0;
                let state = account_state_result_status_tuple.1;
                let result = account_state_result_status_tuple.2;
                let status = account_state_result_status_tuple.3;
                let change_id_result = account_state_result_status_tuple.4;
                let project = CarbonCredits::get_proj_by_id(1).unwrap();

                assert_ok!(result, ());
                assert_eq!(state, project.state);
                assert!(EvercityFilesign::address_has_signed_the_file(project_doc_id.unwrap(), &acc));
                assert_eq!(status, project.status);
                assert_noop!(change_id_result, RuntimeError::InvalidState);
            });

        let project_after_registry_sign = CarbonCredits::get_proj_by_id(1).unwrap();    
        assert_eq!(*project_after_registry_sign.get_standard(), Standard::GOLD_STANDARD);
        assert_eq!(0, project_after_registry_sign.annual_reports.len());
    });
}

#[test]
fn it_fails_sign_project_not_an_owner_role_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);
        assign_project_mock_users_required_signers_gold_standard(1);

        // Assign as stub:
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[0].0, MASTER_ROLE_MASK, 1);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[4].0, CC_INVESTOR_ROLE_MASK, 1);


        ROLES.iter()
            .filter(|x| x.1 != CC_PROJECT_OWNER_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let owner_sign_result = CarbonCredits::sign_project(Origin::signed(x), 1);
                assert_noop!(
                    owner_sign_result,
                    RuntimeError::AccountNotOwner
                );
            });
    });
}

#[test]
fn it_fails_sign_project_not_owner_signer_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        let owner_sign_result = CarbonCredits::sign_project(Origin::signed(owner), 1);

        assert_noop!(
            owner_sign_result,
            RuntimeError::IncorrectProjectSigner
        );
    });
}

#[test]
fn it_fails_sign_project_no_file_id() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, None);
        let owner_sign_result = CarbonCredits::sign_project(Origin::signed(owner), 1);

        assert_noop!(
            owner_sign_result,
            RuntimeError::IncorrectFileId
        );
    });
}

#[test]
fn it_fails_sign_project_not_auditor_signer_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));

        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, 1);
        let _owner_sign_result = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let auditor_sign_result = CarbonCredits::sign_project(Origin::signed(auditor), 1);

        assert_noop!(
            auditor_sign_result,
            RuntimeError::IncorrectProjectSigner
        );
    });
}

#[test]
fn it_fails_sign_project_not_standard_signer_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));

        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, 1);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, 1);

        let _owner_sign_result = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let _auditor_sign_result = CarbonCredits::sign_project(Origin::signed(auditor), 1);
        let standard_sign_result = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);

        assert_noop!(
            standard_sign_result,
            RuntimeError::IncorrectProjectSigner
        );
    });
}

#[test]
fn it_fails_sign_project_not_registry_signer_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;
        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));

        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), owner, ROLES[1].1, 1);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), auditor, ROLES[2].1, 1);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), standard_acc, ROLES[3].1, 1);

        let _owner_sign_result = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let _auditor_sign_result = CarbonCredits::sign_project(Origin::signed(auditor), 1);
        let _standard_sign_result = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);
        let registry_sign_result = CarbonCredits::sign_project(Origin::signed(registry), 1);

        assert_noop!(
            registry_sign_result,
            RuntimeError::IncorrectProjectSigner
        );
    });
}

#[test]
fn it_fails_sign_project_not_an_owner_of_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GOLD_STANDARD;

        // Create new acc with owner role
        let new_owner_id = create_user_with_owner_role();
        let is_owner = EvercityAccounts::account_is_cc_project_owner(&new_owner_id);
        
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        let _result = CarbonCredits::assign_project_signer(Origin::signed(owner), new_owner_id, CC_PROJECT_OWNER_ROLE_MASK, 1);
        let owner_sign_result = CarbonCredits::sign_project(Origin::signed(new_owner_id), 1);

        assert!(is_owner);
        assert_noop!(
            owner_sign_result,
            RuntimeError::AccountNotOwner
        );
    });
}

#[test]
fn it_fails_sign_project_not_an_auditor_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GOLD_STANDARD;
        let proj_file_id = create_project_documentation_file(owner);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, proj_file_id);
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);
        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);

        ROLES.iter()
            .filter(|x| x.1 != CC_AUDITOR_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let auditor_sign_result = CarbonCredits::sign_project(Origin::signed(x), 1);
                assert_ne!(auditor_sign_result, DispatchResult::Ok(()));
            });

        let signatures_len = EvercityFilesign::get_file_by_id(proj_file_id.unwrap())
                                                .unwrap()
                                                .versions.last()
                                                .unwrap()
                                                .signatures.len();

        assert_eq!(1, signatures_len);
    });
}

#[test]
fn it_fails_sign_project_not_a_standard_acc_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard = Standard::GOLD_STANDARD;
        let proj_file_id = create_project_documentation_file(owner);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, proj_file_id);
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);
        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);

        ROLES.iter()
            .filter(|x| x.1 != CC_STANDARD_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let standard_sign_result = CarbonCredits::sign_project(Origin::signed(x), 1);
                assert_ne!(standard_sign_result, DispatchResult::Ok(()));
            });

        let signatures_len = EvercityFilesign::get_file_by_id(proj_file_id.unwrap())
                                        .unwrap()
                                        .versions.last()
                                        .unwrap()
                                        .signatures.len();


        assert_eq!(2, signatures_len);
    });
}

#[test]
fn it_fails_sign_project_not_a_registry_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let standard = Standard::GOLD_STANDARD;
        let proj_file_id = create_project_documentation_file(owner);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, proj_file_id);
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);
        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);

        ROLES.iter()
            .filter(|x| x.1 != CC_REGISTRY_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let registry_sign_result = CarbonCredits::sign_project(Origin::signed(x), 1);
                assert_ne!(registry_sign_result, DispatchResult::Ok(()));
            });
        
        let signatures_len = EvercityFilesign::get_file_by_id(proj_file_id.unwrap())
                                        .unwrap()
                                        .versions.last()
                                        .unwrap()
                                        .signatures.len();


        assert_eq!(3, signatures_len);
    });
}

#[test]
fn it_fails_sign_project_already_registered_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;
        let standard = Standard::GOLD_STANDARD;
        let some_new_acc = 7;

        let all_roles = ROLES.iter().map(|x| x.1).reduce(|x, y| x + y).unwrap();
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(0), some_new_acc, all_roles);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);
        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(registry), 1);

        // check that acc with any role cant sign it
        let some_new_acc_sign_result = CarbonCredits::sign_project(Origin::signed(some_new_acc), 1);
        assert_ne!(some_new_acc_sign_result, DispatchResult::Ok(()));

        // check all separate existing roles
        ROLES.iter()
            .map(|x| x.0)
            .for_each(|x| {
                let sign_result = CarbonCredits::sign_project(Origin::signed(x), 1);
                assert_ne!(sign_result, DispatchResult::Ok(()));
            });        
    });
}

#[test]
fn it_works_for_create_new_project_deposit_event_gold_standard() {
    new_test_ext_with_event().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        let last_event = last_event().unwrap();
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);

        let check_event = Event::pallet_carbon_credits(crate::RawEvent::ProjectCreated(owner, 1));

        assert_eq!(check_event, last_event);
    });
}

#[test]
fn it_works_sign_project_deposit_events_gold_standard() {
    new_test_ext_with_event().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;

        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);

        let tuple_vec = vec![
            (owner, Event::pallet_carbon_credits(crate::RawEvent::ProjectSubmited(owner, 1))),
            (auditor, Event::pallet_carbon_credits(crate::RawEvent::ProjectSignedByAduitor(auditor, 1))),
            (standard_acc, Event::pallet_carbon_credits(crate::RawEvent::ProjectSignedByStandard(standard_acc, 1))),
            (registry, Event::pallet_carbon_credits(crate::RawEvent::ProjectSignedByRegistry(registry, 1)))
        ];

        // sign here:
        tuple_vec.iter()
        .for_each(|(acc, check_event)| {
            let _ = CarbonCredits::sign_project(Origin::signed(*acc), 1);
            let last_event = last_event().unwrap();

            assert_eq!(*check_event, last_event);
        });
    });
}