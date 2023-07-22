use core::time;
use std::fs;
use std::thread::sleep;

// Q1
pub fn get_current_dir_file_info(arg: &str) {
    println!("get path[{arg}] files/folders:");
    for file in fs::read_dir(arg).unwrap() {
        println!("{} ", file.unwrap().path().display());
    }
    println!("");
}

// Q2: get stack pointer from asm -> rust
// hint website
// https://blogs.oracle.com/linux/post/unwinding-stack-frame-pointers-and-orc
// https://blog.csdn.net/weixin_41519463/article/details/122203481
pub fn get_stack_pointer() -> u64 {
    return 1;
}
// Q3: no os support scheduling, just use rust api instead.
pub fn sleep_5_secs() {
    println!("\nsleep for 5 x 100 ms:");
    let one_hundred = time::Duration::from_millis(100);
    for turn in 0..5 {
        sleep(one_hundred);
        println!("sleep {} x 100 m_secs.", turn + 1);
    }
}
