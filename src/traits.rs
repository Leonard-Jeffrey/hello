pub trait Dress {
    // A person dresses
    fn dress(&self, cloth: String);
}

pub trait Eat {
    // A person eats
    fn eat(&self, food: String);
}

pub trait Live {
    // A person lives
    fn live(&self, house: String);
}

pub trait Mov {
    // A person moves
    fn mov(&self, device: String);
}

pub trait Speak {
    // A person speaks
    fn speak(&self, word: String);
}

// 提供默认实现
pub trait Write {
    // A person read
    fn write(&self) {
        println!("A person writes something ....");
    }
}

pub trait ShowInfo {
    fn show_info(&self, priority: String) {
        println!("Ready to Print the Personal Information .......");
        self.print_info(priority);
    }

    fn print_info(&self, priority: String);
}

// pub trait ShowInfoV2<P> {
//     fn show_info() {
//         println!("Ready to Print the Personal Information .......");

//     }
// }

// pub trait ShowInfoForP<P> {
//     fn print_info_for_p(p: &P, priority: String);

//     fn show_info_for_p(p: &P, priority: String) {
//         println!("Ready to Print the Personal Information .......");
//         print_info_for_p(p, priority);
//     }
// }