use proc_macro_lib::{make_answer, AnswerFn, HelperAttr};

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

fn main() {
    make_answer!();
    println!("{}", answer());
    println!("{}", answer_derive());
}
