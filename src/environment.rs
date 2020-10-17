pub struct Bounce {
    width: usize,
    height: usize,
}

impl Bounce {
    pub fn new(width: usize, height: usize) -> Bounce {
        Bounce { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounce_sets_correct_width_and_height() {
        let board_width = 10;
        let board_height = 30;
        let b1 = Bounce::new(board_width, board_height);

        assert_eq!(board_width, b1.width);
        assert_eq!(board_height, b1.height);
    }
}
