use add_one;

fn main() {
    let num: i32 = 10;

    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
