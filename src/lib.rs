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
