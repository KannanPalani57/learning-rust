




struct MyNameInSentence<'a> {
	name: &'a str, 
}


pub fn lifetime() {
	println!("Learning LifeTime");

	let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = largest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

	let myName = String::from("Kannan");

	let myNameStruct = MyNameInSentence {
		name: myName.as_str()
	};

	println!("My name in a struct as reference {}", myNameStruct.name);


}

fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}



// impl definition with lifetime
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }