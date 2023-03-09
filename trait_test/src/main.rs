pub trait Summary {
    // fn summarize(&self) -> String;
    fn test(&self) {}
    fn summarize_author(&self) -> String {
        format!("")
    }

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Tweet {
    fn my_tweet(&self) {
        println!(" i am myTweet !!!");
    }
    fn test(&self) {
        println!(" i am Tweet tetst !!!");
    }
}

impl Summary for Tweet {
    //无法取代  原来的
    fn test(&self) {
        println!(" i am Summary tetst !!!");
    }

    fn summarize_author(&self) -> String {
        self.test();
        self.my_tweet();
        format!("@{}", self.username)
    }
}

////////////////////

pub fn notify(item: &impl Summary) {
    println!("notify news! {}", item.summarize());
}

pub fn notify_bound<T: Summary>(item: T) {
    println!("notify_bound news! {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    notify(&article);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

    //
    println!("\r\n");
    notify(&article);
    notify(&tweet);

    tweet.test(); //i am Tweet tetst !!!  无法重构
}
