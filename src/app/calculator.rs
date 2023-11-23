use anyhow::{self, Error};

#[derive(Clone, Debug, PartialEq)]
enum Math {
    Multiplication,
    Divide,
    Addition,
    Subtraction,
    // ^
    Power,
    Fact,
    Cos,
    Tan,
    Sin,
    Log,
    Rad,
    Ln,
    CRoot,
    SRoot,
    Abs,

    //last answ
    Answ,

    //pi
    Pi,

    //Invalid
    KurvaAnyad,

    //curve
    X,
}

#[derive(Clone)]
pub struct Engine {
    buf: Vec<String>,
    num_list: Vec<f64>,
    expr_list: Vec<Math>,
    last_answ: Option<Vec<f64>>,
    step: f32,
    max: i16,
}
impl Default for Engine {
    fn default() -> Self {
        Self {
            buf: Vec::new(),
            num_list: Vec::new(),
            expr_list: Vec::new(),
            last_answ: None,
            step: 1.,
            max: 10,
        }
    }
}

#[allow(dead_code)]
impl Engine {
    fn invalid_equation(&self, err: Error, item_at_fault: String) {
        println!(
            "\n\n[{err}]\n{}\n{}",
            format!(
                "Item(s) at fault => {} [at index : {}]",
                item_at_fault,
                *self
                    .buf
                    .iter()
                    .position(|element| *element == item_at_fault)
                    .get_or_insert(0)
                    + 1 as usize
            ),
            self.buf
                .iter()
                .map(|f| format!("{} ", f))
                .collect::<String>()
        );
        let _ = std::io::stdin();
    }

    fn mathmain(mut self, buf: String) -> Option<Vec<f64>> {
        self.expr_list.clear();
        self.num_list.clear();

        self.buf = buf.split_whitespace().map(|f| f.to_string()).collect();
        if !self.buf.is_empty() {
            self.last_answ = self.mathdivider();
            return self.last_answ;
        } else {
            self.invalid_equation(Error::msg("Invalid equation (Empty equation)"), " ".into());
            None
        }
    }

    fn mathdivider(&mut self) -> Option<Vec<f64>> {
        for item in self.buf.clone() {
            match Engine::mathdecide(&item) {
                Ok(ok) => self.num_list.push(ok),
                Err(_expr) => self.expr_list.push(match item.to_lowercase().as_str() {
                    "+" => Math::Addition,
                    "-" => Math::Subtraction,
                    "*" => Math::Multiplication,
                    "/" | "%" | ":" => Math::Divide,
                    "^" | "pow" | "**" => Math::Power,
                    "cos" => Math::Cos,
                    "tan" => Math::Tan,
                    "sin" => Math::Sin,
                    "rad" => Math::Rad,
                    "log" => Math::Log,
                    "!" => Math::Fact,
                    "ln" => Math::Ln,
                    "croot" => Math::CRoot,
                    "sroot" => Math::SRoot,
                    "ans" | "answ" => Math::Answ,
                    "pi" => Math::Pi,
                    "x" => Math::X,
                    "abs" => Math::Abs,
                    _ => {
                        /*Go apeshit*/
                        self.invalid_equation(
                            Error::msg("Syntax Error (Invalid expression)"),
                            item.clone(),
                        );
                        Math::KurvaAnyad
                    }
                }),
            }
        }

        //finsihed sorting the 2 vectors
        if !(self.expr_list.is_empty() && self.num_list.is_empty()) && !self.expr_list.iter().any(|f| *f == Math::KurvaAnyad) {
            return self.mathengine();
        } else {
            None
        }
    }

