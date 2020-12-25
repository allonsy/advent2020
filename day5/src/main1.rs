use util;

struct Seat {
    row: usize,
    seat: usize,
}

impl Seat {
    fn parse_seat(seat_str: String) -> Seat {
        let mut row_pow = 6;
        let mut seat_pow = 2;
        let mut row = 0;
        let mut seat = 0;

        for ch in seat_str.chars() {
            match ch {
                'F' => {
                    row_pow -= 1;
                }
                'B' => {
                    row += 1 << row_pow;
                    row_pow -= 1;
                }
                'L' => {
                    seat_pow -= 1;
                }
                'R' => {
                    seat += 1 << seat_pow;
                    seat_pow -= 1;
                }
                _ => panic!("Unknown char {} read in", ch),
            }
        }

        Seat { row, seat }
    }

    fn get_id(&self) -> usize {
        self.row * 8 + self.seat
    }
}

fn main() {
    let lines = util::read_file_lines("input.txt");
    let mut max_id = 0;

    for line in lines {
        let seat = Seat::parse_seat(line);
        let id = seat.get_id();
        if id > max_id {
            max_id = id;
        }
    }

    println!("{}", max_id);
}
