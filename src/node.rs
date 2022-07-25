pub struct Node<T> {
    pub nodes: Vec<Node<T>>,
    pub key: String,
    pub handler: Option<T>,
}

impl<T> Node<T> {
    /// Creates a new node with the given path argument.
    pub fn new(key: &str) -> Self {
        Node {
            nodes: Vec::new(),
            key: String::from(key),
            handler: None,
        }
    }

    /// Inserts a new path and associated handler along the node tree.
    pub fn insert(&mut self, path: &str, f: T) {
        match path.split_once('/') {
            Some((root, "")) => {
                self.key = String::from(root);
                self.handler = Some(f);
            }
            Some(("", path)) => self.insert(path, f),
            Some((root, path)) => {
                let node = self.nodes.iter_mut().find(|m| root == &m.key);
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

    pub fn get(&self, path: &str) -> Option<&T> {
        match path.split_once('/') {
            Some((root, "")) => {
                if root == &self.key {
                    self.handler.as_ref()
                } else {
                    None
                }
            }
            Some(("", path)) => self.get(path),
            Some((root, path)) => {
                let node = self.nodes.iter().find(|m| root == &m.key);
                if let Some(node) = node {
                    node.get(path)
                } else {
                    None
                }
            }
            None => {
                let node = self.nodes.iter().find(|m| path == &m.key);
                if let Some(node) = node {
                    node.handler.as_ref()
                } else {
                    None
                }
            }
        }
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_routes() {
        let mut root = Node::new("");
        root.insert("/", |_| Ok(()));
        root.insert("/foo", |_| Ok(()));
        root.insert("/foo/bar", |_| Ok(()));

        println!("{:?}", root);
    }

    #[test]
    fn test_get_route() {
        println!("{:?}", "foo".split_once('/'));
        println!("{:?}", "".split_once('/'));
        let mut root = Node::new("");
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

        println!("{:?}", root);
    }
}
*/
