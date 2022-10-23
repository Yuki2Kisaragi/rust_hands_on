#[allow(dead_code)]
fn sample_var0() {
    let mut x = 1;
    println!("x = {}", x); // x = 1
    x = 10;
    println!("x = {}", x); // x = 11
    x += 9; // x = x + 9 と同義
    println!("x = {}", x); // x = 20
}

#[allow(dead_code)]
fn sample_var1() {
    let n1 = 10_000; // n1 = 10000, Defaultだとi32で定義される
    let n2 = 0u8; // u8
    let n3 = -100_isize; // isize型
    let n4: i128 = 10; // i128型
    let n5 = 250;
    println!("n1 = {}", n1);
    println!("n2 = {}", n2);
    println!("n3 = {}", n3);
    println!("n4 = {}", n4);
    println!("n5 = {}", n5);
}

#[allow(dead_code)]
fn sample_var2() {
    let s1 = "Hello world!".to_string();
    let s2 = String::from("Good evening!");
    let str_1 = "ハローワールド"; // str型で定義
    let s3 = str_1.to_string(); // str型からString型を作る

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("str_1: {}", str_1);
    println!("s3: {}", s3);
}

#[allow(dead_code)]
fn sample_var3() {
    let name = "MasaHero"; // str型
    let mut greeting = String::from("My name is "); //String型
    greeting.push_str(name);
    println!("{}", greeting);
}

// function

#[allow(dead_code)]
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn sample_func1() {
    let a: i32 = 100;
    let b: i32 = 500;

    let c = add_i32(a, b);
    println!("a + b = {}", c);
}

// if

#[allow(dead_code)]
fn sample_if_2() {
    let a = 10;

    let result = if a > 10 {
        "greater than 10"
    } else if a == 10 {
        "equal"
    } else {
        "rather than 10"
    };

    println!("result : {}", result);
}

// loop

#[allow(dead_code)]
fn sample_loop1() {
    let mut i = 0;
    loop {
        println!("hoge: {}", i);
        i += 1;
        if i > 10 {
            break;
        }
    }
}

#[allow(dead_code)]
fn sample_loop2() {
    let mut i = 0;
    loop {
        println!("hoge: {}", i);
        i += 1;
        if i > 10 {
            break;
        } else {
            continue;
        }
    }
}

#[allow(dead_code)]
fn sample_loop3() {
    let mut counter = 0;

    let ten = loop {
        if counter == 10 {
            break counter;
        }
        counter += 1;
    };

    println!("counter = {}", ten);
}

#[allow(dead_code)]
fn sample_for_1() {
    let array = [1, 2, 3, 4, 5];

    for e in array.iter() {
        println!("{e}");
    }

    let str_vec = vec!["a", "i", "u", "e", "o"];
    for e in str_vec {
        println!("{e}");
    }
}

#[allow(dead_code)]
fn sample_match_1() {
    let value = 100;

    match value {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => println!("something"),
    }
}

#[allow(dead_code)]
fn sample_match_2() {
    let value = 100;
    let result = match value {
        1 => "one",
        10 => "ten",
        100 => "one hundred",
        _ => "something",
    };
    println!("result: {}", result);
}

use rand::Rng;

#[allow(dead_code)]
fn sample_match_3() {
    // 0~10までの乱数を作る
    let num: i32 = rand::thread_rng().gen_range(0..11);

    let str_number = match num {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 | 4 => "three or four",
        5..=8 => "something",
        9 => "nine",
        _ => "greater than 9",
    };
    println!("str_number = {str_number}");
}

// Enum + match

#[allow(dead_code)]
#[derive(Debug)]
enum Country {
    Japan,
    USA,
    UK,
}

#[allow(dead_code)]
fn sample_enum_2() {
    let country1 = Country::USA;
    println!("{:?}", country1);

    let country = Country::Japan;
    let cont = match country {
        Country::Japan => {
            println!("日本です");
            "日本"
        }
        Country::USA => {
            println!("米国です");
            "米国"
        }
        Country::UK => {
            println!("英国です");
            "英国"
        }
    };
    println!("国 : {}", cont);
}

#[allow(dead_code)]
fn lesson1_scope_and_drop() {
    {
        let a = 10;
        println!("a = {}", a); // a = 1
                               // ここでaは破棄される
    }
    // aは破棄されているのでコンパイルエラーとなる
    // 下の行をコメントアウトして実行してみてください
    // println!("a = {}", a);
}

#[allow(dead_code)]
fn lesson2_move_semantics() {
    // ムーブセマンティクス
    let x = String::from("hello");
    println!("x = {}", x);
    let y = x;
    println!("y:{}", y);

    // xからyへ所有権(値)が移動したためコンパイルエラーとなる
    // println!("x:{}", x);
}

// ownerships and function

#[allow(dead_code)]
fn move_ownership(str1: String) {
    // String型なのでムーブセマンティクス。
    println!("{}", str1);
}

#[allow(dead_code)]
fn copy_data(some_integer: i32) {
    // i32型なのでコピーセマンティクス。
    println!("{}", some_integer);
}

#[allow(dead_code)]
fn lesson3_ownership_and_func() {
    let s = String::from("hello");

    move_ownership(s); // sが関数に入り、所有権の移動が発生する。sは破棄される。

    let x = 5;

    copy_data(x); // xも関数に入るが、コピーなので`x`は再度使用可能！
}

// reference

fn print_value(str1: &String) {
    println!("{}", str1);
}

#[allow(dead_code)]
fn lesson4_reference() {
    let s = String::from("hello");

    print_value(&s); // 所有権は移動されない。

    println!("{}", s); // sはそのまま使える
}

#[allow(dead_code)]
fn lesson5_mutable_reference() {
    let mut s1 = String::from("hello");
    println!("s1 = {s1}");
    let s2 = &mut s1;
    s2.push_str(" world"); // コンパイルできる
    println!("s2 = {s2}");
}

fn main() {
    println!("Welcome Rust World!!");
    println!(
        "
        You can call the following functions.
            * sample_var0();
            * sample_var1();
            * sample_var2();
            * sample_var3();
            * sample_loop1();
            * sample_loop2();
            * sample_loop3();
            * sample_if_2();
            * sample_for_1();
            * sample_match_1();
            * sample_match_2();
            * sample_match_3();
            * sample_enum_2();
            * lesson1_scope_and_drop();
            * lesson2_move_semantics();
            * lesson3_ownership_and_func();
            * lesson4_reference();
            * lesson5_mutable_reference();
        "
    );

    // sample_var0();
    // sample_var1();
    // sample_var2();
    // sample_var3();
    // sample_loop1();
    // sample_loop2();
    // sample_loop3();
    // sample_if_2();
    // sample_for_1();
    // sample_match_1();
    // sample_match_2();
    // sample_match_3();
    // sample_enum_2();
    // lesson1_scope_and_drop();
    // lesson2_move_semantics();
    // lesson3_ownership_and_func();
    // lesson4_reference();
    // lesson5_mutable_reference();
}
