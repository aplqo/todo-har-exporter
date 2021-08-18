mod export;
mod group;
mod har;
mod url;

use export::{export_json, export_tasks};
use group::GroupData;
use std::{env::args, fs::File, println};
use url::Type;

fn main() {
    let http_data =
        har::load_har(File::open(args().nth(1).expect("Expect har file name")).unwrap());
    let list_info = GroupData::from_har(&http_data);
    let count = list_info.list_count();
    let success = http_data.into_iter().fold(0usize, |cnt, i| {
        if let Err(e) = match i.action {
            Type::Tasks(ref id) => export_tasks(&list_info, &i, &i.url.as_str()[id.clone()]),
            Type::TaskLists => export_json("taskfolders", i.result.as_str()),
            Type::TaskGroups => export_json("foldergroups", i.result.as_str()),
            _ => Ok(()),
        } {
            println!("Error exporing url {}: {}", i.url, e);
            cnt
        } else if let Type::Tasks(_) = i.action {
            cnt + 1
        } else {
            cnt
        }
    });
    println!("Found tasklist: {}", count);
    println!("Success exported tasklist: {}", success);
}
