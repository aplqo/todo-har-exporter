extern crate lazy_static;
extern crate regex;

use crate::{group::GroupData, har::APIResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{create_dir_all, OpenOptions},
    io::{Error, Write},
    path::PathBuf,
};

lazy_static! {
    static ref TASKLIST_REGEX:Regex = Regex::new(r#"https://substrate.office.com/todo/api/v1/taskfolders/(.+)/tasks\?\$expand=LinkedEntity&\$select=\*"#).unwrap();
}

pub fn export(group_info: &GroupData, task_list: &APIResult) -> Result<(), Error> {
    let task = match TASKLIST_REGEX.captures(&task_list.url) {
        Some(c) => group_info.get_tasklist(c.get(1).unwrap().as_str()),
        None => return Ok(()),
    };
    let mut buf = PathBuf::from("./json");
    if let Some(g) = task.group {
        buf.push(g);
    }
    buf.push(task.name);
    create_dir_all(buf.parent().unwrap())?;
    OpenOptions::new()
        .create_new(true)
        .open(buf.as_path())?
        .write(task_list.result.as_bytes())
        .map(|_| ())
}
