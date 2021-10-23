use eva::Eva;

#[test]
fn number_eval() {
    let eva = Eva::new();
    assert_eq!(&eva.eval("1"), &"1");
}

#[test]
fn digit_eval() {
    let eva = Eva::new();
    assert_eq!(&eva.eval("12"), &"12");
}

#[test]
fn string_eval() {
    let eva = Eva::new();
    assert_eq!(&eva.eval("'hello'"), &"hello");
}
