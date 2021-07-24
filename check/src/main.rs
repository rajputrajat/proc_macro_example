use proc_macro_lib::make_answer;

fn main() {
    make_answer!();
    println!("{}", answer());
}
