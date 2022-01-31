use rand;
use std::{
    env, fs,
    io::{self, Write},
    mem,
    ops::{Index, IndexMut},
    thread, time,
};

#[derive(Clone, Copy, Debug)]
enum Cell {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<Cell>>);

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        let vs = vec![vec![Cell::Dead; cols]; rows];
        Grid(vs)
    }
}

impl Index<usize> for Grid {
    type Output = Vec<Cell>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

struct Game {
    rows: usize,
    cols: usize,
    front_buf: Grid,
    back_buf: Grid,
}

impl Game {
    fn new(rows: usize, cols: usize) -> Self {
        Game {
            rows,
            cols,
            front_buf: Grid::new(rows, cols),
            back_buf: Grid::new(rows, cols),
        }
    }

    fn from(path: &String, padding: usize) -> Self {
        // load .cells file
        // find the rows and columns for the grid
        // fill new grid from file

        let file = fs::read_to_string(path).expect("could not read file");

        let rows = file.lines().count() + padding * 2;

        let cols = if let Some(line) = file.lines().max_by(|x, y| x.len().cmp(&y.len())) {
            line.len() + padding * 2
        } else {
            0
        };

        let mut front_buf = Grid::new(rows, cols);

        for (i, line) in file.lines().enumerate() {
            for (j, chr) in line.chars().enumerate() {
                if chr == 'O' {
                    front_buf[i + padding][j + padding] = Cell::Alive;
                }
            }
        }

        Game {
            rows,
            cols,
            front_buf,
            back_buf: Grid::new(rows, cols),
        }
    }

    fn fill_random(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.front_buf[i][j] = if rand::random() {
                    Cell::Alive
                } else {
                    Cell::Dead
                };
            }
        }
    }

    fn count_neighbors(&self, i: usize, j: usize) -> i32 {
        let neighbors = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut total = 0;

        for (di, dj) in neighbors {
            let ni = i as i32 - di;
            let nj = j as i32 - dj;

            if ni < 0 || nj < 0 || ni >= self.rows as i32 || nj >= self.cols as i32 {
                continue;
            }

            match self.front_buf[ni as usize][nj as usize] {
                Cell::Alive => total += 1,
                Cell::Dead => {}
            }
        }
        total
    }

    fn update(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                match (&self.front_buf[i][j], self.count_neighbors(i, j)) {
                    (Cell::Alive, 2 | 3) => self.back_buf[i][j] = Cell::Alive,
                    (Cell::Dead, 3) => self.back_buf[i][j] = Cell::Alive,
                    _ => self.back_buf[i][j] = Cell::Dead,
                }
            }
        }
        mem::swap(&mut self.front_buf, &mut self.back_buf);
    }

    fn show(&self) {
        io::stdout().flush().expect("could not flush");
        print!("{}[2J", 27 as char); // clear screen
        for i in 0..self.rows {
            for j in 0..self.cols {
                match self.front_buf[i][j] {
                    Cell::Alive => print!("O "),
                    Cell::Dead => print!(". "),
                }
            }
            println!();
        }
    }

    fn set(&mut self, i: usize, j: usize, state: Cell) {
        self.front_buf[i][j] = state;
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut game = if args.len() == 1 {
        let mut g = Game::new(30, 30);
        g.fill_random();
        g
    } else {
        Game::from(&args[1], 10)
    };

    println!("starting game {}: {} x {}", args[0], game.rows, game.cols);

    let mut count = 0;
    loop {
        game.show();
        game.update();
        count += 1;
        if count == 10_000 {
            break;
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}
