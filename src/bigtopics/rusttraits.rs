use std::{fmt::{Debug, Display}, iter::Sum};

struct NewsArticle {
    author: String,
    headline: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// adding the Summary trait to both NewsArticle and Tweet
impl Summary for NewsArticle {
    // this is commented to see the default implementation of summarize
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for Tweet {
    // here Tweet overrides the default implementation and has its own implementation of summarize
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// for every type that has this trait they should have this method
pub trait Summary {
    // adding the method w/o a body
    // fn summarize(&self) -> String;
    
    // adding a default implementation
    // fn summarize(&self) -> String {
    //     String::from("Read more...")
    // }

    // default implementations can call other methods inside our traits
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

// returning any type that implements the trait, it is commonly used in Closures, this has a restriction on which it can only return 1 type
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Twitter Dev"),
        content: String::from("Fixing Twitter tweets"),
        reply: false,
        retweet: true,
    }
}

// Conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x: {}", self.x);
        }
        else {
            println!("The largest member is y: {}", self.y);
        }
    }    
}

// Blanket implementations
// impl<T: Display> ToString for T {
//     // ...
// }

// traits as parameters
// fn notify(item: &impl Summary) { // this means anything or any type that implements the Summary trait
//     println!("Breaking News: {}", item.summarize());
// }

// trait bound
// this is the same as the code above, meaning its generic is restricted to the type that implements the Summary trait
fn notify<T: Summary>(item: &T) {
    println!("Breaking News: {}", item.summarize());
}

// this function can have different types
// we can also add multiple traits
fn notify_1(item1: &(impl Summary + Display), item2: &impl Summary) {
    // ...
}

// this function can have 1 type
fn notify_2<T: Summary + Display>(item1: &T, item2: &T) {
    // ...
}

// this code is hard to read to simplify this use the "where"
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     return 32
// }

// using the "where" clause
fn some_function<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone, U: Clone + Debug
{
    return 32
}

pub fn rust_traits(){
    println!("Rust Big Topic: Traits");

    // Sub-topic: Traits - difines functionality a particular type has and can share with other types, specify shared behaviour across multiply types

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not actually falling 🤓☝️")
    };

    let tweet = Tweet {
        username: String::from("John Doe"),
        content: String::from("Hello Twitter"),
        reply: false,
        retweet: false
    };

    // both article and tweet have access to the summarize() method since they both have the Summary trait
    println!("Article Summary | {}", article.summarize());
    println!("Tweet Summary | {}", tweet.summarize());

    notify(&article);

    println!("{}", return_summarizable().summarize());

    let new_pair = Pair::new(10, 20);
    new_pair.cmp_display();
}