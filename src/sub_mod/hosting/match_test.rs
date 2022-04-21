pub fn match_t_if_let() {
	let favorite_color: Option<&str> = None;
	let is_tuesday = false;
	let age: Result<u8, _> = "34".parse();

	if let Some(color) = favorite_color {
		println!("favorite_color: {}", color);
	} else if is_tuesday {
		println!("tuesday: {}", is_tuesday)
	} else if let Ok(age) = age {
		println!("age: {}", age)
	} else {
		println!("others")
	}
}

pub fn match_t_while_let() {
	let mut stack = Vec::new();

	stack.push(1);
	stack.push(2);
	stack.push(3);

	while let Some(top) = stack.pop() {
		println!("top: {}", top);
	}

	/*
	 * 这个例子会打印出 3、2 接着是 1。
	 * pop 方法取出 vector 的最后一个元素并返回 Some(value)。
	 * 如果 vector 是空的，它返回 None。while 循环只要 pop 返回 Some 就会一直运行其块中的代码。
	 * 一旦其返回 None，while 循环停止。我们可以使用 while let 来弹出栈中的每一个元素。
	 */
}

pub fn match_t_value() {
	let x = 1;

	match x {
		1 | 2 => println!("one or two"),
		3..=5 => println!("three through five"),
		_ => println!("something else"),
	}

	let y = 'c';

	match y {
		'a'..='j' => println!("early ASCII letter"),
		'k'..='z' => println!("late ASCII letter"),
		_ => println!("something else"),
	}
}

pub fn match_t_var() {
	let x = Some(5);
	let y = 10;

	match x {
		Some(50) => println!("got 50"),
		Some(y) => println!("y: {}", y),
		_ => println!("default case,x = {:?}", x),
	}

	println!("end: x = {:?}, y = {:?}", x, y)
}

struct Point {
	x: i32,
	y: i32,
}

pub fn match_t_struct() {
	let p = Point { x: 0, y: 7 };

	let Point { x: a, y: b } = p;
	assert_eq!(0, a);
	assert_eq!(7, b);

	let Point { x, y } = p;
	assert_eq!(x, a);
	assert_eq!(y, b);

	match p {
		Point { x, y: 0 } => println!("one the x axis at {}", x),
		Point { x: 0, y } => println!("one the y axis at  {}", y),
		Point { x, y } => println!("one neither axis: ({},{})", x, y),
	}
}

enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

pub fn match_t_enum() {
	let msg = Message::ChangeColor(0, 160, 255);

	match msg {
		Message::Quit => {
			println!("The Quit variant has no data to destructure.")
		}
		Message::Move { x, y } => {
			println!("Move in the x direction {} and in the y direction {}", x, y)
		}
		Message::Write(text) => println!("text message: {}", text),
		Message::ChangeColor(r, g, b) => {
			println!("Change the color to red {}, green {},and blue {}", r, g, b)
		}
	}

	let _ = Message::Move { x: 1, y: 2 };
	let _ = Message::Quit {};
	let _ = Message::Write(String::from(""));
}
