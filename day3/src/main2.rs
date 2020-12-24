use util;

struct SkiSlope {
    grid: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl SkiSlope {
    fn new(grid: Vec<Vec<bool>>) -> SkiSlope {
        let height = grid.len();
        let width = grid[0].len();

        SkiSlope {
            grid,
            height,
            width,
        }
    }

    fn get_pos(&self, x: usize, y: usize) -> bool {
        let x = x % self.width;
        self.grid[y][x]
    }
}

fn main() {
    let mut grid = Vec::new();
    let lines = util::read_file_lines("input.txt");

    for line in lines {
        let mut row = Vec::new();
        for slot in line.chars() {
            if slot == '#' {
                row.push(true)
            } else {
                row.push(false)
            }
        }
        grid.push(row);
    }

    let ski_slope = SkiSlope::new(grid);
    let mut total: usize = 1;

    let deltas = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for (delta_x, delta_y) in deltas.iter() {
        let mut num_trees = 0;
        let mut cur_x = 0;
        let mut cur_y = 0;

        while cur_y < ski_slope.height {
            if ski_slope.get_pos(cur_x, cur_y) {
                num_trees += 1;
            }

            cur_y += delta_y;
            cur_x += delta_x;
        }

        total *= num_trees;
    }

    println!("{}", total);
}
