#![warn(dead_code)]
#![allow(unused)]
use proc_macro_lib::{make_answer, show_streams, AnswerFn, HelperAttr};

make_answer!();

#[derive(AnswerFn)]
struct Struct;

#[derive(HelperAttr)]
struct AttrStruct {
    #[helper_1]
    a: i32,
    #[helper_2]
    b: f32,
    #[helper_3]
    c: i8,
}

#[show_streams]
fn invoke1() {}

#[show_streams(bar)]
fn invoke2() {}

#[show_streams(multiple => tokens)]
fn invoke3() {}

#[show_streams{ delimiters }]
fn invoke4() {}

#[macro_export]
macro_rules! custom_vec {
    ( $( $i: expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push( $i ); )*
            v
        }
    };
}

fn main() {
    println!("{}", answer(10, 20));
    println!("{}", answer_derive());

    let int_vec = custom_vec![1, 2, 3];
    println!("{:#?}", int_vec);
}
