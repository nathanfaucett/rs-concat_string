#[macro_use]
extern crate concat_string;


#[test]
fn test_concat_string() {
    let s = concat_string!("a", "b", &String::from("c"));
    assert_eq!(&s, "abc")
}
