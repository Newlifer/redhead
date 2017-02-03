extern crate uuid;

mod kernel;

use kernel::types::{CellType,
                    CellFormat,
                    RecFormat,
                    Rec,
                    Table,
                    construct_rec};

fn main() {

    {
        let cell_format = CellFormat{name: "id".to_string(),
                                     type_: CellType::Int32(None)};
        let rec_format = RecFormat{cols: vec![cell_format]};
        let rec = construct_rec(rec_format);
    }

    println!("Chpokey!");
}
