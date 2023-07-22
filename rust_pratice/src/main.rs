mod tool_linux;

fn main() {
    println!("Hello, world!");
    tool_linux::get_current_dir_file_info("/home/user/Workspace");
    tool_linux::sleep_5_secs();
}
