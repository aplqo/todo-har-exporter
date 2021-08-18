extern crate regex;

use crate::{group::GroupData, har::APIResult};
use std::{
    fs::{create_dir_all, OpenOptions},
    io::{Result, Write},
    path::{Path, PathBuf},
};

fn write_file(path: &Path, value: &str) -> Result<()> {
    {
        let parent = path.parent().unwrap();
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)?
        .write_all(value.as_bytes())
        .map(|_| ())
}

struct MyPathBuf {
    val: PathBuf,
}
impl MyPathBuf {
    fn push(&mut self, val: &str) -> &mut Self {
        self.val.push(val);
        self
    }
    fn set_json(&mut self) -> &mut Self {
        match self.val.extension() {
            Some(v) => {
                let mut ext = v.to_owned();
                ext.push(".json");
                self.val.set_extension(ext)
            }
            None => self.val.set_extension("json"),
        };
        self
    }
    fn from(p: &str) -> Self {
        Self {
            val: PathBuf::from(p),
        }
    }
}

pub fn export_tasks(group_info: &GroupData, task_list: &APIResult, id: &str) -> Result<()> {
    write_file(
        &MyPathBuf::from("./tasklists/by-id").push(id).set_json().val,
        task_list.result.as_str(),
    )?;

    let mut buf = MyPathBuf::from("./tasklists/by-name");
    let task = group_info.get_tasklist(id);
    if let Some(g) = task.group {
        buf.push(g);
    }
    write_file(
        &buf.push(task.name).set_json().val,
        task_list.result.as_str(),
    )
}
pub fn export_json(name: &str, data: &str) -> Result<()> {
    write_file(&MyPathBuf::from(".").push(name).set_json().val, data)
}
