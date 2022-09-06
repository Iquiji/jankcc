mod common;

#[test]
fn simple_add() {
    let path = "tests/source_files/simple_add.c";

    let expected_output = "4000+777=4777";

    assert_eq!(
        common::execute_and_cleanup_capturing_stdout(path),
        expected_output
    );
}

#[test]
fn simple_hello_world() {
    let path = "tests/source_files/hello_world.c";

    let expected_output = "Hello World!\n";

    assert_eq!(
        common::execute_and_cleanup_capturing_stdout(path),
        expected_output
    );
}
