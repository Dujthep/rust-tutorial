pub struct Dog;

impl Dog {
    pub fn bark(&self) {
        println!("baf");
    }
}

pub struct Interpreter {
    // res: f32,
    // input: String,
}

// impl std::fmt::Display for Interpreter {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "(response is : {}, input is : {})", self.res, self.input)
//     }
// }

impl Interpreter {
    // pub fn new() -> Interpreter {
    //     return Interpreter {
    //         res: 1.0,
    //         input: String::from("1.0 + 0.0"),
    //     };
    // }

    pub fn input(&mut self, _input: &str) -> Result<Option<f32>, String> {
        let tmp = _input.split_whitespace().collect::<Vec<&str>>();
        if !tmp
            .iter()
            .any(|x| x == &"+" || x == &"-" || x == &"*" || x == &"/")
        {
            return Err(_input.to_string());
        }
        let mut res = 0.0;
        let mut operator: Option<&str> = None;
        for letter in tmp {
            println!("{}", letter);
            if letter.parse::<f32>().is_ok() && operator != None {
                match operator {
                    Some("+") => res = res + letter.parse::<f32>().unwrap(),
                    Some("-") => res = res - letter.parse::<f32>().unwrap(),
                    Some("*") => res = res * letter.parse::<f32>().unwrap(),
                    Some("/") => res = res / letter.parse::<f32>().unwrap(),
                    _ => return Err(_input.to_string())
                };
            } else if letter.parse::<f32>().is_ok() {
                res = letter.parse::<f32>().unwrap();
            } else {
                operator = Some(letter);
            }
        }
        // let mut res: f32;
        // for x in tmp {

        // }
        // let response = tmp.iter().map(|letter: &str| {
        //     let mut res: f32;
            // let a = match letter {
            //     "+" => 1.0
            //     _ => res = letter.parse::<f32>().unwrap(),
            // };
        //     return res;
        // });
        // let operator = tmp.iter().filter(|&&x| {
        //     x == "+" ||  x == "-" ||  x == "*" ||  x == "/"
        // });

        // println!("{:?}", operator);
        // if _input == "A" {
        return Ok(Some(res));
        // }
    }
}

pub enum FooBar {
    Foo,
    Bar,
}
