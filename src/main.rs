use tutorial::FooBar;
use tutorial::Dog;

fn main() {
    let dog = Dog;
    dog.bark();
    
    let foobar: Option<FooBar> = Some (FooBar::Bar);
    match foobar {
        Some(FooBar::Foo) => { println!("Foo") }
        Some(FooBar::Bar) => { println!("Bar") }
        _ => { println!("None") }
    }
}