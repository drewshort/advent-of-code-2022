use core::marker::{ Send, Sync };
use std::{ collections::BTreeMap, rc::Rc, sync::{ atomic::AtomicUsize, Arc } };
use r3bl_rs_utils::{
    tree_memory_arena::{ Arena, HasId, MTArena, ResultUidList },
    utils::{ style_primary, style_prompt },
};

enum NodePayload {
    FilesystemNode(FilesystemNode),
    FilesystemLeaf(FilesystemLeaf),
}

#[derive(Debug, Clone)]
struct Node {
    parent: usize,
    tree: Arc<&mut Tree>,
    id: usize,
    payload: NodePayload,
    children: Vec<usize>,
}

impl Node {
    fn new(tree: Arc<&mut Tree>, id: usize, parent: usize, payload: NodePayload) -> Self {
        let node = Self { parent, tree, id, payload, children: Vec::new() };
    }

    pub fn add_directory(&mut self, path: &str) {
        let node_id = self.tree.get_next_id();
        let directory = FilesystemNode::new(self.id, path);
        let directory_ref = Rc::new(directory);
        let directory_node = Self::new(
            Arc::clone(&self.tree),
            node_id,
            self.id,
            NodePayload::FilesystemNode(directory)
        );
        self.insert(String::from(path), Rc::clone(&directory_ref));
        self.tree.add_node(directory_node);
    }

    pub fn add_file(&mut self, file_name: String, file_size: usize) {
        let node_id = self.tree.get_next_id();
        let file = FilesystemLeaf::new(self.id, file_name, file_size);
        let file_ref = Rc::new(file);
        let file_node = Self::new(Arc::clone(&self.tree), node_id, self.id, NodePayload::FilesystemLeaf(file));
        self.files.insert(String::from(&file_leaf.name), file_leaf);
    }
}

impl HasId for Node {
    type Id = usize;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl HasId for usize {
    type Id = usize;

    fn id(&self) -> &Self::Id {
        self
    }
}

struct Tree {
    arena: Arena<Node>,
    root_node: Option<Node>,
    next_id: AtomicUsize,
}

impl Tree {
    fn new() -> Self {
        let mut tree = Self { arena: Arena::<usize>::new(), root_node: None, next_id: 1 };
        let node = Node::new(Arc::new(&mut tree), 0, 0, NodePayload::FilesystemNode(FilesystemNode::root()));
        tree.arena.add_new_node(node, None);
        tree.root_node = Some(node);
        tree
    }

    fn get_next_id(&mut self) -> usize {
        self.next_id.fetch_add(1, order)
    }

    fn add_node(&mut self, node: Node) -> usize {
        self.arena.add_new_node(node, Some(node.parent))
    }
}

pub struct FilesystemLeaf {
    parent: usize,
    name: String,
    size: usize,
}

impl FilesystemLeaf {
    pub fn new(parent: usize, name: String, size: usize) -> Self {
        Self { parent, name, size }
    }
}

pub struct FilesystemNode {
    parent: usize,
    path: String,
    directories: BTreeMap<String, usize>,
    files: BTreeMap<String, usize>,
}

impl FilesystemNode {
    pub fn root_node(parent: usize) -> Self {
        Self {
            parent,
            path: String::from(""),
            directories: BTreeMap::new(),
            files: BTreeMap::new(),
        }
    }

    pub fn new(parent: usize, path: &str) -> Self {
        Self {
            parent,
            path: String::from(path),
            directories: BTreeMap::new(),
            files: BTreeMap::new(),
        }
    }
}