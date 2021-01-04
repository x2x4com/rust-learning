use std::io;
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;


fn main() {

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Char debug: {}, {}, {}", c, z, heart_eyed_cat);

    let t = true;
    let f: bool = false;

    println!("t: {}, f: {}", t, f);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("tup: {}", tup.0);

    // 8.3 example

    let text = "hello world wonderful world";
    
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        // println!("{}", word);
        // 如果存在就返回当前值(可变借用，指向内存地址)，不存在就插入0
        let count = map.entry(word).or_insert(0);
        // println!("{}", count);
        // 因为是可变的借用，指向的内存，所以+1直接影响map对象
        *count += 1;
        // println!("{:?}", map);
    }
    
    println!("{:?}", map);

    // 8.3 over

    // 9.2 example
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };

    // 9.2 over

    // guess start

    println!("猜猜看");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("秘密数字是: {}", secret_number);

    let mut count_loop = 1;

    loop {
        println!("第{}次，请输入一个数字：", count_loop);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("输入不对哦！");

        // let guess: u32 = guess.trim().parse().expect("输入错误，只能输入数字");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你输入的是: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                println!("太小了");
                count_loop += 1;
            },
            std::cmp::Ordering::Greater => {
                println!("太大了");
                count_loop += 1;
            },
            std::cmp::Ordering::Equal => {
                println!("刚好，你赢了，一共用了{}次", count_loop);
                break;
            }
        }

    }


}
