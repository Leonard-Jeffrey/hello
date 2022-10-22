use crate::traits::*;
use crate::Person;
use crate::Dog;

impl Dress for Person {
    fn dress(&self, cloth: String) { 
        println!("{} dresses a/an/the {}!", self.Name, cloth);
    }
}

impl Eat for Person {
    fn eat(&self, food: String) {
        println!("{} eats a/an/the {}!", self.Name, food);
    }
}

impl Live for Person {
    fn live(&self, house: String) {
        println!("{} lives in {}!", self.Name, house);
    }
}

impl Mov for Person {
    fn mov(&self, vehicle: String) {
        println!("{} movs with a/an/the {}!", self.Name, vehicle);
    }
}

impl Speak for Person {
    fn speak(&self, word: String) {
        println!("{} speaks: {}!", self.Name, word);
    }
}

impl Write for Person {}

impl ShowInfo for Person {
    fn print_info(&self, priority: String) {
        println!("Name:{}\nAge:{}\nGender:{}\nPriority:{}",
        self.Name, self.Age, self.Gender, priority);
    }
}

// impl ShowInfoForP<Person> for Person {
//     fn print_info_for_p(p: &Person, priority: String) {
//         println!("Name: {}\nAge: {}\nGender: {}\nPriority: {}", 
//         p.Name, p.Age, p.Gender, priority);
//     }
// }

impl Eat for Dog {
    fn eat(&self, food: String) {
        println!("Dog {} is eating {}!", self.Name, food);
    }
}

impl Live for Dog {
    fn live(&self, house: String) {
        println!("Dog {} lives in a/an/the {}", self.Name, house);
    }
}

impl Speak for Dog {
    fn speak(&self, word: String) {
        println!("Dog {} speaks {}", self.Name, word);
    }
}
