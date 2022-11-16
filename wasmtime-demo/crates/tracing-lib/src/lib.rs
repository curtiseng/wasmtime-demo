use tracing::info;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn test() {
    info!("I'm tracing log in lib.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
