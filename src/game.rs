mod simulation_logic;

use crate::cli::{print_world, reset_console};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

pub use simulation_logic::do_step;

pub struct Game {
    pub world: Vec<Vec<u8>>,
    pub buffer: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Game {
    pub fn new(rows: usize, cols: usize, portion_alive: f64) -> Game {
        let world = Game::generate_world(rows, cols, portion_alive);
        let buffer: Vec<Vec<u8>> = vec![vec![0; cols]; rows];

        Game {
            world,
            buffer,
            rows,
            cols,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn swap_world_and_buffer(&mut self) {
        std::mem::swap(&mut self.world, &mut self.buffer);
    }

    fn generate_world(rows: usize, cols: usize, portion_alive: f64) -> Vec<Vec<u8>> {
        let mut world = vec![Vec::<u8>::with_capacity(cols); rows];
        for row in world.iter_mut() {
            for _ in 0..cols {
                row.push(thread_rng().gen_bool(portion_alive) as u8);
            }
        }

        world
    }
}

pub fn get_stop_condition(seconds: Option<u32>) -> Box<dyn Fn() -> bool> {
    match seconds {
        Some(s) => {
            let now = Instant::now();
            let ending_time = now + Duration::from_secs(s.into());
            Box::new(move || Instant::now() > ending_time)
        }
        None => Box::new(|| false),
    }
}

pub fn run(game: &mut Game, should_stop: Box<dyn Fn() -> bool>) {
    while !should_stop() {
        reset_console();
        print_world(&game.world);
        do_step(game);
        game.swap_world_and_buffer();
        std::thread::sleep(std::time::Duration::from_secs_f64(0.5));
    }
}
