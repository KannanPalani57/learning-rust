use std::fmt::format;



pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_content(&self) -> String {
        String::from("Read More")
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
}

pub fn learning_traits() {
    println!("Learing Traits");

    let tweet = Tweet {
        username: String::from("Kannan"),
        content: String::from("Hey everyone, I am learning Rust!"),
        reply: false, 
        retweet: false,  
    };



    println!(" {}", tweet.summarize());

    let news_article = NewsArticle {
        headline: String::from("today CSK have interesting IPL Match"),
        location: String::from("India"),
        author: String::from("John"),    
        content: String::from("Every csk fans are excited to watch today IPL Match")
    };


    println!(" {}", news_article.summarize_content());

}