use crate::tests::mock::*;
use frame_support::{assert_ok, dispatch::{
    DispatchResult
}};
use crate::H256;
use crate::standard::Standard;
use pallet_evercity_accounts::accounts::*;
use crate::project::*;


#[test]
fn it_works_get_unexisting_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let filehash = H256::from([0x66; 32]);
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);
        let option = CarbonCredits::get_proj_by_id(2);
        assert!(option.is_none())
    });
}

#[test]
fn it_works_for_create_new_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let filehash = H256::from([0x66; 32]);
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard.clone(), filehash);
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
        let filehash = H256::from([0x66; 32]);
        let create_project_result = CarbonCredits::create_project(Origin::signed(auditor), standard, filehash);
        let project_opt = CarbonCredits::get_proj_by_id(1);

        assert_ne!(create_project_result, DispatchResult::Ok(()));
        assert!(project_opt.is_none())
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

        let standard = Standard::GoldStandard;
        let filehash = H256::from([0x66; 32]);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);

        // sign here:
        vec![
            (owner, AUDITOR_SIGN_PENDING, ProjectStatus::Registration), 
            (auditor, STANDARD_SIGN_PENDING, ProjectStatus::Registration),
            (standard_acc, REGISTRY_SIGN_PENDING, ProjectStatus::Registration), 
            (registry, REGISTERED, ProjectStatus::Issuance),
        ].iter()
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
                assert_eq!(acc, *project.signatures.last().unwrap());
                assert_eq!(status, project.status);
            });

        let project_after_registry_sign = CarbonCredits::get_proj_by_id(1).unwrap();    
        assert_eq!(*project_after_registry_sign.get_standard(), Standard::GoldStandard);
        assert_eq!(1, project_after_registry_sign.document_versions.len());
        assert_eq!(project_after_registry_sign.document_versions[0].filehash, filehash);
        assert_eq!(0, project_after_registry_sign.annual_reports.len());
    });
}

#[test]
fn it_fails_sign_project_gold_standard_not_an_owner_role_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GoldStandard;
        let filehash = H256::from([0x66; 32]);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);

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
fn it_fails_sign_project_gold_standard_not_an_owner_of_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GoldStandard;
        let filehash = H256::from([0x66; 32]);

        // Create new acc with owner role
        let new_owner_id = 555;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), new_owner_id, CC_PROJECT_OWNER_ROLE_MASK);
        let is_owner = EvercityAccounts::account_is_cc_project_owner(&new_owner_id);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);
        let owner_sign_result = CarbonCredits::sign_project(Origin::signed(new_owner_id), 1);

        assert!(is_owner);
        assert_ne!(owner_sign_result, DispatchResult::Ok(()));
    });
}

#[test]
fn it_fails_sign_project_gold_standard_not_an_auditor_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::GoldStandard;
        let filehash = H256::from([0x66; 32]);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);
        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);

        ROLES.iter()
            .filter(|x| x.1 != CC_AUDITOR_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let auditor_sign_result = CarbonCredits::sign_project(Origin::signed(x), 1);
                assert_ne!(auditor_sign_result, DispatchResult::Ok(()));
            });

        assert_eq!(1, CarbonCredits::get_proj_by_id(1).unwrap().signatures.len());
    });
}

#[test]
    fn it_fails_sign_project_gold_standard_not_a_standard_acc_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard = Standard::GoldStandard;
        let filehash = H256::from([0x66; 32]);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);
        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);

        ROLES.iter()
            .filter(|x| x.1 != CC_STANDARD_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let standard_sign_result = CarbonCredits::sign_project(Origin::signed(x), 1);
                assert_ne!(standard_sign_result, DispatchResult::Ok(()));
            });

        assert_eq!(2, CarbonCredits::get_proj_by_id(1).unwrap().signatures.len());
    });
}

#[test]
fn it_fails_sign_project_gold_standard_not_a_registry_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let standard = Standard::GoldStandard;
        let filehash = H256::from([0x66; 32]);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);
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
        
        assert_eq!(3, CarbonCredits::get_proj_by_id(1).unwrap().signatures.len());
    });
}

#[test]
fn it_fails_sign_project_gold_standard_already_registered_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;
        let standard = Standard::GoldStandard;
        let filehash = H256::from([0x66; 32]);
        let some_new_acc = 7;

        let all_roles = ROLES.iter().map(|x| x.1).reduce(|x, y| x + y).unwrap();
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(0), some_new_acc, all_roles);

        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);
        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(registry), 1);

        // check that acc with any role can sign it
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