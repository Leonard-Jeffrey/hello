use hello::*;

fn main() {
    let s: &str = "(jeffrey_shang,23,Man)";
    let p1: Person = Person::from_str(s).unwrap();
    //println!("{}", p1.to_string());

    println!("serialize/deseralize test");
    let seria = serde_json::to_string(&p1).unwrap();
    //println!("{}", seria);
    
    let deseria: Person = serde_json::from_str(&seria).unwrap();
    //println!("{}", deseria.to_string());

    p1.dress(String::from("T-shirt"));
    p1.eat(String::from("shit"));
    p1.live(String::from("mansion"));
    p1.mov(String::from("airplane"));
    p1.speak(String::from("Son of bitch"));
    
    // 自实现 Write trait
    //p1.write(String::from("Waiting for a long time!"));
    
    // 采用默认实现 Write trait
    p1.write();

    // 增加代码复用
    p1.show_info(String::from("I am king!!!"));

    //Person::show_info_for_p(&p1, String::from("I am hello kitty!!!"));

    let seria = serde_json::to_string(&p1).unwrap();
    println!("seria = {}", seria);

    let deseria: Person = serde_json::from_str(&seria).unwrap();
    println!("deseria = {:?}", deseria);

    let s: &str = "(蛋蛋,3,柯基)";
    let d1: Dog = Dog::from_str(s).unwrap();

    d1.eat(String::from("狗粮"));
    d1.live(String::from("狗窝"));
    d1.speak(String::from("汪汪"));

    allow_w1(&p1);
    allow_w2(&p1);
    allow_w3(&p1, &p1);
    allow_w4(&p1, &p1);

    allow_wel1(&p1);
    allow_wel2(&p1);
    allow_wel3(&p1, &d1);
    allow_wel4(&p1, &d1);
}
