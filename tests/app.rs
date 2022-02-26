#[test]
fn hello_world() {
    let value = true;
    assert!(value == true);
}

#[test]
#[should_panic]
fn test_panic_condition() {
    panic!("Darn it");
}
