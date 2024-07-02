

use std::rc::Rc;




enum List{
    Cons(i32, Rc<List>), 
    Nil,
}


use crate::List::{Cons, Nil};



fn main() {
    println!("Hello, world!");

    let heapData = Box::new(5);


    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));


    let c = Cons(4, Rc::clone(&a));

    println!("count after creating c = {}", Rc::strong_count(&a));


    println!("{} ", heapData);

    //Mutating the value inside an immutable value is the interior mutability pattern
} // heapData will goes out of scope here 


