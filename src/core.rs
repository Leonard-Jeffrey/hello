
//extern crate serde_derive;
//extern crate serde;
//extern crate serde_json;

pub use std::convert::TryFrom;
pub use std::convert::TryInto;

// Display test
pub use std::fmt;

// ToString test
pub use std::string::ToString;

// FromStr test
pub use std::str::FromStr;
use std::num::ParseIntError;

// traits test
use crate::traits::*;

use crate::Person;
use crate::Dog;

impl TryFrom<(String, u8, String)> for Person {
    // 自定义 Result 泛型中的类型
    type Error = ();
    fn try_from((name, age, gender):(String, u8, String)) -> Result<Self, Self::Error> {
        if age > 18 && age < 120 {
            Ok(Person {
                Name: name,
                Age: age,
                Gender: gender,
            })
        }
        else {
            Err(())
        }
    }
}

// 实现 new 功能
#[allow(dead_code)]
impl Person {
    fn new(name: String, age: u8, gender: String) -> Self {
        Self { Name: name, Age: age, Gender: gender }
    }
}

// impl fmt::Display for Person {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Name: {}\nAge: {}\nGender: {}", self.Name, self.Age, self.Gender)
//     }
// }

impl ToString for Person {
    fn to_string(&self) -> String {
        format!("Person Information\n{}\n{}\n{}", self.Name, self.Age, self.Gender)
    }
}

impl FromStr for Person {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let info: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
                                .split(',')
                                .collect();

        let age:u8 = info[1].parse::<u8>()?;

        Ok(Person{Name: String::from(info[0]), Age: age, Gender: String::from(info[2])})
    }
}

// Function: parse()
// pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err>
// where
//     F: FromStr, 

impl FromStr for Dog {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let info: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
                                .split(',')
                                .collect();

        let age:u8 = info[1].parse::<u8>()?;

        Ok(Dog{Name: String::from(info[0]), Age: age, Type: String::from(info[2])})
    }
}