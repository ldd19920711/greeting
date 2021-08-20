fn main() {
    // let a = "world";
    // println!("hello,{}", a);
    // let b = 2;
    // println!("a is {}", b);
    // println!("a is {0}, a again is {0}", b);
    // println!("{{hello,world}}");
    // let mut c = 3;
    // println!("{}", c);
    // c = 4;
    // println!("{}", c);
    // let c = 5;
    // let c = 6;
    // println!("{}", c);
    // const d: i32 = 7;
    // println!("{}", d);
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x);
    // let sum = 5 + 10; // 加
    // println!("{}", sum);
    // let difference = 95.5 - 4.3; // 减
    // println!("{}", difference);
    // let product = 4 * 30; // 乘
    // println!("{}", product);
    // let quotient = 56.7 / 32.2; // 除
    // println!("{}", quotient);
    // let remainder = 43 % 5; // 求余
    // println!("{}", remainder);
    // let flag = true;
    // let flag = false;
    // let flag: bool = true;
    // let byte1 = 'A';
    // println!("{}", byte1);
    // let tup: (i32, f64, u8) = (500, 40.0, b'A');
    // println!("{},{},{}", tup.0, tup.1, tup.2);
    // let arr = [1, 2, 3, 4];
    // println!(arr)
    // let i = add(1, 2);
    // println!("{}", i)
    // let x = 1;
    // let y = 2;
    // println!("x={},y={}", x, y);
    // let tmp = x;
    // let x = y;
    // let y = tmp;
    // let swap_result = swap(x, y);
    // println!("x={},y={}", swap_result.0, swap_result.1);
    //
    // let z = {
    //     let a = 7;
    //     a + 2;
    //     let c = 2;
    //     c * a
    // };
    // println!("{}", z);
    // fn five() -> i32 {
    //     5
    // }
    // println!("five() 的值为: {}", five())
    // fn sum(a: i32, b: i32) -> i32 {
    //     return a + b
    // }
    // println!("sum(a,b)的值为: {}", sum(1, 2));
    // let number = 5;
    // if number < 5 {
    //     println!("number < 5")
    // } else {
    //     println!("number >= 5")
    // }
    // let mut number = 1;
    // while number != 4 {
    //     println!("{}", number);
    //     number += 1;
    // }
    // println!("EXIT();");
    // let list1 = [1, 2, 3];
    // for item in 0..list1.len() {
    //     println!("位置为{}的元素值为: {}", item, list1[item]);
    // }
    // let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    // let mut i = 0;
    // loop {
    //     let ch = s[i];
    //     if ch == 'O' {
    //         break;
    //     }
    //     println!("\'{}\'", ch);
    //     i += 1;
    // }
    // {
    //     // 在声明以前，变量 s 无效
    //     let s = "runoob";
    //     // 这里是变量 s 的可用范围
    // }
    // {
    //     // 在声明以前，变量 s 无效
    //     let s = "runoob";
    //     // 这里是变量 s 的可用范围
    // }
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("{}, world!", s1); // 错误！s1 已经失效
    // let mut s1 = String::from("hello");
    // s1 = return_str(s1);
    // println!("{}", s1);
    // let i1 = 32;
    // print_int32(i1);
    // println!("{}", i1);
    // let s1 = String::from("hello");
    // let s2 = &s1;
    // println!("{}", s1);
    // println!("{}", s2);
    // let s1 = String::from("hello");
    // let len = get_len(&s1);
    // println!("The length of '{}' is {}.", s1, len)
    // let s1 = String::from("hello");
    // let s2 = &s1;
    // let s3 = s1;
    // println!("{}", s2);
    // let mut s2 = &s1;
    // let s3 = s2;
    // s2 = &s3;
    // println!("{}", s2);
    // let mut s2 = &s1;
    // println!("{}", s2);
    // s2.push_str("oob");
    // println!("{}", s2);
    // let mut s1 = String::from("run");
    // let s2 = &mut s1;
    // let s3 = &mut s1;
    // s2.push_str("oob");
    // println!("{}", s2);
    // let s = String::from("helloworld");
    // let part1 = &s[0..5];
    // let part2 = &s[5..10];
    // let part3 = &s[..];
    // println!("{},{},{}", part1, part2, part3);
    // let arr = [1, 3, 5, 7, 9];
    // let part = &arr[..];
    // for i in part.iter() {
    //     println!("{}", i);
    // }
    // let user = Person {
    //     name: String::from("jack"),
    //     age: 18,
    //     score: 99,
    // };
    // println!("name={},age={},score={}", user.name, user.age, user.score);
    // let student = Person {
    //     score: 100,
    //     ..user
    // };
    // println!("name={},age={},score={}", student.name, student.age, student.score);
    // println!("student is {:#?}", student);
    // student.display();
    // let color1 = Color(1, 2, 3);
    // println!("{:#?}", color1);
    // let rectangle1 = Rectangle::create(32,40);
    // let rectangle2 = Rectangle { width: 31, height: 40 };
    // let area = rectangle1.area();
    // println!("{}", area);
    // let wider = rectangle1.wider(&rectangle2);
    // println!("{}", wider);
    // let book = Book::Papery { index: 1001 };
    // let ebook = Book::Electronic { url: String::from("url...") };
    // // println!("{:#?}", book);
    // match book {
    //     Book::Papery { index } => {
    //         println!("Papery book {}", index);
    //     }
    //     Book::Electronic { url } => {
    //         println!("Electronic book {}", url);
    //     }
    // }

    // println!("{:#?}", Book::Electronic(String::from("hello")));
    // let t = "abc";
    // match t {
    //     "abc" => {
    //         println!("{}", "abc yyds")
    //     }
    //     _ => {
    //         println!("{}", "no abc")
    //     }
    // }
    // let opt = Option::Some("hello");
    let opt: Option<&str> = Option::None;
    // println!("{:?}", opt);
    match opt {
        Option::Some(something) => {
            println!("{}", something)
        }
        Option::None => {
            println!("{}", "is None")
        }
    }
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}


#[derive(Debug)]
enum Book {
    Papery { index: u32 },
    Electronic { url: String },
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}

#[derive(Debug)]
struct Color(u8, u8, u8);

struct Point(f64, f64);

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    score: u32,
}

impl Person {
    fn display(&self) {
        println!("{},{},{}", self.name, self.age, self.score)
    }
}


fn get_len(s: &String) -> usize {
    s.len()
}

fn return_str(s: String) -> String {
    return s;
}

fn print_int32(i: i32) {
    println!("{}", i);
}

fn print_str(s: String) {
    println!("{}", s);
}

// fn add(a: i32, b: i32) -> i32 {
//     return a + b;
// }


// fn swap(a: i32, b: i32) -> (i32, i32) {
//     return (b, a);
// }

