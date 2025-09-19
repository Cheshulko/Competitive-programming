// https://leetcode.com/problems/design-spreadsheet

type Cell = (usize, usize);

struct Spreadsheet {
    table: Vec<[i32; 26]>,
}

impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            table: vec![[0; 26]; 1 + rows as usize],
        }
    }

    fn parse_cell<S: AsRef<str>>(&self, cell: S) -> Cell {
        let cell = cell.as_ref();

        let col = (cell.as_bytes()[0] - b'A') as usize;
        let row = cell[1..].parse::<usize>().unwrap();

        (row, col)
    }

    fn get_maybe_cell_value<S: AsRef<str>>(&self, s: S) -> i32 {
        let s = s.as_ref();

        if matches!(s.as_bytes()[0], b'A'..=b'Z') {
            let (row, col) = self.parse_cell(s);

            self.table[row][col]
        } else {
            s.parse::<i32>().unwrap()
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        let (row, col) = self.parse_cell(cell);

        self.table[row][col] = value;
    }

    fn reset_cell(&mut self, cell: String) {
        let (row, col) = self.parse_cell(cell);

        self.table[row][col] = 0;
    }

    fn get_value(&self, formula: String) -> i32 {
        let mut formula = formula[1..].split("+");

        let cell_1 = formula.next().unwrap();
        let cell_2 = formula.next().unwrap();

        self.get_maybe_cell_value(cell_1) + self.get_maybe_cell_value(cell_2)
    }
}
