use reflective::Reflective;

#[derive(Reflective)]
struct Foo{
    a: i32,
    b: bool,
    c: String
}

fn main() {

    let f = Foo{
        a: 4,
        b: false,
        c: String::from("foo")
    };

    println!("{}", f.name());

}
