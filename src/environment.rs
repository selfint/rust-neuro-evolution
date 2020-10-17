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
            board: Bounce::initialize_board(width, height),
        }
    }

    fn initialize_board(width: usize, height: usize) -> Vec<Vec<BounceTile>> {
        let mut board = vec![];

        for _ in 0..width {
            let mut row = vec![];

            for _ in 0..height {
                row.push(BounceTile::Ground);
            }
            board.push(row);
        }

        board
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
    fn bounce_generates_correct_board_size() {
        let board_width = 10;
        let board_height = 30;
        let b = Bounce::new(board_width, board_height);

        assert_eq!(board_width, b.board.len());
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
