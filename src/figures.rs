use crate::field::*;

pub enum RotateDirection {
    CW,
    CCW,
}

#[derive(Clone)]
pub enum FigureType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub struct Figure {
    x: i32,
    y: i32,
    ftype: FigureType,
    width: i32,
    height: i32,
    tiles: Vec<Vec<bool>>,
}

impl FigureType {
    pub fn all() -> &'static [FigureType] {
        &[
            FigureType::I,
            FigureType::O,
            FigureType::T,
            FigureType::S,
            FigureType::Z,
            FigureType::J,
            FigureType::L,
        ]
    }

    fn get_tiles(&self) -> Vec<Vec<bool>> {
        match self {
            FigureType::I => vec![
                vec![false, true, false, false],
                vec![false, true, false, false],
                vec![false, true, false, false],
                vec![false, true, false, false],
            ],

            FigureType::O => vec![
                vec![true, true],
                vec![true, true]
            ],

            FigureType::T => vec![
                vec![false, true, false],
                vec![false, true, true],
                vec![false, true, false],
            ],

            FigureType::S => vec![
                vec![false, true, false],
                vec![false, true, true],
                vec![false, false, true],
            ],

            FigureType::Z => vec![
                vec![false, false, true],
                vec![false, true, true],
                vec![false, true, false],
            ],

            FigureType::J => vec![
                vec![false, true, true],
                vec![false, true, false],
                vec![false, true, false],
            ],

            FigureType::L => vec![
                vec![false, true, false],
                vec![false, true, false],
                vec![false, true, true],
            ],
        }
    }
}

impl Figure {
    pub fn new(ftype: &FigureType) -> Figure {
        let tiles = ftype.get_tiles();

        return Figure {
            x: 6,
            y: 20,
            ftype: ftype.clone(),
            width: tiles.first().unwrap().len() as i32,
            height: tiles.len() as i32,
            tiles: tiles,
        };
    }

    pub fn change(&mut self) {
        for i in 0..self.height {
            for j in i..self.width {
                let i = i as usize;
                let j = j as usize;

                let temp = self.tiles[i][j];

                self.tiles[i][j] = self.tiles[j][i];
                self.tiles[j][i] = temp;
            }
        }
    }

    pub fn flip(&mut self) {
        for i in 0..self.height {
            for j in (self.height / 2)..self.width {
                let revj = (self.width - j - 1) as usize;

                let i = i as usize;
                let j = j as usize;

                let temp = self.tiles[i][j];

                self.tiles[i][j] = self.tiles[i][revj];
                self.tiles[i][revj] = temp;
            }
        }
    }

    pub fn rotate(&mut self, field: &Field, direction: RotateDirection) -> bool {
        match direction {
            RotateDirection::CW => {
                self.change();
                self.flip();
            }

            RotateDirection::CCW => {
                self.flip();
                self.change();
            }
        }

        if self.check_bounds(field) && self.check_collision(field) {
            return true;
        } else {
            self.rotate(
                field,
                match direction {
                    RotateDirection::CW => RotateDirection::CCW,
                    RotateDirection::CCW => RotateDirection::CW,
                },
            );

            return false;
        }
    }

    pub fn opened(&self, x: i32, y: i32) -> bool {
        return self.tiles[x as usize][y as usize];
    }

    pub fn r#move(&mut self, field: &Field, x: i32, y: i32) -> bool {
        self.x += x;
        self.y += y;

        if self.check_bounds(field) && self.check_collision(field) {
            return true;
        } else {
            self.x -= x;
            self.y -= y;

            return false;
        }
    }

    pub fn x(&self) -> i32 {
        return self.x;
    }

    pub fn y(&self) -> i32 {
        return self.y;
    }

    pub fn width(&self) -> i32 {
        return self.width;
    }

    pub fn height(&self) -> i32 {
        return self.height;
    }

    fn check_bounds(&self, field: &Field) -> bool {
        for i in 0..self.height {
            for j in 0..self.width {
                let x = self.x - i;
                let y = self.y - j;

                if self.opened(i, j)
                    && !(x >= 0 && x < field.width() && y >= 0 && y < field.height())
                {
                    return false;
                }
            }
        }

        return true;
    }

    fn check_collision(&self, field: &Field) -> bool {
        for i in 0..self.height {
            for j in 0..self.width {
                let x = self.x - i;
                let y = self.y - j;

                if self.opened(i, j) && field.opened(x, y) {
                    return false;
                }
            }
        }

        return true;
    }
}
