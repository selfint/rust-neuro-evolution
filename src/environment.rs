#[derive(Debug)]
enum BounceTile {
    Empty,
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
        let ground_height = height / 3;

        for _ in 0..width {
            let mut row = vec![];

            for i in 0..height {
                if i == ground_height {
                    row.push(BounceTile::Ground);
                } else {
                    row.push(BounceTile::Empty)
                }
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
        for col in b.board {
            assert_eq!(board_height, col.len());
        }
    }

    #[test]
    fn bounce_generates_ground() {
        let board_width = 10;
        let board_height = 30;
        let b = Bounce::new(board_width, board_height);

        for tile_col in b.board {
            for (tile_height, tile) in tile_col.into_iter().enumerate() {
                if tile_height == 10 {
                    assert!(matches!(tile, BounceTile::Ground));
                }
            }
        }
    }

    #[test]
    fn bounce_generates_empty() {
        let board_width = 10;
        let board_height = 30;
        let b = Bounce::new(board_width, board_height);

        for tile_col in b.board {
            for (tile_height, tile) in tile_col.into_iter().enumerate() {
                if tile_height == 10 {
                    continue;
                }

                assert!(matches!(tile, BounceTile::Empty));
            }
        }
    }
}
