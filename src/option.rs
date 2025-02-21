


struct User {
    first_name: String, 
    last_name: Option<String>
}


impl User {
    fn get_user_full_name(&self) -> String {
        match &self.last_name {
            None => self.first_name.clone(), 
            Some(name) => format!("{} {}", self.first_name.clone(), name)  
        }
    }
}


pub fn learning_option() {
    let john = User {
        first_name: "John".to_string(), 
        last_name: Some("Doe".to_string())
    };

    println!("John full name is {} ",john.get_user_full_name());
    
    let mike = User {
        first_name: "Mike".to_string(), 
        last_name: None, 
    };
    
    println!("Mike full name is {}", mike.get_user_full_name());
}