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

pub trait Summary {
    fn summarize(&self) -> String;


    // デフォルト実装も使える
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
}

fn func() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "hoge"
        ),
    };

    println!("New article available! {}", article.summarize2());
}

// Summary を実装している何らかの型を引数に取る
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// トレイト境界も可
pub fn notify2<T: Summary>(item: &T) {
    println!("YOYOYOYO {}", item.summarize());
}

trait Display {
    fn display(&self) -> String;
}

// トイレト境界を複数指定することもできる

pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
    println!("Display: {}", item.display());
}

pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Display: {}", item.display());
}

// whereキーワードを使うことで、より読みやすい形にできる

fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
      U: Clone + Display
{
    0
}