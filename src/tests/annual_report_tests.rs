use crate::tests::mock::*;
use frame_support::{assert_ok, dispatch::{
    DispatchResult
}};
use crate::H256;
use crate::standard::Standard;
use crate::project;
use crate::annual_report::*;
use pallet_evercity_accounts::accounts::*;


/// Return tuple -> (project, project_id, project_owner)
fn get_registerd_project_and_owner_gold_standard() -> (project::ProjectStruct<u64>, u32, u64) {
    let owner = ROLES[1].0;
    let auditor = ROLES[2].0;
    let standard_acc = ROLES[3].0;
    let registry = ROLES[5].0;
    let standard = Standard::GoldStandard;
    let filehash = H256::from([0x66; 32]);

    let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);
    let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
    let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);
    let _ = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);
    let _ = CarbonCredits::sign_project(Origin::signed(registry), 1);
    let project = CarbonCredits::get_proj_by_id(1).unwrap();

    (project, 1, owner)
}

#[test]
fn it_works_for_create_new_annual_report_gold_standard() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_hash = H256::from([0x69; 32]);

        let create_report_result = CarbonCredits::create_annual_report(Origin::signed(owner), project_id, report_hash);
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_eq!(project.annual_reports.len() + 1, project_with_report.annual_reports.len());
        assert_eq!(report_hash, project_with_report.annual_reports.last().unwrap().filehash);
        assert_eq!(REPORT_PROJECT_OWNER_SIGN_PENDING, project_with_report.annual_reports.last().unwrap().state);
        assert_ok!(create_report_result, ());
    });
}
#[test]
fn it_works_for_create_new_annual_report_multiple_annual_reports_gold_standard() {
    new_test_ext().execute_with(|| {
        let (project, project_id, owner) = get_registerd_project_and_owner_gold_standard();
        let report_hash = H256::from([0x69; 32]);

        let _ = CarbonCredits::create_annual_report(Origin::signed(owner), project_id, report_hash);
        let create_second_report_result = CarbonCredits::create_annual_report(Origin::signed(owner), project_id, report_hash);
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert_ne!(create_second_report_result, DispatchResult::Ok(()));
        assert_eq!(project.annual_reports.len() + 1, project_with_report.annual_reports.len());
    });
}

#[test]
fn it_fails_for_create_new_annual_report_gold_standard_not_registered() {
    new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let standard = Standard::GoldStandard;
        let filehash = H256::from([0x66; 32]);
        let report_hash = H256::from([0x69; 32]);
        let project_id = 1;

        let mut report_results = Vec::with_capacity(4);
        let mut projects = Vec::with_capacity(4);
    
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, filehash);
        report_results.push(CarbonCredits::create_annual_report(Origin::signed(owner), project_id, report_hash));
        projects.push(CarbonCredits::get_proj_by_id(1).unwrap());

        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
        report_results.push(CarbonCredits::create_annual_report(Origin::signed(owner), project_id, report_hash));
        projects.push(CarbonCredits::get_proj_by_id(1).unwrap());

        let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);
        report_results.push(CarbonCredits::create_annual_report(Origin::signed(owner), project_id, report_hash));
        projects.push(CarbonCredits::get_proj_by_id(1).unwrap());

        let _ = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);
        report_results.push(CarbonCredits::create_annual_report(Origin::signed(owner), project_id, report_hash));
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
        let (project, project_id, _) = get_registerd_project_and_owner_gold_standard();
        let report_hash = H256::from([0x69; 32]);

        ROLES.iter()
            .filter(|x| x.1 != CC_PROJECT_OWNER_ROLE_MASK)
            .map(|x| x.0)
            .for_each(|x| {
                let create_report_result = CarbonCredits::create_annual_report(Origin::signed(x), project_id, report_hash);
                let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

                assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());
                assert_ne!(create_report_result, DispatchResult::Ok(()));
            });
    });
}

#[test]
fn it_fails_for_create_new_annual_report_not_an_owner_of_the_project_gold_standard() {
    new_test_ext().execute_with(|| {
        let (project, project_id, _) = get_registerd_project_and_owner_gold_standard();
        let report_hash = H256::from([0x69; 32]);

        // Create new acc with owner role
        let new_owner_id = 555;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), new_owner_id, CC_PROJECT_OWNER_ROLE_MASK);
        let is_owner = EvercityAccounts::account_is_cc_project_owner(&new_owner_id);

        let create_report_result = CarbonCredits::create_annual_report(Origin::signed(new_owner_id), project_id, report_hash);
        let project_with_report = CarbonCredits::get_proj_by_id(project_id).unwrap();

        assert!(is_owner);
        assert_eq!(project.annual_reports.len(), project_with_report.annual_reports.len());
        assert_ne!(create_report_result, DispatchResult::Ok(()));
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
        let report_hash = H256::from([0x69; 32]);

        let _ = CarbonCredits::create_annual_report(Origin::signed(owner), project_id, report_hash);

        vec![
            (owner, REPORT_AUDITOR_SIGN_PENDING), 
            (auditor, REPORT_STANDARD_SIGN_PENDING),
            (standard_acc, REPORT_REGISTRY_SIGN_PENDING), 
            (registry, REPORT_ISSUED),
        ].iter()
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

                assert_ok!(result, ());
                assert_eq!(acc, *project.annual_reports.last().unwrap().signatures.last().unwrap());
                assert_eq!(state, project.annual_reports.last().unwrap().state);
            });
    });
}

