

pub fn learning_iterators(){
    let list = vec![15, 25, 35];

    let list_iter = list.iter();

    for value in list_iter {
        println!("Value {}", value);
    }
}