use std::io::{self, Write};
use std::ops::Add;
enum CellValue {
    None,
    Boolean(bool),
}

struct Cell {
    value: CellValue,
}

pub struct GameField {
    size: u8,
    cells: Vec<Cell>,
}

impl GameField {
    pub fn init() {
        let init_params: &GameField = &GameField {
            size: 3,
            cells: vec![
                Cell { value: CellValue::Boolean(true) },
                Cell { value: CellValue::Boolean(true) },
                Cell { value: CellValue::Boolean(true) },
                Cell { value: CellValue::Boolean(true) },
                Cell { value: CellValue::Boolean(true) },
                Cell { value: CellValue::Boolean(true) },
                Cell { value: CellValue::Boolean(true) },
                Cell { value: CellValue::Boolean(true) },
                Cell { value: CellValue::Boolean(true) },
            ]
        };
        Self::draw_field(init_params);
        let _ = io::stdin().read_line(&mut "".to_string());
    }
    fn draw_field(&self) {
        for (index, line) in self.cells.iter().enumerate() {
            let count: u8 = (index + 1) as u8;
            let mut part_to_print = Self::create_cell_part_of_string(line);
            part_to_print = part_to_print.add(" ");

            print!("{}", part_to_print);

            if count % &self.size == 0 {
                println!();
            }
        }
    }
    fn create_cell_part_of_string(cell: &Cell) -> String {
        match cell.value {
            CellValue::None => String::from("[ ]"),
            CellValue::Boolean(x) => {
                if x {
                    String::from("[X]")
                } else {
                    String::from("[O]")
                }
            }
        }
    }
}