fn main() {
    let mut coin: String = String::new();
    let result: Result<usize, Error> = std::io::stdin().read_line(buf: &mut coin);
