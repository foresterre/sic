//! The goal of the functionality within this module is to find a common sub directory `D` for a set
//! of input files, split the input files at `D`, and concat the second element (directories and
//! files which aren't a common path for all input files) to an output directory.
//!
//! In summary, we aim to mirror an unrooted file structure to a new root.

use std::path::{Path, PathBuf};

/// A common dir (1st element), and a set of paths (2nd element) which when concatenated with the
/// common dir, result in a full path again.
#[derive(Debug)]
pub struct CommonDir(
    PathBuf, // The common directory
    Vec<(
        PathBuf, // The original input path
        PathBuf, // The `k` in `concat(common dir, k)`
    )>,
);

impl CommonDir {
    /// Expects canonicalized paths
    pub fn try_new<P: AsRef<Path>, I: IntoIterator<Item = P> + Clone>(
        paths: I,
    ) -> anyhow::Result<Self> {
        naive_find_common_dir(paths)
    }

    /// The found common root path
    pub fn common_root(&self) -> &Path {
        self.0.as_path()
    }

    /// The original input paths
    pub fn input_paths(&self) -> Vec<&Path> {
        self.1.iter().map(|(lp, _)| lp.as_path()).collect()
    }

    ///  The k's in `concat(common dir, k)`
    pub fn path_branches(&self) -> Vec<&Path> {
        self.1.iter().map(|(_, rp)| rp.as_path()).collect()
    }

    /// A tuple of the original input path, and its path branch
    pub fn path_combinations(&self) -> Vec<(&Path, &Path)> {
        self.1
            .iter()
            .map(|(lp, rp)| (lp.as_path(), rp.as_path()))
            .collect()
    }
}

fn naive_find_common_dir<P: AsRef<Path>, I: IntoIterator<Item = P> + Clone>(
    paths: I,
) -> anyhow::Result<CommonDir> {
    let mut iter = paths.clone().into_iter();
    let first_path = iter
        .next()
        .ok_or_else(|| anyhow::anyhow!("No paths found (glob mode)"))?;
    let path_trunk = first_path
        .as_ref()
        .parent()
        .ok_or_else(|| anyhow::anyhow!("No root directory found (glob mode)"))?;
    let ancestors = path_trunk.ancestors();

    let set_of_paths = paths
        .into_iter()
        .map(|p| p.as_ref().to_path_buf())
        .collect::<Vec<PathBuf>>();

    for ancestor in ancestors {
        // checks whether the current path has a common ancestors for all inputs
        if set_of_paths.iter().all(|path| path.starts_with(ancestor)) {
            let vec: Vec<(PathBuf, PathBuf)> = set_of_paths
                .iter()
                .map(|path| (path.to_path_buf(), unroot(ancestor, path)))
                .collect();

            return Ok(CommonDir(ancestor.to_path_buf(), vec));
        }
    }

    let set = set_of_paths
        .iter()
        .map(|p| {
            p.file_name()
                .map(|str| (p.to_path_buf(), PathBuf::from(str.to_os_string())))
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Unable to mirror input directory to output: found an invalid file path"
                    )
                })
        })
        .collect::<anyhow::Result<Vec<(PathBuf, PathBuf)>>>()?;

    Ok(CommonDir(path_trunk.to_path_buf(), set))
}

// The intention of this function is to remove a common root path from a given path.
pub(in crate::cli::common_dir) fn unroot(root: &Path, path: &Path) -> PathBuf {
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

        let stem = common.path_branches();

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

        let stem = common.path_branches();

        assert_eq!(stem[0], Path::new("a.png"));
        assert_eq!(stem[1], Path::new("b.png"));
        assert_eq!(stem[2], Path::new("c.png"));
    }

    #[test]
    fn no_path_before_file() {
        let common = CommonDir::try_new(vec!["a.png", "b.png", "c.png"]).unwrap();

        assert_eq!(common.common_root(), Path::new(""));

        let stem = common.path_branches();

        assert_eq!(stem[0], Path::new("a.png"));
        assert_eq!(stem[1], Path::new("b.png"));
        assert_eq!(stem[2], Path::new("c.png"));
    }

    #[test]
    fn empty_common_dir() {
        let common = CommonDir::try_new(Vec::<PathBuf>::new());
        assert!(common.is_err());
    }
}
