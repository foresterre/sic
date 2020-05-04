//! The goal of the functionality within this module is to find a common sub directory `D` for a set
//! of input files, split the input files at `D`, and concat the second element (directories and
//! files which aren't a common path for all input files) to an output directory.
//!
//! In summary, we aim to mirror an unrooted file structure to a new root.

use std::path::{Path, PathBuf};

/// A common dir (1st element), and a set of paths (2nd element) which when concatenated with the
/// common dir, result in a full path again.
#[derive(Debug)]
pub struct CommonDir(PathBuf, Vec<PathBuf>);

impl CommonDir {
    /// Expects canonicalized paths
    pub fn try_new<P: AsRef<Path>, I: IntoIterator<Item = P> + Clone>(
        paths: I,
    ) -> anyhow::Result<Self> {
        naive_find_common_dir(paths)
    }

    pub fn common_root(&self) -> &Path {
        self.0.as_path()
    }

    pub fn path_stems(&self) -> Vec<&Path> {
        self.1.iter().map(|p| p.as_path()).collect()
    }
}

fn naive_find_common_dir<P: AsRef<Path>, I: IntoIterator<Item = P> + Clone>(
    paths: I,
) -> anyhow::Result<CommonDir> {
    let mut iter = paths.clone().into_iter();
    let first_path = iter.next().unwrap();
    let path_trunk = first_path.as_ref().parent().unwrap();
    let ancestors = path_trunk.ancestors();

    let set_of_paths = paths
        .into_iter()
        .map(|p| p.as_ref().to_path_buf())
        .collect::<Vec<PathBuf>>();

    for ancestor in ancestors {
        // checks whether the current path has a common ancestors for all inputs
        if set_of_paths.iter().all(|path| path.starts_with(&ancestor)) {
            let vec: Vec<PathBuf> = set_of_paths
                .iter()
                .map(|path| unroot(&ancestor, path))
                .collect();

            return Ok(CommonDir(ancestor.to_path_buf(), vec));
        }
    }

    let set = set_of_paths
        .iter()
        .map(|p| {
            p.file_name()
                .map(|str| PathBuf::from(str.to_os_string()))
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Unable to mirror input directory to output: found an invalid file path"
                    )
                })
        })
        .collect::<anyhow::Result<Vec<PathBuf>>>()?;

    Ok(CommonDir(path_trunk.to_path_buf(), set))
}

// The intention of this function is to remove a common root path from a given path.
pub(in crate::common_dir) fn unroot(root: &Path, path: &Path) -> PathBuf {
    let root_len = root.components().count();

    path.components()
        .skip(root_len)
        .fold(PathBuf::new(), |mut parent, child| {
            parent.push(child.as_os_str());

            parent
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unroot_test_file_only() {
        let root = Path::new("/my/common");
        let full = Path::new("/my/common/a.png");

        let expected = PathBuf::from("a.png".to_string());
        assert_eq!(unroot(root, full), expected);
    }

    #[test]
    fn unroot_with_similar_dir_test() {
        let root = Path::new("/my");
        let full = Path::new("/my/common/a.png");

        let expected = PathBuf::from("common/a.png".to_string());
        assert_eq!(unroot(root, full), expected);
    }

    #[test]
    fn uncommon_dir_test() {
        let common = CommonDir::try_new(vec![
            "/my/common/path/a.png",
            "/my/common/path/b.png",
            "/my/uncommon/path/c.png",
        ])
        .unwrap();

        assert_eq!(common.common_root(), Path::new("/my"));

        let stem = common.path_stems();

        assert_eq!(stem[0], Path::new("common/path/a.png"));
        assert_eq!(stem[1], Path::new("common/path/b.png"));
        assert_eq!(stem[2], Path::new("uncommon/path/c.png"));
    }

    #[test]
    fn common_dir_test() {
        let common = CommonDir::try_new(vec![
            "/my/common/path/a.png",
            "/my/common/path/b.png",
            "/my/common/path/c.png",
        ])
        .unwrap();

        assert_eq!(common.common_root(), Path::new("/my/common/path/"));

        let stem = common.path_stems();

        assert_eq!(stem[0], Path::new("a.png"));
        assert_eq!(stem[1], Path::new("b.png"));
        assert_eq!(stem[2], Path::new("c.png"));
    }
}
