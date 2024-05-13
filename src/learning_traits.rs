use std::fmt::format;



pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    fn summarize_content(&self) -> String {
        format!("Read More  {}" , self.summarize_author())   
    }
}

pub struct NewsArticle {
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("The author name is {}", self.author)
    }
}

pub struct Tweet {
    pub username: String, 
    pub content: String , 
    pub reply: bool, 
    pub retweet: bool 
}


impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)    
    }

    fn summarize_author(&self) -> String {
        format!("The author name is {}", self.username)
    }
}


pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize_content());
}


pub fn learning_traits() {
    println!("Learing Traits");

    let tweet = Tweet {
        username: String::from("Kannan"),
        content: String::from("Hey everyone, I am learning Rust!"),
        reply: false, 
        retweet: false,  
    };



    println!(" {}", tweet.summarize_content());

    let news_article = NewsArticle {
        headline: String::from("today CSK have interesting IPL Match"),
        location: String::from("India"),
        author: String::from("John"),    
        content: String::from("Every csk fans are excited to watch today IPL Match")
    };


    println!(" {}", news_article.summarize_content());

}


// Both are same
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// defining more than one trait bound
// pub fn notify(item: &(impl Summary + Display)) {}

// pub fn notify<T: Summary + Display>(item: &T) {}


// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { }

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
    // T: Display + Clone,
    // U: Clone + Debug,
// { }


// Returning types that implements traits

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     }
// }    