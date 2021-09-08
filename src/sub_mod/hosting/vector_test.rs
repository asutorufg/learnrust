pub fn vct() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(4);

    println!("vector v: {:?}", v);

    let b = vec![1, 2, 3, 4];
    println!("vector b: {:?}", b);

    let third: &i32 = &b[2];
    println!("the third element is {}", third);

    match b.get(16) {
        Some(third) => println!("the third element is {}", third),
        None => println!("there is no third element"),
    }

    for i in &b {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50
    }
    println!("vector v: {:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Float(6.4),
        SpreadsheetCell::Text(String::from("string")),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(_) => println!("Int: {:?}", i),
            SpreadsheetCell::Text(_) => println!("Text: {:?}", i),
            SpreadsheetCell::Float(_) => println!("Float: {:?}", i),
        }
    }
    println!("vector row: {:?}", row);
}
