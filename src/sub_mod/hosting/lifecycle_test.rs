fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("{}", y);
    x
}

pub fn longest_test() {
    let a = "abcd";
    let b = String::from("aaaaa");

    println!("longest: {}", longest(a, b.as_str()));

    let result;
    let c = "aad";
    {
        let d = String::from("dddd");
        result = longest(c, d.as_str());
    }
    println!("longest: {}", result);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

pub fn it() {
    let c = ImportantExcerpt { part: "a" };
    println!("{}-{}", c.part, c.level());
}
