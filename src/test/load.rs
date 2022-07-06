pub fn get_all_tja_paths(root: &str) -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(std::path::PathBuf::from(root));
    while let Some(directory) = queue.pop_front() {
        if let Ok(entires) = directory.read_dir() {
            for entry in entires {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        let path = entry.path();
                        if file_type.is_dir() {
                            queue.push_back(path);
                        } else if file_type.is_file() {
                            if let Some(extension) = path.extension() {
                                if extension == "tja" {
                                    if let Ok(_) = tja::Chart::load(&path, None) {
                                        paths.push(path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    paths
}
