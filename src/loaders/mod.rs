use crate::tja;

pub fn get_all_tja_paths(root: &str) -> Vec<std::path::PathBuf> {
    let conf = crate::conf::Conf::default();
    let mut paths = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(std::path::PathBuf::from(root));
    while let Some(directory) = queue.pop_front() {
        if let Ok(entires) = directory.read_dir() {
            for entry in entires {
                if let Ok(entry) = entry {
                    if let Ok(filetype) = entry.file_type() {
                        let path = entry.path();
                        if filetype.is_dir() {
                            queue.push_back(path);
                        } else if filetype.is_file() {
                            if let Some(extension) = path.extension() {
                                if extension == "tja" {
                                    println!(
                                        "{}",
                                        tja::Chart::parse_from_path(&path, None, &conf, None)
                                            .unwrap()
                                    );
                                    paths.push(path);
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
