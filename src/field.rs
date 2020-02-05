use crate::figures::*;

pub struct Field {
    width: i32,
    height: i32,
    tiles: Vec<Vec<bool>>,
}

impl Field {
    pub fn new(width: i32, height: i32) -> Field {
        let mut field = Field { width: width,
                                height: height,
                                tiles: Vec::new() };

        for _ in 0..width {
            field.tiles.push(Vec::new());
            let vec2 = field.tiles.last_mut().unwrap();

            for _ in 0..height {
                vec2.push(false);
            }
        }

        return field;
    }

    pub fn opened(&self, x: i32, y: i32) -> bool {
        return self.tiles[x as usize][y as usize];
    }

    pub fn clear_lines(&mut self) {
        for i in 0..self.height-1 {
            while self.check_line(i, true) {
                self.clear_line(i);

                let mut j = i + 1;

                while !self.check_line(j, false) {
                    self.move_line(j, -1);
                    j += 1;
                }
            }
        }
    }

    pub fn place(&mut self, figure: &Figure) {
        for i in 0..figure.height() {
            for j in 0..figure.width() {
                let x = figure.x() - i;
                let y = figure.y() - j;

                if figure.opened(i, j) {
                    self.set(x, y, true);
                }
            }
        }
    }

    pub fn width(&self) -> i32 {
        return self.width;
    }

    pub fn height(&self) -> i32 {
        return self.height;
    }

    fn set(&mut self, x: i32, y: i32, opened: bool) {
        self.tiles[x as usize][y as usize] = opened;
    }

    fn check_line(&self, line: i32, mode: bool) -> bool {
        for i in 0..self.width {
            if mode ^ self.opened(i, line) {
                return false;
            }
        }

        return true;
    }

    fn clear_line(&mut self, line: i32) {
        for i in 0..self.width {
            self.set(i, line, false);
        }
    }

    fn move_line(&mut self, line: i32, delta_y: i32) {
        for i in 0..self.width {
            self.set(i, line + delta_y, self.opened(i, line));
            self.set(i, line, false);
        }
    }
}
