use crate::tests::mock::*;
use frame_support::{assert_ok, assert_noop, dispatch::{
    DispatchResult,
    Vec,
}};
use crate::standard::Standard;
use pallet_evercity_accounts::accounts::*;
use crate::project::*;
use crate::tests::helpers::*;
use crate::Error;

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
fn it_fails_for_create_new_project_not_owner_role_gold_standard() {
    new_test_ext().execute_with(|| {
        let auditor = ROLES[3].0;
        let standard = Standard::default();
        let create_project_result = CarbonCredits::create_project(Origin::signed(auditor), standard, create_project_documentation_file(auditor));
        let project_opt = CarbonCredits::get_proj_by_id(1);

        assert_ne!(create_project_result, DispatchResult::Ok(()));
        assert!(project_opt.is_none());
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
            RuntimeError::AccountNotOwner
        );
    });
}

#[test]
fn it_fails_for_create_new_project_other_owner_file_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let not_existing_file_id = 666;
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard, not_existing_file_id);
        let project_opt = CarbonCredits::get_proj_by_id(1);

        assert_ne!(create_project_result, DispatchResult::Ok(()));
        assert!(project_opt.is_none());
        assert_noop!(
            create_project_result,
            RuntimeError::AccountNotOwner
        );
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

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);

        let mut tuple_vec = Vec::new();
        tuple_vec.push((owner, AUDITOR_SIGN_PENDING, ProjectStatus::REGISTRATION));
        tuple_vec.push((auditor, STANDARD_SIGN_PENDING, ProjectStatus::REGISTRATION));
        tuple_vec.push((standard_acc, REGISTRY_SIGN_PENDING, ProjectStatus::REGISTRATION));
        tuple_vec.push((registry, REGISTERED, ProjectStatus::ISSUANCE));

        // sign here:
        tuple_vec.iter()
            .map(|account_state_tuple| {
                let acc = account_state_tuple.0;
                let state = account_state_tuple.1;
                let result = CarbonCredits::sign_project(Origin::signed(acc), 1);
                let status = account_state_tuple.2.clone();
            
                (acc, state, result, status)
            })
            .for_each(|account_state_result_status_tuple|{
                let acc = account_state_result_status_tuple.0;
                let state = account_state_result_status_tuple.1;
                let result = account_state_result_status_tuple.2;
                let status = account_state_result_status_tuple.3;
                let project = CarbonCredits::get_proj_by_id(1).unwrap();

                assert_ok!(result, ());
                assert_eq!(state, project.state);
                // assert_eq!(acc, *project.signatures.last().unwrap());
                assert_eq!(status, project.status);
            });

        let project_after_registry_sign = CarbonCredits::get_proj_by_id(1).unwrap();    
        assert_eq!(*project_after_registry_sign.get_standard(), Standard::GOLD_STANDARD);
        // assert_eq!(1, project_after_registry_sign.document_versions.len());
        // assert_eq!(project_after_registry_sign.document_versions[0].filehash, filehash);
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

        ROLES.iter()
            .filter(|x| x.1 != CC_PROJECT_OWNER_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let owner_sign_result = CarbonCredits::sign_project(Origin::signed(x), 1);
                assert_ne!(owner_sign_result, DispatchResult::Ok(()));
            });
    });
}

#[test]
fn it_fails_sign_project_not_an_owner_of_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GOLD_STANDARD;

        // Create new acc with owner role
        let new_owner_id = 555;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), new_owner_id, CC_PROJECT_OWNER_ROLE_MASK);
        let is_owner = EvercityAccounts::account_is_cc_project_owner(&new_owner_id);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        let owner_sign_result = CarbonCredits::sign_project(Origin::signed(new_owner_id), 1);

        assert!(is_owner);
        assert_ne!(owner_sign_result, DispatchResult::Ok(()));
    });
}

#[test]
fn it_fails_sign_project_not_an_auditor_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        crate::tests::helpers::assign_project_mock_users_required_signers_gold_standard(1);
        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);

        ROLES.iter()
            .filter(|x| x.1 != CC_AUDITOR_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let auditor_sign_result = CarbonCredits::sign_project(Origin::signed(x), 1);
                assert_ne!(auditor_sign_result, DispatchResult::Ok(()));
            });

        // assert_eq!(1, CarbonCredits::get_proj_by_id(1).unwrap().signatures.len());
    });
}

#[test]
    fn it_fails_sign_project_not_a_standard_acc_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
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

        // assert_eq!(2, CarbonCredits::get_proj_by_id(1).unwrap().signatures.len());
    });
}

#[test]
fn it_fails_sign_project_not_a_registry_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let standard = Standard::GOLD_STANDARD;

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
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
        
        // assert_eq!(3, CarbonCredits::get_proj_by_id(1).unwrap().signatures.len());
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
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard.clone(), create_project_documentation_file(owner));
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

        let mut tuple_vec = Vec::new();
        tuple_vec.push((owner, Event::pallet_carbon_credits(crate::RawEvent::ProjectSubmited(owner, 1))));
        tuple_vec.push((auditor, Event::pallet_carbon_credits(crate::RawEvent::ProjectSignedByAduitor(auditor, 1))));
        tuple_vec.push((standard_acc, Event::pallet_carbon_credits(crate::RawEvent::ProjectSignedByStandard(standard_acc, 1))));
        tuple_vec.push((registry, Event::pallet_carbon_credits(crate::RawEvent::ProjectSignedByRegistry(registry, 1))));

        // sign here:
        tuple_vec.iter()
        .for_each(|(acc, check_event)| {
            let _ = CarbonCredits::sign_project(Origin::signed(*acc), 1);
            let last_event = last_event().unwrap();

            assert_eq!(*check_event, last_event);
        });
    });
}