use crate::mock::*;
use frame_support::{assert_ok, dispatch::{
    DispatchResult, 
    Vec,
}};
use crate::H256;
use crate::standard::Standard;
use pallet_evercity_accounts::accounts::*;


#[test]
fn it_works_get_unexisting_project() {
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
fn it_works_for_create_new_project() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = Standard::default();
        let filehash = H256::from([0x66; 32]);
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard.clone(), filehash);
        let project = CarbonCredits::get_proj_by_id(1).unwrap();

        assert_eq!(owner, project.owner);
        assert_eq!(standard, project.standard);
        assert_eq!(1, project.id);
        assert_ok!(create_project_result, ());
    });
}

#[test]
fn it_fails_for_create_new_project_not_owner_role() {
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
        let owner_sign_result = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let project_after_owner_sign = CarbonCredits::get_proj_by_id(1).unwrap();
        let auditor_sign_result = CarbonCredits::sign_project(Origin::signed(auditor), 1);
        let project_after_auditor_sign = CarbonCredits::get_proj_by_id(1).unwrap();
        let standard_acc_sign_result = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);
        let project_after_standard_sign = CarbonCredits::get_proj_by_id(1).unwrap();
        let registry_acc_sign_result = CarbonCredits::sign_project(Origin::signed(registry), 1);
        let project_after_registry_sign = CarbonCredits::get_proj_by_id(1).unwrap();

        assert_ok!(owner_sign_result, ());
        assert_ok!(auditor_sign_result, ());
        assert_ok!(standard_acc_sign_result, ());
        assert_ok!(registry_acc_sign_result, ());

        assert_eq!(crate::state::AUDITOR_SIGN_PENDING, project_after_owner_sign.state);
        assert_eq!(crate::state::STANDARD_SIGN_PENDING, project_after_auditor_sign.state);
        assert_eq!(crate::state::REGISTRY_SIGN_PENDING, project_after_standard_sign.state);
        assert_eq!(crate::state::REGISTERED, project_after_registry_sign.state);

        assert_eq!(crate::project::ProjectStatus::Registration, project_after_owner_sign.status);
        assert_eq!(crate::project::ProjectStatus::Registration, project_after_auditor_sign.status);
        assert_eq!(crate::project::ProjectStatus::Registration, project_after_standard_sign.status);
        assert_eq!(crate::project::ProjectStatus::Issuance, project_after_registry_sign.status);

        assert!(project_after_registry_sign.signatures.iter().any(|x| *x == owner));
        assert!(project_after_registry_sign.signatures.iter().any(|x| *x == auditor));
        assert!(project_after_registry_sign.signatures.iter().any(|x| *x == standard_acc));
        assert!(project_after_registry_sign.signatures.iter().any(|x| *x == registry));

        assert_eq!(project_after_registry_sign.standard, Standard::GoldStandard);
        assert_eq!(1, project_after_registry_sign.document_versions.len());
        assert_eq!(project_after_registry_sign.document_versions[0].filehash, filehash);
    });
}

#[test]
fn it_fails_sign_project_gold_standard_not_an_owner() {
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
fn it_fails_sign_project_gold_standard_not_an_auditor() {
    new_test_ext().execute_with(|| {
        todo!();
    });
}

#[test]
    fn it_fails_sign_project_gold_standard_not_a_standard_acc() {
    new_test_ext().execute_with(|| {
        todo!();
    });
}

#[test]
fn it_fails_sign_project_gold_standard_not_a_registry() {
    new_test_ext().execute_with(|| {
        todo!();
    });
}