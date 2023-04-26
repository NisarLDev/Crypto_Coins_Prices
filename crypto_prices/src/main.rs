fn main() {
    let mut coin: String = String::new();
    let result: Result<usize, Error> = std::io::stdin().read_line(buf: &mut coin);
    match result {
        Ok(nro_bytes: usize) => println!("{nro_bytes}"),
        Err() => todo!(),
    }
}
