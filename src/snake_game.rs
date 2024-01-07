use crate::vga_buffer::{Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};
use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    pub static ref SNAKE_GAME: Mutex<SnakeGame> = Mutex::new(SnakeGame::new((80, 25)));
}

#[derive(PartialEq, Copy, Clone)]
pub struct Point(u16, u16);
const MAX_SNAKE_LENGTH: usize = 256;
pub struct SnakeGame {
    snake: [Point; MAX_SNAKE_LENGTH],
    length: usize,
    pub(crate) direction: Direction,
    food: Point,
    board_size: (u16, u16),
}
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeGame {
    pub fn new(board_size: (u16, u16)) -> Self {
        let mid_x = board_size.0 / 2;
        let mid_y = board_size.1 / 2;

        let mut snake = [Point(mid_x, mid_y); MAX_SNAKE_LENGTH];
        snake[0] = Point(mid_x, mid_y);

        Self {
            snake,
            length: 1,
            direction: Direction::Right,
            food: Point(mid_x - 5, mid_y),
            board_size,
        }
    }

    fn place_food(&mut self) {
        static mut SEED: u16 = 0x_A5A5;

        unsafe {
            SEED ^= SEED << 7;
            SEED ^= SEED >> 9;
            SEED ^= SEED << 8;
            let new_x = SEED % BUFFER_WIDTH as u16;
            SEED ^= SEED << 7;
            SEED ^= SEED >> 9;
            SEED ^= SEED << 8;
            let new_y = SEED % BUFFER_HEIGHT as u16;

            let new_food = Point(new_x, new_y);

            if !self.snake[..self.length].contains(&new_food) {
                self.food = new_food;
            } else {
                SEED = SEED.wrapping_add(1);
                self.place_food();
            }
        }
    }

    fn check_collision(&self, new_head: &Point) -> bool {
        if self.snake[..self.length].contains(new_head) {
            return true;
        }

        if new_head.0 >= self.board_size.0 || new_head.1 >= self.board_size.1 {
            return true;
        }

        false
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.board_size);
    }

    pub fn update(&mut self) {
        let new_head = match self.direction {
            Direction::Up => Point(self.snake[0].0, self.snake[0].1.wrapping_sub(1)),
            Direction::Down => Point(self.snake[0].0, self.snake[0].1.wrapping_add(1)),
            Direction::Left => Point(self.snake[0].0.wrapping_sub(1), self.snake[0].1),
            Direction::Right => Point(self.snake[0].0.wrapping_add(1), self.snake[0].1),
        };

        if new_head == self.food {
            if self.length < MAX_SNAKE_LENGTH {
                self.length += 1;
            }

            self.place_food();
        } else if !self.check_collision(&new_head) {
            for idx in (1..self.length).rev() {
                self.snake[idx] = self.snake[idx - 1];
            }
            self.snake[0] = new_head;
        } else {
            self.reset();
        }
    }

    pub fn render(&self) {
        let mut writer = WRITER.lock();

        writer.clear_screen();

        for point in &self.snake[..self.length] {
            writer.write_char(
                point.1 as usize,
                point.0 as usize,
                0xFE,
                ColorCode::new(Color::LightGreen, Color::Black),
            );
        }

        writer.write_char(
            self.food.1 as usize,
            self.food.0 as usize,
            b'*',
            ColorCode::new(Color::Red, Color::Black),
        );
    }
}
