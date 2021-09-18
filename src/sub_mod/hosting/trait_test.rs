fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

pub fn get_max_number() {
    let list = vec![1, 2, 3, 4, 5, 6];
    let largest = largest(&list);
    println!("list:{:?},largest: {}", list, largest)
}

#[derive(Debug)]
struct Point<T> {
    a: T,
    b: T,
}

#[derive(Debug)]
struct Point2<T, U, Z> {
    a: T,
    b: U,
    c: Z,
}

impl<T, U, Z> Point2<T, U, Z> {
    fn a(&self) -> &T {
        &self.a
    }

    fn b(&self) -> &U {
        &self.b
    }

    fn c(&self) -> &Z {
        &self.c
    }
}

pub fn struct_def() {
    let a = Point { a: 1, b: 1 };
    let b = Point { a: 1.0, b: 1.0 };

    println!("a: {:?}, b: {:?}", a, b);

    let c = Point2 {
        a: 1,
        b: "1",
        c: 1.0,
    };
    let d = Point2 {
        a: "1",
        b: 1.0,
        c: 1,
    };
    println!("c:{:?}, d: {:?}", c, d);
    println!("{},{},{}", c.a(), c.b(), c.c());
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize2(&self) -> String;
    fn summarize3(&self) -> String {
        format!("summarize3 default impl")
    }
}

pub struct ImplSummary {}

impl Summary for ImplSummary {
    fn summarize(&self) -> String {
        format!("summarize")
    }

    fn summarize2(&self) -> String {
        format!("summarize2")
    }
}

fn notify(item: impl Summary) {
    println!("{}", item.summarize());
    println!("{}", item.summarize2());
    println!("{}", item.summarize3());
}

fn notify2<T: Summary>(item: T) {
    println!("{}", item.summarize());
    println!("{}", item.summarize2());
    println!("{}", item.summarize3());
}

fn notify3<T>(item: T)
where
    T: Summary,
{
    println!("{}", item.summarize());
    println!("{}", item.summarize2());
    println!("{}", item.summarize3());
}

pub fn trait_test() {
    let a = ImplSummary {};
    notify(a);
    let b = ImplSummary {};
    notify2(b);
    let c = ImplSummary {};
    notify3(c);
}
