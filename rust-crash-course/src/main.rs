#![deny(clippy::all)]

// const MY_AGE: u8 = 22;

fn greet(name: &String) {
    println!("Hello, {}!", name);
}

fn main() {
    // let mut name = "Foo";
    // name = "Doe";
    // let population = 62_000_000;
    // let distance1 = 5.5;
    // let distance2 = 62.2;
    // let distance3 = 30.2;
    // let total_distance = distance1 + distance2 + distance3;
    // println!("Hello,{}!", total_distance);
    // println!("We country have {} peoples", population);
    // let data = "Foo";
    // {
    //     let data = data.to_string();
    // }
    // println!("Hello, world!");
    // println!("My age is {}", MY_AGE);
    // let personal_data = (22u8, "levent");
    // let age = personal_data.0;

    let name1 = String::from("John");
    let name2 = &name1;
    println!("Hello, {}", name1);
    println!("Hello, {}", &name1);
    greet(name2);
    {
        let age1 = 10;
        let age2 = age1;
        println!("You are {} years old", age1);
        println!("You are {} years old", age2);
    }
}
