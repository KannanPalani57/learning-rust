
use std::thread;


#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red, 
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory{


    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }


    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = self.red_shirt_count();
        let mut num_blue = self.blue_shirt_count();

        if num_red > num_blue {
            ShirtColor::Red
        }else {
            ShirtColor::Blue
        }

    }

    fn blue_shirt_count(&self) -> i32{
        let mut blue_shirts = 0;
        for shirt in &self.shirts {
            if shirt == &ShirtColor::Blue {
                blue_shirts = blue_shirts + 1;
            }
        }

        blue_shirts
    }

    fn red_shirt_count(&self) -> i32{
        let mut red_shirts = 0;
        for shirt in &self.shirts {
            if shirt == &ShirtColor::Red {
                red_shirts = red_shirts + 1;
            }
        }

        red_shirts
    }
}


pub fn closure() {
    println!("Learning Closure");

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Blue,  ShirtColor::Red]
    };

    println!("Blue Shirts Count {}", store.blue_shirt_count());
    println!("Red Shirts Count {}", store.red_shirt_count());
    println!("Most stocked {:?}", store.most_stocked());

    let user1_pref = Some(ShirtColor::Blue);

    let give_away1 = store.give_away(user1_pref);

    println!(
        "The user with preference {:?} gets {:?}",
        user1_pref, give_away1
    );

    let user2_pref : Option<ShirtColor> = None;

    let give_away2 = store.give_away(user1_pref);

    println!(
        "The user with preference {:?} gets {:?}",
        user2_pref, give_away2
    );


    // capturing references or moving ownership

    let mut list = vec![1,2,3];

    let only_borrows = || println!("From Clousure {:?}", list);


    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // println!("Before calling borrows_mutably: {:?}", list);

    // let mut borrows_mutably = || list.push(12);

    // println!("After calling borrows_mutably: {:?}", list);

    thread::spawn(move || println!("From thread {:?}", list)).join().unwrap();


}

// learn more about FnOnce, FnMut, Fn Traits.
fn move_env_fnonce() {

    let greeting = String::from("Welcome");


    // moves ownership of the variable by using FnOnce trait
    let greet_user = move |name| {
        println!("{} {}", greeting, name);
    };
    
    greet_user("Kannan");
    
}

fn takes_mutable_env() {
    let mut count = 0;

    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };

    increment();
    increment();
}

fn takes_reference_env(){
    let greeting = String::from("Hello");

    let greet_user = |name| {
        println!("{} {}", greeting, name);
    };

    greet_user("Alice");
}