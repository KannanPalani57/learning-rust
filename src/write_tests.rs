
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2,2);
        assert_eq!(result, 4);
    }
}  