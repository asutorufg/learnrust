#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 1,
            style: String::from("a"),
        },
        Shoe {
            size: 2,
            style: String::from("aa"),
        },
        Shoe {
            size: 1,
            style: String::from("aaaaa"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 1);

    println!("{:?}", in_my_size);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn counter() {
    let c: Vec<_> = Counter::new().collect();
    println!("{:?}", c);

    let c1: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .collect();
    println!("{:?}", c1);

    let c2: Vec<_> = Counter::new().zip(Counter::new().skip(1)).collect();
    println!("{:?}", c2);

    let c3: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .collect();
    println!("{:?}", c3);
}
