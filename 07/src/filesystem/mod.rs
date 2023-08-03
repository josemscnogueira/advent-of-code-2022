use std::collections::HashMap;

#[derive(Debug)]
pub enum Inode {
    File(usize),
    Directory(HashMap<String, Self>),
}

impl Inode {
    pub fn new_dir() -> Self {
        Self::Directory(HashMap::new())
    }

    pub fn new_file(size: usize) -> Self {
        Self::File(size)
    }

    pub fn size(&self) -> usize {
        match self {
            Self::File(s) => *s,
            Self::Directory(m) => m.iter().map(|(_, n)| n.size()).sum(),
        }
    }

    pub fn get_directories(&self) -> Vec<&Self> {
        match self {
            Self::File(_) => Vec::new(),
            Self::Directory(m) => {
                let mut result = vec![self];

                for (_, d) in m.iter() {
                    result = [&result[..], &d.get_directories()[..]].concat();
                }

                result
            }
        }
    }

    pub fn enter(&mut self, name: &str) -> Option<&mut Self> {
        match self {
            Self::File(_) => None,
            Self::Directory(m) => {
                if let Some(candidate) = m.get_mut(name) {
                    Some(candidate)
                } else {
                    None
                }
            }
        }
    }

    pub fn navigate(&mut self, path: &[&str]) -> Option<&mut Self> {
        let mut result = Some(self);

        for name in path {
            if let Some(r) = result {
                result = r.enter(&name);
            } else {
                break;
            }
        }

        result
    }

    pub fn add(
        &mut self,
        path: &[&str],
        name: String,
        node: Self,
    ) -> Result<(), String> {
        if let Some(Self::Directory(m)) = self.navigate(path) {
            let result = m.insert(name, node);
            debug_assert!(result.is_none());
            Ok(())
        } else {
            Err("Path not found, or it's not a directory".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let root = Inode::new_dir();

        println!("{:?}", root);
    }

    #[test]
    fn add() {
        let mut root = Inode::new_dir();

        root.add(&["a"], Inode::new_dir()).unwrap();
        root.add(&["a", "b.txt"], Inode::new_file(14848514))
            .unwrap();
        root.add(&["a", "c.dat"], Inode::new_file(8504156)).unwrap();
        root.add(&["a", "d"], Inode::new_dir()).unwrap();

        println!("{:?}", root);
    }
}
