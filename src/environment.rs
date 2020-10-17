pub struct Bounce {
    width: usize,
    height: usize,
}

impl Bounce {
    pub fn new(width: usize, height: usize) -> Bounce {
        Bounce { width, height }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn bounce_generates_correct_world_size() {
        let board_width = 10;
        let board_height = 30;
        let b1 = Bounce::new(board_height, board_width);
        let b2 = Bounce::new(board_height, board_width);

        assert_eq!(board_width, b.width);
    }
}
