fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 3), 5);
    }
}
