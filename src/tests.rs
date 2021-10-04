use crate::mock::*;
use frame_support::{assert_ok, dispatch::{
    DispatchResult, 
    Vec,
}};


#[test]
fn it_works_for_create_new_file() {
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