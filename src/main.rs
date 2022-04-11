use tutorial::Interpreter;
// use tutorial::FooBar;

fn main() {
    // let foobar: Option<FooBar> = Some (FooBar::Bar);
    // match foobar {
    //     Some(FooBar::Foo) => { println!("Foo") }
    //     Some(FooBar::Bar) => { println!("Bar") }
    //     _ => { println!("None") }
    // }
    
    let mut i = Interpreter {};
    println!("{:?}", i.input("1 + 1 * 2"));
}

#[test]
fn basic_arithmetic() {
    let mut i = Interpreter {};
    assert_eq!(i.input("1 + 1"), Ok(Some(2.0)));
    assert_eq!(i.input("2 - 1"), Ok(Some(1.0)));
    assert_eq!(i.input("2 * 3"), Ok(Some(6.0)));
    assert_eq!(i.input("8 / 4"), Ok(Some(2.0)));
    assert_eq!(i.input("7 % 4"), Ok(Some(3.0)));
    assert_eq!(i.input("1 + 1 * 2"), Ok(Some(3.0)));
}