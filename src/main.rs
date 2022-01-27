#[derive(Clone, Copy, Debug)]
enum Cell {
    Alive,
    Dead,
}

struct Game<const W: usize, const H: usize> {
    grid: [[Cell; H]; W],
}

impl<const W: usize, const H: usize> Game<W, H> {
    fn new() -> Self {
        Game {
            grid: [[Cell::Dead; H]; W],
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

            if ni < 0 || nj < 0 || ni >= H as i32 || nj >= W as i32 {
                continue;
            }

            match self.grid[ni as usize][nj as usize] {
                Cell::Alive => total += 1,
                Cell::Dead => {}
            }
        }
        total
    }

    fn update(&mut self) {
        let mut buf = [[Cell::Dead; H]; W];
        for i in 0..W {
            for j in 0..H {
                match (&self.grid[i][j], self.count_neighbors(i, j)) {
                    (Cell::Alive, 2 | 3) => buf[i][j] = Cell::Alive,
                    (Cell::Dead, 3) => buf[i][j] = Cell::Alive,
                    (_, _) => {}
                }
            }
        }
        self.grid = buf;
    }

    fn show(&self) {
        for i in 0..W {
            for j in 0..H {
                match self.grid[i][j] {
                    Cell::Alive => print!("#"),
                    Cell::Dead => print!("."),
                }
            }
            println!();
        }
        println!();
    }

    fn set(&mut self, x: usize, y: usize, state: Cell) {
        self.grid[x][y] = state;
    }
}

fn main() {
    let mut game = Game::<10, 10>::new();
    game.set(5, 3, Cell::Alive);
    game.set(5, 4, Cell::Alive);
    game.set(5, 5, Cell::Alive);
    let mut count = 0;
    loop {
        game.show();
        game.update();
        count += 1;
        if count == 5 {
            break;
        }
    }
}
