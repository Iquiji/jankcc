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

#[test]
fn simple_recursive_fibonacci_easy() {
    let path = "tests/source_files/fib_easy_blocks.c";

    let expected_output = "fib_easy 27: 196418\n";

    assert_eq!(
        common::execute_and_cleanup_capturing_stdout(path),
        expected_output
    );
}

#[test]
fn simple_recursive_fibonacci_harder_blocks_hacky() {
    let path = "tests/source_files/fib_harder_blocks.c";

    let expected_output = "fib 27: 196418\n";

    assert_eq!(
        common::execute_and_cleanup_capturing_stdout(path),
        expected_output
    );
}
