use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num_1 = Number::from(30);
    println!("From is {:?}", num_1);

    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("Into is {:?}", num);
}
