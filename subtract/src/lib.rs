pub fn main() {
    println!("In Subtract");
}

#[cfg(test)]
mod test {

    #[test]
    fn right_test() {
        assert_eq!(2, 2);
    }
    
    #[test]
    fn wrong_test() {
        assert_eq!(2, 4);
    }
}
