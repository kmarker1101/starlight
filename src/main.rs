fn main() {
    println!("Hello, world!");
    println!("Just a test for Github actions to Main!");
    let needle = 0xCB;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in haystack.iter() {
        if *item == needle {
            println!("{}", item);
        }
    }
}
