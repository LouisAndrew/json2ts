#[test]
fn check_get_matches() {
    let buffer = "Hello\nWorld\nHello\n".as_bytes();
    let matches = json2ts::get_matches(buffer, "Hello");
    assert_eq!(matches.len(), 2);
    assert_eq!(matches, vec!["Hello", "Hello"]);
}
