use std::time::Duration;
use std::rc::Rc;
use rand::rngs::ThreadRng;

use glad_gl::gl;
use glfw::{ Action, Key };

use crate::field::*;
use crate::figures::*;

pub struct Game {
    field: Field,
    want_exit: bool,
    gravity_elapsed: Duration,
    move_elapsed: Duration,
    figure: Figure,
    next_figure: Figure,
    speed: Duration,
    repeat_speed: Duration,
    right_pressed: bool,
    left_pressed: bool,
    down_pressed: bool,
    random: ThreadRng,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            field: Field::new(12, 20),
            want_exit: false,
            gravity_elapsed: Duration::new(0, 0),
            move_elapsed: Duration::new(0, 0),
            figure: Figure::new(&FigureType::L),
            next_figure: Figure::new(&FigureType::L),
            speed: Duration::from_millis(2000),
            repeat_speed: Duration::from_millis(100),
            right_pressed: false,
            left_pressed: false,
            down_pressed: false,
            random: rand::thread_rng(),
        };

        game.gen_figure(true);

        return game;
    }

    pub fn handle_event(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::E, _, Action::Press, _) => {
                self.figure.rotate(&self.field, RotateDirection::CW);
            }

            glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => {
                self.figure.rotate(&self.field, RotateDirection::CCW);
            }

            glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                self.right_pressed = true;
            }

            glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
                self.right_pressed = false;
            }

            glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                self.left_pressed = true;
            }

            glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
                self.left_pressed = false;
            }

            glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                self.down_pressed = true;
            }

            glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
                self.down_pressed = false;
            }

            glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                self.want_exit = true;
            }

            _ => {}
        }
    }

    pub fn tick(&mut self, delta_time: std::time::Duration) {
        self.gravity_elapsed += delta_time;
        self.move_elapsed += delta_time;

        if self.move_elapsed > self.repeat_speed {
            if self.right_pressed {
                if self.figure.r#move(&self.field, 1, 0) {
                    self.move_elapsed = Duration::new(0, 0);
                }
            }

            if self.left_pressed {
                if self.figure.r#move(&self.field, -1, 0) {
                    self.move_elapsed = Duration::new(0, 0);
                }
            }
        }

        let wait = if self.down_pressed {
            self.repeat_speed
        } else {
            self.speed
        };

        if self.gravity_elapsed > wait {
            if !self.figure.r#move(&self.field, 0, -1) {
                self.field.place(&self.figure);
                self.gen_figure(false);
                self.field.clear_lines();
            }

            self.gravity_elapsed = Duration::new(0, 0);
        }
    }

    #[allow(dead_code)]
    pub fn draw_debug(&self, window: &glfw::Window) {
        let field = &self.field;
        let cell_size = 24;

        unsafe {
            gl::Begin(gl::GL_QUADS);
            gl::Color3f(1.0, 1.0, 1.0);
        }

        for i in 0..field.height() {
            for j in 0..field.width() {
                if self.opened(j, i) {
                    unsafe {
                        gl::Vertex2i(j * cell_size, i * cell_size);
                        gl::Vertex2i(j * cell_size, (i + 1) * cell_size);
                        gl::Vertex2i((j + 1) * cell_size, (i + 1) * cell_size);
                        gl::Vertex2i((j + 1) * cell_size, i * cell_size);
                    }
                }
            }
        }

        unsafe {
            gl::End();
        }
    }

    #[allow(dead_code)]
    pub fn draw(&self, window: &glfw::Window) {
        let field = &self.field;
        let cell_size = 24;
        let dr_size = (field.width() / 3 * cell_size, field.height() * cell_size);
        let next_figure = &self.next_figure;

        let mut offset = window.get_size();

        offset.0 = (offset.0 - dr_size.0) / 2;
        offset.1 = (offset.1 - dr_size.1) / 2;

        let offset = offset;

        let border = &[
            (offset.0 - 1,             offset.1 - 2),
            (offset.0 - 1,             offset.1 + 1 + dr_size.1),
            (offset.0 + 2 + dr_size.0, offset.1 + 1 + dr_size.1),
            (offset.0 + 2 + dr_size.0, offset.1 - 2),
        ];

        unsafe {
            gl::Color3f(1.0, 1.0, 1.0);
            gl::Begin(gl::GL_LINE_STRIP);

            for i in border {
                gl::Vertex2i(i.0, i.1);
            }

            gl::Vertex2i(border[0].0, border[0].1);

            gl::End();
            gl::Begin(gl::GL_QUADS);
        }

        for i in 0..field.height() {
            for j in (0..field.width()).step_by(3) {
                let red   = if self.opened(j,     i) { 1.0 } else { 0.0 };
                let green = if self.opened(j + 1, i) { 1.0 } else { 0.0 };
                let blue  = if self.opened(j + 2, i) { 1.0 } else { 0.0 };

                unsafe {
                    gl::Color3f(red, green, blue);

                    gl::Vertex2i(j / 3       * cell_size + offset.0,
                                 i           * cell_size + offset.1);

                    gl::Vertex2i(j / 3       * cell_size + offset.0,
                                 (i + 1)     * cell_size + offset.1);

                    gl::Vertex2i((j / 3 + 1) * cell_size + offset.0,
                                 (i + 1)     * cell_size + offset.1);

                    gl::Vertex2i((j / 3 + 1) * cell_size + offset.0,
                                 i           * cell_size + offset.1);
                }
            }
        }

        let offset = (offset.0 + dr_size.0 + cell_size, offset.1 + dr_size.1 - 4 * cell_size);

        unsafe {
            gl::End();
            gl::Begin(gl::GL_QUADS);
        }

        for i in 0..next_figure.height() {
            for j in (0..next_figure.width()).step_by(3) {
                let red   = if next_figure.opened(j,     i) { 1.0 } else { 0.0 };
                let green = if j + 1 < next_figure.width() && next_figure.opened(j + 1, i) { 1.0 } else { 0.0 };
                let blue  = if j + 2 < next_figure.width() && next_figure.opened(j + 2, i) { 1.0 } else { 0.0 };

                if red != 0.0 || green != 0.0 || blue != 0.0 {
                    unsafe {
                        gl::Color3f(red, green, blue);

                        gl::Vertex2i(j / 3       * cell_size + offset.0,
                                     i           * cell_size + offset.1);

                        gl::Vertex2i(j / 3       * cell_size + offset.0,
                                     (i + 1)     * cell_size + offset.1);

                        gl::Vertex2i((j / 3 + 1) * cell_size + offset.0,
                                     (i + 1)     * cell_size + offset.1);

                        gl::Vertex2i((j / 3 + 1) * cell_size + offset.0,
                                     i           * cell_size + offset.1);
                    }
                }
            }
        }

        unsafe {
            gl::End();
        }
    }

    pub fn is_want_exit(&self) -> bool {
        return self.want_exit;
    }

    fn opened(&self, x: i32, y: i32) -> bool {
        let localx = self.figure.x() - x;
        let localy = self.figure.y() - y;

        if  localx < 0 ||
            localy < 0 ||
            localx >= self.figure.width() ||
            localy >= self.figure.height()
        {
            return self.field.opened(x, y);
        } else {
            return self.field.opened(x, y) || self.figure.opened(localx, localy);
        }
    }

    fn gen_figure(&mut self, first_time: bool) {
        use rand::seq::SliceRandom;

        let iters = if first_time { 2 } else { 1 };

        for i in 0..iters {
            std::mem::swap(&mut self.figure, &mut self.next_figure);

            self.next_figure = Figure::new(
                &FigureType::all()
                    .choose(&mut self.random)
                    .expect("Slice is empty")
                    .clone()
            );
        }
    }
}
