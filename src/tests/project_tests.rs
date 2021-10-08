use crate::mock::*;
use frame_support::{assert_ok, dispatch::{
    DispatchResult, 
    Vec,
}};


#[test]
fn it_works_get_unexisting_project() {
	new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = 1;
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard);
        let option = CarbonCredits::get_proj_by_id(2);

        assert!(option.is_none())
	});
}

#[test]
fn it_works_for_create_new_project() {
	new_test_ext().execute_with(|| {
        let owner = ROLES[1].0;
        let standard = 1;
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard);
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
        let standard = 1;
        let create_project_result = CarbonCredits::create_project(Origin::signed(auditor), standard);
        let project_opt = CarbonCredits::get_proj_by_id(1);

        assert_ne!(create_project_result, DispatchResult::Ok(()));
        assert!(project_opt.is_none())
	});
}