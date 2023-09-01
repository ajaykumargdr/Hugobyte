use my_macro::*;

make_answer!();
make_another!(token);

fn main() {

    println!("{}", answer());
    println!("{}", answer2());

}
