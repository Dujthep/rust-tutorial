pub struct Dog;

impl Dog {
    pub fn bark(&self) {
        println!("baf");
    }
}

pub struct Interpreter {
    res: f32,
    input: String,
}

impl std::fmt::Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(response is : {}, input is : {})", self.res, self.input)
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        return Interpreter {
            res: 1.0,
            input: String::from("1.0 + 0.0"),
        };
    }

    pub fn input(&mut self, _input: &str) -> Result<Option<f32>, String> {
        if _input == "A" {
            return Ok(Some(1.0));
        }
        return Err(_input.to_string())
    }
}

pub enum FooBar {
    Foo,
    Bar,
}
