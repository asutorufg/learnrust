pub fn closure() {
    println!("closure");
    let expensive_closure = |num| {
        println!("calculating slowly...");
        num
    };

    println!("{}", expensive_closure(1));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn closure2() {
    println!("\nclosure2");
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slow...");
        num
    });

    println!("enter:1, get:{}", expensive_closure.value(1));
    println!("enter:2, get:{}", expensive_closure.value(2));
}

pub fn closure3() {
    println!("\nclosure3");
    let x = 4;
    let equal_to_x = |num| num == x;

    // * 使用move关键字会将x的所有权强制移进闭包内；
    // let equal_to_x = move |num| num == x;

    println!("{}", equal_to_x(1));
    println!("{}", equal_to_x(4));
}
