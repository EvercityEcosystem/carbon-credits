use crate::mock::*;
use frame_support::{assert_ok, dispatch::{
    DispatchResult, 
    Vec,
}};

#[test]
fn it_works_get_unexisting_project() {
	new_test_ext().execute_with(|| {
        let owner = 3;
        let standard = 1;
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard);
        let option = CarbonCredits::get_proj_by_id(2);

        assert_eq!(true, match option { None => true, Some(_) => false})
	});
}

#[test]
fn it_works_for_create_new_project() {
	new_test_ext().execute_with(|| {
        let owner = 3;
        let standard = 1;
        let create_project_result = CarbonCredits::create_project(Origin::signed(owner), standard);
        let project = CarbonCredits::get_proj_by_id(1).unwrap();

        assert_eq!(owner, project.owner);
        assert_eq!(standard, project.standard);
        assert_eq!(1, project.id);
        assert_ok!(create_project_result, ());
	});
}