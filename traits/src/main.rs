fn main() {
    println!("Hello, world!");
}

// example 1

pub trait Summary {
    fn summarize(&self) -> String;
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
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// example 2
pub trait Summary2 {
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
}

//example 3
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// example 4
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// example 5
pub fn notify(item: &(impl Summary + Diplay)) {
    println!("Breaking news! {}", item.summarize());
}
// example 6
pub fn notify<T: Summary + Diplay>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// example 7
fn some_function<T,U>(t: T, u: U) -> String
where
    T: Display + Clone
    U: Clone + Debug
{
    String::from("someone else!!!!!")
}
// example 8
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
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
