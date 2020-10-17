enum BounceTile {
    Ground,
}

pub struct Bounce {
    width: usize,
    height: usize,
    board: Vec<Vec<BounceTile>>,
}

impl Bounce {
    pub fn new(width: usize, height: usize) -> Bounce {
        Bounce {
            width,
            height,
            board: vec![vec![BounceTile::Ground]],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounce_sets_correct_width_and_height() {
        let board_width = 10;
        let board_height = 30;
        let b = Bounce::new(board_width, board_height);

        assert_eq!(board_width, b.width);
        assert_eq!(board_height, b.height);
    }

    #[test]
    fn bounce_generates_ground() {
        let board_width = 10;
        let board_height = 30;
        let b = Bounce::new(board_width, board_height);

        for tile_row in b.board {
            for tile in tile_row {
                assert!(matches!(BounceTile::Ground, tile));
            }
        }
    }
}
