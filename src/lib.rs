#![feature(collections)]
#![no_std]


pub extern crate collections;


#[macro_export]
macro_rules! concat_string {
    ($($e:expr),*) => (
        {
            let mut s = $crate::collections::string::String::new();
            $(s.push_str($e);)*
            s
        }
    )
}

#[test]
fn test_concat_string() {
    let s = concat_string!("a", "b", "c");
    assert_eq!(&s, "abc")
}
