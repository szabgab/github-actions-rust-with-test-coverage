fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[allow(dead_code)]
fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }
    // #[test]
    // fn multiply_works() {
    //     assert_eq!(multiply(2, 3), 6);
    // }
}
