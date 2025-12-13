use crate::git::Git;
use crate::lua;
use rstest::rstest;

#[rstest]
#[case(include_str!("./test_path_filename_case_1.lua"))]
#[case(include_str!("./test_path_filename_case_2.lua"))]
#[case(include_str!("./test_path_filename_case_3.lua"))]
fn test_path_filename(#[case] module: &str) {
    type TestCase = (Option<String>, Option<String>);

    let state = lua::state::Builder::new()
        .build()
        .expect("The Lua object should be valid");
    let lua = state.to_inner();
    let chunk = lua.load(module);

    let (actual, expected): TestCase = chunk.call(()).expect("Chunk should run");
    assert_eq!(expected, actual);
}

#[rstest]
#[case(include_str!("./test_path_glob_matches_case_1.lua"))]
#[case(include_str!("./test_path_glob_matches_case_2.lua"))]
fn test_path_glob_matches(#[case] module: &str) {
    type TestCase = (bool, bool);

    let state = lua::state::Builder::new()
        .build()
        .expect("The Lua object should be valid");
    let lua = state.to_inner();
    let chunk = lua.load(module);

    let (actual, expected): TestCase = chunk.call(()).expect("Chunk should run");
    assert_eq!(expected, actual);
}

#[rstest]
#[case(include_str!("./test_git_is_ignored_case_1.lua"))]
#[case(include_str!("./test_git_is_ignored_case_1.lua"))]
fn test_git_is_ignored(#[case] module: &str) {
    type TestCase = (bool, bool);

    // NOTE This runs on this project's own repository, and so assumes a valid repository
    //      state.
    // NOTE We'll ignore if the repository isn't valid because it might be the test is
    //      running on a "non-git" copy of the code.
    let git = match Git::new(env!("CARGO_MANIFEST_DIR")) {
        Ok(Some(git)) => git,
        Ok(None) => {
            eprintln!("Probably not a git repository");
            return;
        }
        Err(e) => {
            eprintln!("Probably not a git repository: {e}");
            return;
        }
    };

    let state = lua::state::Builder::new()
        .with_git(&git)
        .build()
        .expect("The lua object should be valid");
    let lua = state.to_inner();
    let chunk = lua.load(module);

    state
        .in_git_scope(|| {
            let (actual, expected): TestCase = chunk.call(()).expect("Chunk should run");
            assert_eq!(expected, actual);
            Ok(())
        })
        .expect("Lua-scoped function should succeed");
}
