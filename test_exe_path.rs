fn main() {
    let mut exe = std::env::current_exe().unwrap();
    exe.pop();
    exe.pop();
    println!("{:?}", exe);
}
