// 由于枚举List2的成员包含它本身
// rust在编译时无法确定其大小，编译会报错
// 需要使用智能指针（存的是指针，但在使用时与直接使用无区别），如Box<T>
// enum List2 {
// Con(u32, List2),
// Nil,
// }

#[derive(Debug)]
enum List {
    Con(u32, Box<List>),
    Nil,
}

use std::ops::Deref;
use List::Con;
use List::Nil;

pub fn smart_pointers() {
    let a = Con(1, Box::new(Con(2, Box::new(Con(3, Box::new(Nil))))));

    println!("{:?}", a);
}

// deref trait 实现解引用指针

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现解引用方法 需要引入std::ops::Deref
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn deref_trait() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn force_deref_trait() {
    let m = MyBox::new(String::from("Rust"));

    // Deref强制转换
    /*
    这里使用 &m 调用 hello 函数，其为 MyBox<String> 值的引用。因为 MyBox<T> 实现了 Deref trait，
    Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。
    标准库中提供了 String 上的 Deref 实现，
    其会返回字符串 slice，这可以在 Deref 的 API 文档中看到。
    Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了。
     */
    hello(&m);
}

// Drop Trait
// 对于智能指针模式来说第二个重要的 trait 是 Drop，其允许我们在值要离开作用域时执行一些代码。

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoint with data `{}`!", self.data);
    }
}

pub fn drop_trait() {
    let _c = CustomSmartPointer {
        data: String::from("s: c"),
    };
    let _d = CustomSmartPointer {
        data: String::from("s: d"),
    };

    println!("CustomSmartPointers created.");
}

// Rc 引用计数智能指针
// 大部分情况下所有权是非常明确的：可以准确地知道哪个变量拥有某个值。然而，有些情况单个值可能会有多个所有者。
// 例如，在图数据结构中，多个边可能指向相同的节点，而这个节点从概念上讲为所有指向它的边所拥有。节点直到没有任何边指向它之前都不应该被清理。

enum List2 {
    Cons(i32, Rc<List2>),
    Nil2,
}

use std::rc::Rc;
use List2::{Cons, Nil2};
pub fn rc_c() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Rc::new(Cons(11, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Rc::new(Cons(12, Rc::clone(&a)));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    /*
    Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
    Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。
    通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。
    当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
     */
}
