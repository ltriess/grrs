#[path = "../src/main.rs"] mod main;

use crate::main::match_line;

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    for line in "lorem ipsum\ndolor sit amet".lines() {
        match_line(&line, "lorem", &mut result);
    }
    assert_eq!(result, b"lorem ipsum\n");
}
