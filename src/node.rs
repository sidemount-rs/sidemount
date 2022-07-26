#[derive(Debug)]
pub struct Node<T> {
    pub nodes: Vec<Node<T>>,
    pub key: String,
    pub handler: Option<T>,
    pub wildcard: bool,
}

/// Default implementation for node with a "/" at the root path
impl<T> Default for Node<T> {
    fn default() -> Self {
        Node::new("/")
    }
}

impl<T> Node<T> {
    /// Creates a new node with the given path argument.
    pub fn new(key: &str) -> Self {
        Node {
            nodes: Vec::new(),
            key: String::from(key),
            handler: None,
            wildcard: key.starts_with("{") && key.ends_with("}"),
        }
    }

    /// Inserts a new path and associated handler along the node tree.
    pub fn insert(&mut self, path: &str, f: T) {
        match path.split_once('/') {
            Some((root, "")) => {
                self.key = String::from(root);
                self.handler = Some(f);
                self.wildcard = root.starts_with("{") && root.ends_with("}");
            }
            Some(("", path)) => self.insert(path, f),
            Some((root, path)) => {
                let node = self.nodes.iter_mut().find(|m| root == &m.key || m.wildcard);
                match node {
                    Some(n) => n.insert(path, f),
                    None => {
                        let mut node = Node::new(root);
                        node.insert(path, f);
                        self.nodes.push(node);
                    }
                }
            }
            None => {
                let mut node = Node::new(path);
                node.handler = Some(f);
                self.nodes.push(node);
            }
        }
    }

    /// Inserts a new path and associated node structure along the node tree.
    pub fn insert_node(&mut self, path: &str, node: Node<T>) {
        match path.split_once('/') {
            Some((root, "")) => {
                *self = node;
                self.key = String::from(root);
                self.wildcard = root.starts_with("{") && root.ends_with("}");
            }
            Some(("", path)) => self.insert_node(path, node),
            Some((root, path)) => {
                println!("split into {}, {}", root, path);
                let parent = self.nodes.iter_mut().find(|m| root == &m.key || m.wildcard);
                match parent {
                    Some(n) => n.insert_node(path, node),
                    None => {
                        let mut parent = Node::new(root);
                        parent.insert_node(path, node);
                        self.nodes.push(parent);
                    }
                }
            }
            None => {
                let mut parent = Node::new(path);
                parent.nodes = node.nodes;
                self.nodes.push(parent);
            }
        }
    }

    /// Gets a borrowed reference to the handler along the path
    pub fn get(&self, path: &str) -> Option<&T> {
        match path.split_once('/') {
            Some((root, "")) => {
                if root == &self.key || self.wildcard {
                    self.handler.as_ref()
                } else {
                    None
                }
            }
            Some(("", path)) => self.get(path),
            Some((root, path)) => {
                let node = self.nodes.iter().find(|m| root == &m.key || m.wildcard);
                if let Some(node) = node {
                    node.get(path)
                } else {
                    None
                }
            }
            None => {
                let node = self.nodes.iter().find(|m| path == &m.key || m.wildcard);
                if let Some(node) = node {
                    node.handler.as_ref()
                } else {
                    None
                }
            }
        }
    }

    /// Gets a mutable reference to the handler along the path
    pub fn get_mut(&mut self, path: &str) -> Option<&mut T> {
        match path.split_once('/') {
            Some((root, "")) => {
                if root == &self.key || self.wildcard {
                    self.handler.as_mut()
                } else {
                    None
                }
            }
            Some(("", path)) => self.get_mut(path),
            Some((root, path)) => {
                let node = self.nodes.iter_mut().find(|m| root == &m.key || m.wildcard);
                if let Some(node) = node {
                    node.get_mut(path)
                } else {
                    None
                }
            }
            None => {
                let node = self.nodes.iter_mut().find(|m| path == &m.key || m.wildcard);
                if let Some(node) = node {
                    node.handler.as_mut()
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub struct Request {}
    pub type HandlerFn = fn(Request) -> Result<(), std::io::Error>;

    #[test]
    fn test_insert_routes() {
        let mut root = Node::<HandlerFn>::new("");
        root.insert("/", |_| Ok(()));
        root.insert("/foo", |_| Ok(()));
        root.insert("/foo/bar", |_| Ok(()));
    }

    #[test]
    fn test_get_route() {
        let mut root = Node::<HandlerFn>::new("");
        root.insert("/", |_| Ok(()));
        root.insert("/foo/bar", |_| Ok(()));
        root.insert("/foo/foo", |_| Ok(()));
        root.insert("/users/{id}/profile", |_| Ok(()));
        root.insert("/companies/{id}/users/{userid}", |_| Ok(()));

        assert!(root.get("/").is_some());
        assert!(root.get("/foo/bar").is_some());
        assert!(root.get("/foo/foo").is_some());
        assert!(root.get("/fooar").is_none());
        assert!(root.get("/foo/bar/baz").is_none());
        assert!(root.get("/fbar/baz").is_none());
        assert!(root.get("/users/foo/profile").is_some());
        assert!(root.get("/users/bar/profile").is_some());
        assert!(root.get("/users/bar/asdf").is_none());
        assert!(root.get("/companies/1234/asdf").is_none());
        assert!(root.get("/companies/1234/users").is_none());
        assert!(root.get("/companies/1234/users/foo").is_some());
    }
}
