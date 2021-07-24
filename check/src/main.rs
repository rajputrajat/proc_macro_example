use proc_macro_lib::{make_answer, AnswerFn};

#[derive(AnswerFn)]
struct Struct;

fn main() {
    make_answer!();
    println!("{}", answer());
    println!("{}", answer_derive());
}
