mod export;
mod group;
mod har;

use export::export;
use group::GroupData;
use std::{env::args, fs::File, println};

fn main() {
    let http_data =
        har::load_har(File::open(args().nth(1).expect("Expect har file name")).unwrap());
    let list_info = GroupData::from_har(&http_data);
    for i in &http_data {
        if let Err(e) = export(&list_info, i) {
            println!("Error exporting url {}: {}", i.url, e);
        }
    }
}