    fn mathdecide(token: &str) -> anyhow::Result<f64> {
        let token = token.parse::<f64>()?;

        Ok(token)
    }
    // turn off safe rust compiler
    #[allow(unused_assignments)]
    fn mathengine(&mut self) -> Option<Vec<f64>> {
        let mut len: usize = self.expr_list.len();
        let mut index = 0;

        let mut results: Vec<f64> = Vec::new();
        let mut i: f32 = -self.max as f32;

        while i <= self.max.into() {
            i += self.step;

            index = 0;

            let mut expr_list_clone = self.expr_list.clone();
            let mut num_list_clone = self.num_list.clone();
            len = expr_list_clone.len();

            while index < len {
                if expr_list_clone[index] == Math::X {
                    expr_list_clone.remove(index);
                    if num_list_clone.is_empty() {
                        num_list_clone.push(i as f64)
                    } else {
                        num_list_clone.insert(index, i as f64);
                    }
                    //update lenght
                    len = expr_list_clone.len()
                } else if expr_list_clone[index] == Math::Pi {
                    expr_list_clone.remove(index);
                    num_list_clone.insert(index, self.pi());
                    len -= 1;
                }

                if len == 0 {
                    results.push(num_list_clone[0]);
                    continue;
                }

                match num_list_clone.get(index + 1) {
                    Some(_) => {}
                    None => {
                        break;
                    }
                }
                
                if len == 0 {
                    break;
                }

                index = 0;
                if expr_list_clone[index] == Math::Multiplication {
                    let result =
                        self.multiplication(num_list_clone[index], num_list_clone[index + 1]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Divide {
                    let result = self.divide(num_list_clone[index], num_list_clone[index + 1]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else {
                    index += 1;
                }
            }

            index = 0;
            while index < len {
                match num_list_clone.get(index + 1) {
                    Some(_) => {}
                    None => {
                        if expr_list_clone
                            .iter()
                            .any(|f| *f == Math::Power || *f == Math::Log) || num_list_clone.is_empty()
                        {
                            break;
                        }
                    }
                }
                if expr_list_clone[index] == Math::Power {
                    let result = self.power(num_list_clone[index], num_list_clone[index + 1]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Fact {
                    let result = self.fact(num_list_clone[index]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Cos {
                    let result = self.cos(num_list_clone[index]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Log {
                    let result = self.log(num_list_clone[index], num_list_clone[index + 1]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Tan {
                    let result = self.tan(num_list_clone[index]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Sin {
                    let result = self.sin(num_list_clone[index]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Rad {
                    let result = self.rad(num_list_clone[index]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::CRoot {
                    let result = self.croot(num_list_clone[index]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Ln {
                    let result = self.ln(num_list_clone[index]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::SRoot {
                    let result = self.sroot(num_list_clone[index]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else {
                    index += 1;
                }
            }
            index = 0;
            while index < len {
                match num_list_clone.get(index + 1) {
                    Some(_) => {}
                    None => {
                        break;
                    }
                }
                if expr_list_clone[index] == Math::Addition {
                    let result = self.addition(num_list_clone[index], num_list_clone[index + 1]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Subtraction {
                    let result = self.subtraction(num_list_clone[index], num_list_clone[index + 1]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else if expr_list_clone[index] == Math::Abs {
                    let result = self.abs(num_list_clone[index]);
                    expr_list_clone.remove(index);
                    num_list_clone.remove(index);
                    num_list_clone.insert(index, result);
                    len -= 1;
                } else {
                    index += 1;
                }
            }
            match num_list_clone.get(0) {
                Some(_) => {
                    results.push(num_list_clone[0]);
                }
                None => {
                    break;
                }
            }
            index = 0
        }

        if results.is_empty() {
            None
        }
        else {
            Some(results)
        }
    }
}
impl Engine {
    fn multiplication(&self, num1: f64, num2: f64) -> f64 {
        num1 * num2
    }
    fn divide(&self, num1: f64, num2: f64) -> f64 {
        num1 / num2
    }
    fn addition(&self, num1: f64, num2: f64) -> f64 {
        num1 + num2
    }
    fn subtraction(&self, num1: f64, num2: f64) -> f64 {
        num1 - num2
    }
    fn power(&self, num1: f64, num2: f64) -> f64 {
        num1.powf(num2)
    }
    fn fact(&self, num1: f64) -> f64 {
        if num1 == 0.0 || num1 == 1.0 {
            1.0
        } else {
            let mut result = 1.0;
            for i in 2..=num1 as u64 {
                result *= i as f64;
            }
            result
        }
    }
    fn cos(&self, num1: f64) -> f64 {
        num1.cos()
    }
    fn tan(&self, num1: f64) -> f64 {
        num1.cos()
    }
    fn sin(&self, num1: f64) -> f64 {
        num1.sin()
    }
    fn log(&self, num1: f64, num2: f64) -> f64 {
        num1.log(num2)
    }
    fn rad(&self, num1: f64) -> f64 {
        num1.to_radians()
    }
    fn ln(&self, num1: f64) -> f64 {
        num1.ln()
    }
    fn croot(&self, num1: f64) -> f64 {
        num1.cbrt().abs()
    }
    fn sroot(&self, num1: f64) -> f64 {
        num1.sqrt().abs()
    }
    fn pi(&self) -> f64 {
        std::f64::consts::PI
    }
    fn abs(&self, num1: f64) -> f64 {
        num1.abs()
    }
}

pub fn math_eng_init(input: String, step: f32, max: i16) -> Option<Vec<f64>> {
    Engine::mathmain(
        Engine {
            step: step,
            max: max,
            ..Default::default()
        },
        input,
    )
}
