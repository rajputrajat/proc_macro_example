use proc_macro_lib::{make_answer, AnswerFn, HelperAttr};

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

fn main() {
    println!("{}", answer(10, 20));
    println!("{}", answer_derive());
}
