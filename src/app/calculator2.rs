#![allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct Coordinates {
    x: f64,
    y: f64,
}
impl Coordinates {
    pub fn wrap_coordinates(x: f64, y: f64) -> Coordinates {
        Coordinates { x: x, y: y }
    }
}
impl Index<usize> for Coordinates {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            return &self.x;
        }
        else if index == 1 {
            return &self.y;
        }
        else {
            panic!("Painced at Coordinate indexing, index out ofr range (x ; y)");
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Math {
    Multiplication,
    Divide,
    Addition,
    Subtraction,
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

    //curve
    X,
}
pub struct Calculator {
    pub buf: String,
    pub num_buf: Vec<f64>,
    pub expr_buf: Vec<Math>,
    pub bounds: f32,
    pub step: f32,

    //  ::::  Option of vector of coordinates  :::: -> Some(Vec < [0;0] >)
    pub output: Option<Vec<Coordinates>>
}
impl Default for Calculator {
    fn default() -> Self {
        Self {
            buf: String::new(),
            num_buf: Vec::new(),
            expr_buf: Vec::new(),
            bounds: 0.0,
            step: 0.0,

            output: None,
        }
    }
}

use std::{f64::consts::PI, ops::Index};

use anyhow::{self, Error};

impl Calculator {
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
impl Calculator {

    fn mathdecide(&self, token: &str) -> anyhow::Result<f64> {
        let token = token.parse::<f64>()?;

        Ok(token)
    }

    pub fn sort(&mut self) -> Option<Vec<Coordinates>> {
        let vec_buf: Vec<String> = self.buf.split_whitespace().map(|f| f.to_string()).collect();
        for item in vec_buf {
            match self.mathdecide(&item) {
                Ok(num) => {
                    self.num_buf.push(num);
                }
                Err(_) => {
                    self.expr_buf.push(
                        match item.to_lowercase().as_str() {
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
                                return None;
                            }
                        }
                    )
                }
            }
        };
        //finished sorting
        self.calculate()
    }

    pub fn calculate(&self) -> Option<Vec<Coordinates>> {
        //replace X and Pi
        let mut x = -self.bounds;

        let mut partial_result: Vec<f64> = Vec::new();
        let mut partial_coords: Vec<f64> = Vec::new();
        while x < self.bounds {
            
            //make local clone so it can be used for X

            //This vector is the clone of the sorted user input
            let mut num_list_clone = self.num_buf.clone();
            let mut expr_list_clone = self.expr_buf.clone();

            //inceremnt by step
            x += self.step;

            for (index, item) in expr_list_clone.clone().iter().enumerate() {
                match item {
                    Math::X => { expr_list_clone.remove(index); num_list_clone.insert(index, x as f64) }
                    Math::Pi => { expr_list_clone.remove(index); num_list_clone.insert(index, PI) }
                    _ => continue
                }    
            };
            //x ^ 2

            for (index, item) in expr_list_clone.clone().iter().enumerate() {
                match num_list_clone.get(index + 1){
                    Some(_) => {},
                    None => break
                }
                match item {
                    Math::Multiplication => {
                        expr_list_clone.remove(index);
                        let result = self.multiplication(num_list_clone[index], num_list_clone[index + 1]);
                        num_list_clone.remove(index);
                        num_list_clone.remove(index);
                        num_list_clone.insert(index, result);
                        dbg!(result);
                    }
                    Math::Divide => {
                        expr_list_clone.remove(index);
                        let result = self.divide(num_list_clone[index], num_list_clone[index + 1]);
                        num_list_clone.remove(index);
                        num_list_clone.remove(index);
                        num_list_clone.insert(index, result);
                        dbg!(result);
                    }
                    _ => continue
                }
            }

            //back up last saved data
            match num_list_clone.get(0) {
                Some(num) => {
                    partial_result.push(*num);
                    partial_coords.push(x as f64);
                },
                None => {},
            }
        }

        let mut wrapped_result: Vec<Coordinates> = Vec::new();


        //expect self.max == partial_result.len()
        //-10 .. 10
        for (index ,num) in partial_result.iter().enumerate() {
            wrapped_result.push(Coordinates::wrap_coordinates(partial_coords[index], *num))
        };

        Some(wrapped_result)
    }

    pub fn init(&mut self) -> Option<Vec<Coordinates>> {
        let eredmeny = self.sort();        
        return eredmeny;
    }
}