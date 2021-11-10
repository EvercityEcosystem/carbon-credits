use crate::tests::mock::*;
use pallet_evercity_filesign::{file::H256, FileId};
use crate::standard::Standard;
use crate::project::{ProjectId, ProjectStruct};
use crate::annual_report::*;

pub const TEST_CARBON_CREDITS_COUNT: u64 = 15000;

pub fn create_project_documentation_file(account: u64) -> FileId {
    let filehash = H256::from([0x66; 32]);
    EvercityFilesign::create_new_file(Origin::signed(account), "my_project_documentation".to_owned().as_bytes().to_vec(), filehash);
    1
}

pub fn create_annual_report_file(account: u64) -> FileId {
    let filehash = H256::from([0x88; 32]);
    EvercityFilesign::create_new_file(Origin::signed(account), "my_annual_report".to_owned().as_bytes().to_vec(), filehash);
    2
}

/// Return tuple -> (project, project_id, project_owner)
pub(crate) fn get_registerd_project_and_owner_gold_standard() -> (ProjectStruct<u64, TestRuntime, Balance>, ProjectId, u64) {
    let owner = ROLES[1].0;
    let auditor = ROLES[2].0;
    let standard_acc = ROLES[3].0;
    let registry = ROLES[5].0;
    let standard = Standard::GOLD_STANDARD;
    // let filehash = H256::from([0x66; 32]);

    let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
    assign_project_mock_users_required_signers_gold_standard(1);

    let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
    let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);
    let _ = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);
    let _ = CarbonCredits::sign_project(Origin::signed(registry), 1);
    let project = CarbonCredits::get_proj_by_id(1).unwrap();

    (project, 1, owner)
}


pub(crate) fn full_sign_annual_report_gold_standard() -> (ProjectStruct<u64, TestRuntime, Balance>, ProjectId, u64) {
    let (project, proj_id, owner) = get_registerd_project_and_owner_gold_standard();
    let auditor = ROLES[2].0;
    let standard_acc = ROLES[3].0;
    let registry = ROLES[5].0;
    let report_hash = H256::from([0x69; 32]);

    let _ = CarbonCredits::create_annual_report(Origin::signed(owner), proj_id, create_annual_report_file(owner), TEST_CARBON_CREDITS_COUNT);
    assign_annual_report_mock_users_required_signers_gold_standard(proj_id);

    let mut tuple_vec = Vec::new();
    tuple_vec.push((owner, REPORT_AUDITOR_SIGN_PENDING));
    tuple_vec.push((auditor, REPORT_STANDARD_SIGN_PENDING));
    tuple_vec.push((standard_acc, REPORT_REGISTRY_SIGN_PENDING));
    tuple_vec.push((registry, REPORT_ISSUED));

    tuple_vec.iter()
        .map(|account_state_tuple| {
            let acc = account_state_tuple.0;
            let state = account_state_tuple.1;
            let result = CarbonCredits::sign_last_annual_report(Origin::signed(acc), proj_id);

            (acc, state, result)
        })
        .for_each(|account_state_result_tuple|{
            let _ = account_state_result_tuple.0;
            let _ = account_state_result_tuple.1;
            let _ = account_state_result_tuple.2;
            let _ = CarbonCredits::get_proj_by_id(proj_id).unwrap();
        });

    (project, 1, owner)
}

pub(crate) fn assign_project_mock_users_required_signers_gold_standard(project_id: ProjectId) {
    let owner = ROLES[1].0;
    let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
    let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
    let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);
    let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);
}

pub(crate) fn assign_annual_report_mock_users_required_signers_gold_standard(project_id: ProjectId) {
    let owner = ROLES[1].0;
    let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
    let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
    let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);
    let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);
}