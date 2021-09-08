pub fn test() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS {}", MAX_POINTS);
}

pub fn test2() {
    let tup = (1, 2, 3, "4[si]"); // tuple 多元组, 不同类型的元素可以在一起
    let (a, b, c, d) = tup;
    println!("a -> {},b -> {}, c-> {}, d -> {}", a, b, c, d);

    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // array 数组 必须是同一类型元素
    println!("arr[0] -> {}", arr[0]);
}

pub fn test3(a: i32, b: String) -> (i32, i64) {
    println!("test function: a->{},b->{}", a, b);
    (1, 2)
}

pub fn test4() {
    let a = 4;
    if a < 4 {
        println!("a<4");
    } else if a == 4 {
        println!("a==4");
    } else {
        println!("a>4");
    }

    let b = if a == 4 { 5 } else { 6 };
    println!("{}", b);

    let mut count = 0;
    let c = loop {
        count += 1;
        println!("loop");
        break count;
    };
    println!("loop result -> {}", c);

    let mut count = 0;
    while count < 4 {
        println!("count -> {}", count);
        count += 1;
    }

    let count = [1, 2, 3, 4, 5];
    for x in count.iter() {
        println!("x -> {}", x);
    }

    for x in (1..5).rev() {
        println!("xx -> {}", x);
    }
}

pub fn test5() {
    struct Dd {
        a: String,
        b: String,
        c: i64,
        d: f32,
    }

    let a = Dd {
        a: String::from("s: &str"),
        b: String::from("s: &strb"),
        c: 1,
        d: 2.,
    };

    println!("{} {} {} {}", a.a, a.b, a.c, a.d);
}

pub fn test6() {
    #[derive(Clone, Debug)]
    struct Dd {
        a: String,
        b: String,
        c: u64,
    }

    let a = Dd {
        a: String::from("s: &str"),
        b: String::from("s: &str"),
        c: 0,
    };
    let b = a.clone(); // need clone otherwise move a to b, then a can't use anymore

    println!("dd: {:?}", a);
    println!("bb: {:?}", b);
}

pub fn test7() {
    let a = String::from("s: &str");
    println!("{}", test8(&a));

    println!("{}", a);
}

fn test8(s: &String) -> usize {
    s.len()
}

pub fn test9() {
    struct Dd {
        a: String,
        b: String,
    }

    impl Dd {
        fn new() -> Dd {
            Dd {
                a: String::from("s: &str"),
                b: String::from("s: &str"),
            }
        }

        fn print(&self) {
            println!("a->{},b->{}", self.a, self.b);
        }
    }

    let a = Dd::new();

    a.print();
}
