
fn getlegalnuber(sudoku: [[i32; 9]; 9], cell: (i32, i32)) -> [i32; 9] {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    // row
    let row = sudoku[0];
    for i in row {
        if i != 0 {
            arr[(i - 1) as usize] = 0;
        }
    }

    // collums
    for i in sudoku {
        if i[cell.0 as usize] != 0 {
            arr[(i[cell.0 as usize] - 1) as usize] = 0;
        }
    }

    //box
    let cellstart = (cell.0 - cell.0 % 3, cell.0 - cell.0 % 3);
    for i in cellstart.0..(cellstart.0 + 3) {
        for ii in cellstart.1..(cellstart.1 + 3) {

            let num = sudoku[i as usize][ii as usize];
            if num != 0 {
                arr[(num - 1) as usize] = 0;
            }
        }
    }

    arr
}

fn main() {
    println!("Hello, world!");

    let mut sudoku: [[i32; 9]; 9] = [
        [9, 8, 5,   4, 0, 1,    0, 0, 0],
        [0, 0, 0,   0, 3, 0,    0, 0, 0],
        [1, 0, 6,   0, 0, 0,    0, 0, 0],

        [0, 0, 0,   5, 0, 0,    0, 0, 0],
        [4, 0, 2,   0, 0, 9,    0, 0, 3],
        [0, 9, 0,   0, 6, 3,    4, 0, 0],

        [0, 6, 0,   0, 1, 0,    0, 0, 0],
        [0, 0, 0,   3, 0, 6,    0, 0, 5],
        [2, 0, 0,   0, 8, 0,    0, 0, 1],
    ];

    let sus = getlegalnuber(sudoku, (0,0));
    println!("sus: {:?}", sus);
}
