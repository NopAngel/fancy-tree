use crate::lua;
use rstest::rstest;

#[rstest]
#[case(include_str!("./test_path_filename_case_1.lua"))]
#[case(include_str!("./test_path_filename_case_2.lua"))]
#[case(include_str!("./test_path_filename_case_3.lua"))]
fn test_path_filename(#[case] module: &str) {
    type TestCase = (Option<String>, Option<String>);

    let lua = lua::new_state().expect("The Lua object should be valid");
    let chunk = lua.load(module);

    let (actual, expected): TestCase = chunk.call(()).expect("Chunk should run");
    assert_eq!(expected, actual);
}


#[rstest]
#[case(include_str!("./test_path_glob_matches_case_1.lua"))]
#[case(include_str!("./test_path_glob_matches_case_2.lua"))]
fn test_path_glob_matches(#[case] module: &str) {
    type TestCase = (bool, bool);

    let lua = lua::new_state().expect("The Lua object should be valid");
    let chunk = lua.load(module);

    let (actual, expected): TestCase = chunk.call(()).expect("Chunk should run");
    assert_eq!(expected, actual);
}
