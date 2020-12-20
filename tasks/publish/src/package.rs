use guppy::graph::{PackageLink, PackageMetadata};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

// Exists because PackageMetadata doesn't implement Hash
pub struct PackageWrapper<'g> {
    pkg_metadata: PackageMetadata<'g>,
    link: PackageLink<'g>,
}

impl<'g> Debug for PackageWrapper<'g> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pkg_metadata.name())
    }
}

impl<'g> PackageWrapper<'g> {
    pub(crate) fn new(package: PackageMetadata<'g>, link: PackageLink<'g>) -> Self {
        Self {
            pkg_metadata: package,
            link,
        }
    }

    pub(crate) fn metadata(&self) -> PackageMetadata<'g> {
        self.pkg_metadata
    }

    pub(crate) fn link(&self) -> PackageLink<'g> {
        self.link
    }
}

impl<'g> Hash for PackageWrapper<'g> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pkg_metadata.name().hash(state);
    }
}

impl<'g> PartialEq for PackageWrapper<'g> {
    fn eq(&self, other: &Self) -> bool {
        self.pkg_metadata.name().eq(other.pkg_metadata.name())
    }
}

impl<'g> Eq for PackageWrapper<'g> {}
