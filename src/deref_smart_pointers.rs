use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn name(name: &str) {
    println!("Hello {}", name);
}

pub fn deref_smart_pointers(){
    println!("Dereference Smart Pointers");

    let y = MyBox::new(5);

    println!("{:?}", &y);   

    let m = MyBox::new("Kannan");  // Converts &String to &str - Deref Corercion

    // without deref coercion
    // hello(&(*m)[..])

    name(&m)


    
}