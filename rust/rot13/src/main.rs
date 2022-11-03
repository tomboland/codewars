fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| match c as u8 {
            (b'a'..=b'z') => (b'a' + (c as u8 - b'a' + 13) % 26) as char,
            (b'A'..=b'Z') => (b'A' + (c as u8 - b'A' + 13) % 26) as char,
            _ => c,
        })
        .collect()
}

const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

fn dotest(s: &str, expected: &str) {
    assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
}

#[test]
fn sample_tests() {
    dotest("test", "grfg");
    dotest("Test", "Grfg");
}
