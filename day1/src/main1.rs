use util;

fn main() {
    let str_nums = util::read_file_lines("input.txt");
    let nums = util::parse_int_list(str_nums);

    for i in nums.iter() {
        for j in nums.iter() {
            if i + j == 2020 {
                println!("{}", i * j);
                return;
            }
        }
    }
}
