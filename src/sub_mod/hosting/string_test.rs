pub fn string() {
    let mut s = String::new();
    println!("string s: {}", s);
    let data = "some string data";
    s = data.to_string();
    println!("string s: {}", s);
    s = "some string direct".to_string();
    println!("string s: {}", s);
    s = String::from("some string from String from");
    println!("string s: {}", s);

    s.push_str("pushed string");
    println!("string s: {}", s);

    s.push(' ');
    s.push('s');
    println!("string s: {}", s);

    let s1 = String::from("hello, ");
    let s2 = String::from("world!!");
    let s3 = s1 + &s2;
    println!("s1 + s2 = s3: {}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("s7: {}", s7);

    for c in "नमस्ते😅🐮🍺你好".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते😅🐮🍺你好".bytes() {
        println!("{}", b);
    }
}
