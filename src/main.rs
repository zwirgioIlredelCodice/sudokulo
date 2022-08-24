#[derive(Debug)]
struct Cell {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct BestCell {
    cell: Cell,
    legalnums: Vec<u8>,
}

fn getlegalnuber(sudoku: [[u8; 9]; 9], cell: Cell) -> Vec<u8> {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // row
    let row = sudoku[cell.y as usize];
    for i in row {
        if i != 0 {
            arr[(i - 1) as usize] = 0;
        }
    }

    // collums
    for i in sudoku {
        if i[cell.x as usize] != 0 {
            arr[(i[cell.x as usize] - 1) as usize] = 0;
        }
    }

    //box
    let cellstart = (cell.x - cell.x % 3, cell.x - cell.x % 3);
    for i in cellstart.0..(cellstart.0 + 3) {
        for ii in cellstart.1..(cellstart.1 + 3) {
            let num = sudoku[i as usize][ii as usize];
            if num != 0 {
                arr[(num - 1) as usize] = 0;
            }
        }
    }

    let mut v: Vec<u8> = Vec::new();

    for i in arr {
        if i != 0 {
            v.push(i);
        }
    }
    v
}

fn decidecell(sudoku: [[u8; 9]; 9]) -> BestCell {
    let mut bestcell: BestCell = BestCell {
        cell: Cell { x: 255, y: 255 },
        legalnums: vec![],
    };

    let mut minposs = 9;
    let mut v: Vec<u8>;

    for i in 0..9 {
        for ii in 0..9 {
            if sudoku[i as usize][ii as usize] == 0 {
                v = getlegalnuber(sudoku, Cell { x: i, y: ii });
                let len = v.len();

                if len < minposs {
                    minposs = len;
                    bestcell.cell.x = i;
                    bestcell.cell.y = ii;
                    bestcell.legalnums = v;
                }

                if len == 1 {
                    return bestcell;
                }
            }
        }
    }
    bestcell
}

fn solvesudoku(mut sudoku: [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    let mut bestcell: BestCell;

    for i in 0..10 {
        bestcell = decidecell(sudoku);

        if bestcell.legalnums.len() == 1 {
            println!("decidec cell: {:?}", bestcell);
            sudoku[bestcell.cell.y as usize][bestcell.cell.x as usize] = bestcell.legalnums[0];
        } else {
            if bestcell.legalnums.len() == 0 {
                println!("finito");
            }
            break;
        }
    }
    sudoku
}

fn main() {
    println!("Hello, world!");

    let mut sudoku: [[u8; 9]; 9] = [
        [9, 8, 5, 4, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 3, 0, 0, 0, 0],
        [1, 0, 6, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 0, 0, 0, 0],
        [4, 0, 2, 0, 0, 9, 0, 0, 3],
        [0, 9, 0, 0, 6, 3, 4, 0, 0],
        [0, 6, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 3, 0, 6, 0, 0, 5],
        [2, 0, 0, 0, 8, 0, 0, 0, 1],
    ];

    sudoku = solvesudoku(sudoku);
    for row in sudoku {
        println!("{:?}", row);
    }
}
