use crate::traits::*;
use crate::behaviour::*;

//

pub fn allow_w1(P: &impl Write){
    println!("show the write skill~");
    P.write();
}

// trait bound 语法
pub fn allow_w2<T: Write>(P: &T){
    println!("show the write skill~");
    P.write();
}

// P1 P2 不一定是同一种类型
pub fn allow_w3(P1: &impl Write, P2: &impl Write){
    println!("show the write skill~");
    P1.write();
    P2.write();
}

// 要求 P1 P2 是同一种类型
pub fn allow_w4<T: Write>(P1: &T, P2: &T){
    println!("show p1/p2 the write skill");
    P1.write();
    P2.write();
}

//
pub fn allow_wel1(P: &(impl Write + Eat + Live + Speak)){
    println!("show all the skills:");
    P.write();
    P.eat(String::from("wel1"));
    P.live(String::from("wel1"));
    P.speak(String::from("wel1"));
}

pub fn allow_wel2<T: Write + Eat + Live + Speak>(P: &T){
    println!("show all the skills:");
    P.write();
    P.eat(String::from("wel2"));
    P.live(String::from("wel2"));
    P.speak(String::from("wel2"));
}

pub fn allow_wel3<T: Write + Eat + Live + Speak, U: Eat + Live + Speak>(P1: &T, P2: &U){
    println!("show the skills of P1 and P2:");
    P1.write();
    P1.eat(String::from("wel3"));
    P1.live(String::from("wel3"));
    P1.speak(String::from("wel3"));
    P2.eat(String::from("wel3"));
    P2.live(String::from("wel3"));
    P2.speak(String::from("wel3"));
}

// 通过 where 简化 trait bound
pub fn allow_wel4<T, U>(P1: &T, P2: &U)
    where T: Write + Eat + Live + Speak,
          U: Eat + Live + Speak
{        
          println!("show the skills of P1 and P2:");
          P1.write();
          P1.eat(String::from("wel4"));
          P1.live(String::from("wel4"));
          P1.speak(String::from("wel4"));
          P2.eat(String::from("wel4"));
          P2.live(String::from("wel4"));
          P2.speak(String::from("wel4"));
}