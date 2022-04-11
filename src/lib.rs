pub struct Dog;

impl Dog {
    pub fn bark(&self) {
        println!("baf");
    }
}

pub struct Interpreter;

impl Interpreter {

    pub fn input(&mut self, _input: &str) -> Result<Option<f32>, String> {
        let tmp = _input.split_whitespace().collect::<Vec<&str>>();
        if !tmp
            .iter()
            .any(|x| x == &"+" || x == &"-" || x == &"*" || x == &"/" || x == &"%")
        {
            return Err(_input.to_string());
        }
        let mut res = 0.0;

        operator("+", 0.0, 0.0);

        // let mut operator: Option<&str> = None;
        // for letter in tmp {
        //     if letter.parse::<f32>().is_ok() && operator != None {
        //         match operator {
        //             Some("+") => res = res + letter.parse::<f32>().unwrap(),
        //             Some("-") => res = res - letter.parse::<f32>().unwrap(),
        //             Some("*") => res = res * letter.parse::<f32>().unwrap(),
        //             Some("/") => res = res / letter.parse::<f32>().unwrap(),
        //             Some("%") => res = res % letter.parse::<f32>().unwrap(),
        //             _ => return Err(_input.to_string())
        //         };
        //     } else if letter.parse::<f32>().is_ok() {
        //         res = letter.parse::<f32>().unwrap();
        //     } else {
        //         operator = Some(letter);
        //     }
        // }
        return Ok(Some(res));
    }
}

fn operator(operator: &str, left: f32, right: f32) -> &str {
    return match operator {
        "+" => "",
        "-" => "",
        "*" => "",
        "/" => "",
        "%" => "",
        _ => operator,
    };
}

pub enum FooBar {
    Foo,
    Bar,
}
