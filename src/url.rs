extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;

const TASKFOLDER_URL_PREFIX: &str =
    "https://substrate.office.com/todo/api/v1/taskfolders?$select=*";
const FOLDERGROUP_URL: &str = "https://substrate.office.com/todo/api/v1/foldergroups";

lazy_static! {
    static ref TASKLIST_REGEX:Regex = Regex::new(r#"https://substrate.office.com/todo/api/v1/taskfolders/(.+)/tasks\?\$expand=LinkedEntity&\$select=\*"#).unwrap();
}

#[derive(PartialEq)]
pub enum Type {
    TaskLists,
    Tasks(Range<usize>),
    TaskGroups,
    Other,
}

pub fn get_type(url: &str) -> Type {
    if url == FOLDERGROUP_URL {
        Type::TaskGroups
    } else if url.starts_with(TASKFOLDER_URL_PREFIX) {
        Type::TaskLists
    } else if let Some(c) = TASKLIST_REGEX.captures(url) {
        Type::Tasks(c.get(1).unwrap().range())
    } else {
        Type::Other
    }
}
