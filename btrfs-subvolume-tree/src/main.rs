use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    ffi::CStr,
    io,
};

use proc_mounts::MountIter;
use uuid::Uuid;

use btrfsutil::*;
use btrfsutil_sys::*;

#[derive(Clone)]
struct Node {
    info: btrfs_util_subvolume_info,
    path: Option<String>,
    mountpoint: Option<String>,
    children: Vec<Node>,
}

impl ptree::TreeItem for Node {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, _style: &ptree::Style) -> io::Result<()> {
        let id = self.info.id;
        let uuid = Uuid::from_slice(&self.info.uuid).unwrap();
        let path = self
            .path
            .as_deref()
            .unwrap_or_else(|| if id == 5 { "ROOT" } else { "<unknown>" });
        if let Some(mountpoint) = &self.mountpoint {
            write!(f, "{: <25}\t\t[{}]\t\t{}", path, uuid, mountpoint)
        } else {
            write!(f, "{: <25}\t\t[{}]", path, uuid)
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::Borrowed(self.children.as_slice())
    }
}

fn main() {
    let search_path = "/";

    let mounts = MountIter::new()
        .expect("unable to create mount iterator")
        .map(|mount| mount.expect("unable to get next mount"))
        .filter(|mount| mount.fstype == "btrfs")
        .map(|mut mount| {
            mount.options.retain(|option| option.starts_with("subvol"));
            (mount.source, mount.dest, mount.options)
        })
        .collect::<Vec<_>>();

    let source_for_path = mounts
        .iter()
        .find(|(_, path, _)| search_path.starts_with(path.to_string_lossy().as_ref()))
        .map(|(source, _, _)| source);

    let mounts = mounts
        .iter()
        .filter(|(source, _, _)| Some(source) == source_for_path)
        .map(|(_, path, options)| {
            let subvolid = options
                .iter()
                .find(|option| option.starts_with("subvolid="))
                .map(|option| option.trim_start_matches("subvolid="))
                .map(|id| id.parse::<u64>().expect("subvolid is not a number"));
            (subvolid, path)
        })
        .filter_map(|(subvol, path)| subvol.map(|subvol| (subvol, path)))
        .collect::<HashMap<_, _>>();

    let subvolumes =
        SubvolumeIterator::new(search_path, 5, 0).expect("unable to create subvolume iterator");

    let mut queue = VecDeque::new();

    for item in subvolumes {
        let (info, path) = item.expect("unable to get next item");
        let mountpoint = mounts
            .get(&info.id)
            .map(|path| path.to_string_lossy().into_owned());
        let node = Node {
            info,
            path: path
                .as_deref()
                .map(CStr::to_string_lossy)
                .map(Cow::into_owned),
            mountpoint,
            children: Vec::new(),
        };
        queue.push_back(node);
    }

    let mut root_children = Vec::new();
    while let Some(mut node) = queue.pop_back() {
        if node.info.parent_id == 5 {
            node.path = node
                .path
                .map(|path| path.trim_start_matches('/').to_owned());
            root_children.push(node);
            continue;
        }

        if let Some(parent) = queue.iter_mut().find(|n| n.info.id == node.info.parent_id) {
            if let Some(parent_path) = parent.path.as_ref() {
                node.path = node.path.map(|path| {
                    path.trim_start_matches(parent_path)
                        .trim_start_matches('/')
                        .to_owned()
                });
            }

            parent.children.push(node);
            parent.children.sort_by(|a, b| a.info.id.cmp(&b.info.id));
            continue;
        }

        if let Some(parent) = root_children
            .iter_mut()
            .find(|n| n.info.id == node.info.parent_id)
        {
            if let Some(parent_path) = parent.path.as_ref() {
                node.path = node.path.map(|path| {
                    path.trim_start_matches(parent_path)
                        .trim_start_matches('/')
                        .to_owned()
                });
            }

            parent.children.push(node);
            parent.children.sort_by(|a, b| a.info.id.cmp(&b.info.id));
            continue;
        }

        panic!("unable to find parent for subvolume {}", node.info.id);
    }
    root_children.sort_by(|a, b| a.info.id.cmp(&b.info.id));

    let root_info = subvolume_info("/", 5)
        .expect("unable to get subvolume info for root, is this a btrfs filesystem?");
    let root_node = Node {
        info: root_info,
        path: None,
        mountpoint: mounts
            .get(&5)
            .map(|path| path.to_string_lossy().into_owned()),
        children: root_children,
    };

    ptree::print_tree(&root_node).unwrap();
}
