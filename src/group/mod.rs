extern crate regex;
extern crate serde;
extern crate serde_json;

use crate::har::APIResult;
use serde::Deserialize;
use std::{collections::HashMap, vec::Vec};

const TASKFOLDER_URL_PREFIX: &str =
    "https://substrate.office.com/todo/api/v1/taskfolders?$select=*";
const FOLDERGROUP_URL: &str = "https://substrate.office.com/todo/api/v1/foldergroups";

#[derive(Deserialize)]
struct Folder {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ParentFolderGroupId")]
    parent_folder_group_id: Option<String>,
}
#[derive(Deserialize)]
struct Folders {
    #[serde(rename = "Value")]
    value: Vec<Folder>,
}

#[derive(Deserialize)]
struct FolderGroup {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Id")]
    id: String,
}
#[derive(Deserialize)]
struct FolderGroups {
    #[serde(rename = "Value")]
    value: Vec<FolderGroup>,
}

struct FolderInfo {
    name: String,
    group: Option<String>,
}
pub struct GroupData {
    folders: HashMap<String, FolderInfo>,
    groups: HashMap<String, String>,
}
pub struct TaskList<'a> {
    pub name: &'a String,
    pub group: Option<&'a String>,
}

impl GroupData {
    fn from_str(taskfolders: &str, foldergroups: &str) -> GroupData {
        GroupData {
            folders: serde_json::from_str::<Folders>(taskfolders)
                .unwrap()
                .value
                .into_iter()
                .map(|i| {
                    (
                        i.id,
                        FolderInfo {
                            name: i.name,
                            group: i.parent_folder_group_id,
                        },
                    )
                })
                .collect(),
            groups: serde_json::from_str::<FolderGroups>(foldergroups)
                .unwrap()
                .value
                .into_iter()
                .map(|i| (i.id, i.name))
                .collect(),
        }
    }
    pub fn from_har(har: &Vec<APIResult>) -> GroupData {
        let mut taskfolders = None;
        let mut foldergroup = None;
        for i in har {
            if i.url == FOLDERGROUP_URL {
                foldergroup = Some(&i.url);
            } else if i.url.starts_with(TASKFOLDER_URL_PREFIX) {
                taskfolders = Some(&i.url);
            }
        }
        Self::from_str(taskfolders.unwrap().as_str(), foldergroup.unwrap().as_str())
    }
    pub fn get_tasklist(&self, name: &str) -> TaskList<'_> {
        let list = self.folders.get(name).unwrap();
        TaskList {
            name: &list.name,
            group: list.group.as_ref().and_then(|gid| self.groups.get(gid)),
        }
    }
}
