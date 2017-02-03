extern crate uuid;

mod kernel;

use kernel::types::{ CellType,
                     CellFormat,
                     RecFormat,
                     Rec,
                     RecCollection };

fn main() {

    {
        let cell_format = CellFormat { fieldname: "id".to_string(),
                                       fieldtype: CellType::Int32(None)
                                     };

        let rec_format = RecFormat::new(vec![cell_format]);
        let rec = Rec::new(rec_format);
    }

    println!("Chpokey!");
}
