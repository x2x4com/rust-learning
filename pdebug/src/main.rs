use futures::executor::block_on;

async fn hello_world() {
    println!("async | hello, world!");
}

struct Solution;
struct Solution1;
struct Solution2;

struct Song(u8);

async fn learn_song() -> Song {
    Song(1)
}
async fn sing_song(song: Song) {
    println!("sing {}", song.0)
}
async fn dance() {
    println!("start dance")
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
    dance().await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

macro_rules! test_role {
    // 参数不需要使用逗号隔开。
    // 参数可以任意组合！
    ($left:expr; and $right:expr) => (
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    );
    // ^ 每个分支都必须以分号结束。
    ($left:expr; or $right:expr) => (
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    );
}

macro_rules! find_min {
    // 基本情形：
    ($x:expr) => ($x);
    // `$x` 后面跟着至少一个 `$y,`
    ($x:expr, $($y:expr),+) => (
        // 对 `$x` 后面的 `$y` 们调用 `find_min!`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    impl Solution1 {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut a = 0;
            let mut b = 0;
            for (index,i) in nums.iter().enumerate() {
                // let e = &target - i;
                let e = nums.iter().position(|&x | x == (&target - i));
                match e {
                    Some(t) => {
                        if t == index {
                            continue
                        }
                        b = t as i32;
                        //println!("b={}", b);
                        //println!("{}", i)
                        // 这里找到了，看下另外一个的id
                        a = index as i32;
                        break
                    },
                    None => {}
                }
            }
            //println!("[{}, {}]", a, b);
            vec![a, b]
        }
    }
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            // (0..) == (0..nums.len() as i32)
            let mut h: Vec<(&i32, i32)> = nums.iter().zip((0..)).collect();
            println!("{:?}", nums.iter().zip((0..)));
            println!("{:?}", h.clone());
            h.sort();
            let mut p: usize = 0;
            let mut q: usize = nums.len().saturating_sub(1);
            loop {
                let s = *h[p].0 + *h[q].0;
                if p >= q {
                    break vec![0, 0];
                }
                //println!("- {} {} {}", p, q, s);
                if s == target {
                    break vec![h[p].1, h[q].1];
                } else if s > target {
                    q -= 1;
                } else {
                    p += 1;
                }
            }
        }
    }

    let a = Solution::two_sum(vec![2,7,11,15,122,333,444], 9);
    println!("[{}, {}]", a[0], a[1]);
    let a = Solution::two_sum(vec![3,2,4], 6);
    println!("[{}, {}]", a[0], a[1]);
    let a = Solution::two_sum(vec![3,3], 6);
    println!("[{}, {}]", a[0], a[1]);
    // let b = (0..);
    // let c = (..);
    // assert_eq!(b, c)
    let a1 = [1, 2, 3, 4, 5, 6, 7, 8];
    let a2: Vec<_> = a1.iter().zip((0..)).collect();
    println!("(0..) => {:?}", a2);
    let a3: Vec<_> = a1.iter().zip((0..3)).collect();
    println!("(0..3) => {:?}", a3);

    let future = hello_world();
    block_on(future);

    block_on(async_main());

    test_role!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test_role!(true; or false);

    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));

    impl Solution2 {
        pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
            let len_t = nums.len();
            for a in 0..len_t {
                for b in a..len_t {
                    if nums[a] > nums[b] {
                        let temp = nums[a];
                        nums[a] = nums[b];
                        nums[b] = temp
                    }
                }
            }
            nums
        }
    }

    let xyz = "66.33";
    let fxy = xyz.parse::<f64>().unwrap();
    let uxy = fxy.round() as u64;
    println!("{:?}", fxy);
    println!("{:?}", uxy);
}
