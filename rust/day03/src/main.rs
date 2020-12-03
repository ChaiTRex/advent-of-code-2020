#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Space {
    Open,
    Tree,
}

impl Space {
    pub fn new(ch: char) -> Self {
        match ch {
            '.' => Self::Open,
            '#' => Self::Tree,
            _ => panic!("Bad input: {}", ch),
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::Open => true,
            Self::Tree => false,
        }
    }

    pub fn is_tree(&self) -> bool {
        match self {
            Self::Open => false,
            Self::Tree => true,
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Space>>,
    width: usize,
}

impl Grid {
    pub fn new(str: &str) -> Grid {
        let grid: Vec<Vec<Space>> = str
            .lines()
            .map(|line| line.chars().map(Space::new).collect())
            .collect();
        let width = grid.iter().next().unwrap().iter().count();

        Grid { grid, width }
    }

    pub fn space_at(&self, x: usize, y: usize) -> Option<Space> {
        self.grid
            .get(y)
            .and_then(|row| row.get(x % self.width).map(|&space| space))
    }

    pub fn trees_hit(&self, right: usize, down: usize) -> usize {
        (0..)
            .step_by(right)
            .zip((0..).step_by(down))
            .map(|(x, y)| self.space_at(x, y))
            .take_while(Option::is_some)
            .flatten()
            .filter(Space::is_tree)
            .count()
    }
}

fn main() {
    let grid = Grid::new(&std::fs::read_to_string("../03.txt").unwrap());

    println!("{}", grid.trees_hit(3, 1));

    println!(
        "{}",
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&(right, down)| grid.trees_hit(right, down) as u128)
            .product::<u128>()
    )
}
