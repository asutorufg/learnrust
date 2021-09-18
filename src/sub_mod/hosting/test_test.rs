#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic]
fn another() {
    panic!("make this test fail");
}

pub fn for_test(a: bool, b: bool) -> bool {
    a & b
}

#[test]
#[should_panic(expected = "enter is")]
fn assert_test() {
    assert!(for_test(true, true), "enter is {},{}", true, true);
    assert!(for_test(false, false), "enter is {} {}", false, false);
}

#[test]
fn result_test() -> Result<(), String> {
    if false {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
