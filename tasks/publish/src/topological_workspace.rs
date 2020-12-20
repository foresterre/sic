use guppy::graph::{DependencyDirection, PackageMetadata, PackageSet};

/// Get the packages in the workspace, sorted in reverse topological order
/// Reverser topological means ordered from packages which depend on the fewest workspace
/// packages to the most. This is useful, because when updating versions throughout the workspace
/// and uploading them to the registry, the registry needs to know the packages you depend on.
///
/// Say you have a package A and a package B and they are both version 1 (v1). Let's say A v1 depends
/// on B v1. We have been developing both in tandem, and depending on them in our workspace by
/// using the local path. Both API's have breaking changes.
///
/// Now you want to update both to v2. However there are a few constraints: the registry can
/// only depend on versions it knows already about and the registry doesn't accept packages outside
/// it's own registry (such as our locally referenced path).
///
/// Now we can't first update A to v2, since it depends on breaking changes in B v2, and B is not
/// published to the registry yet. We can however update B to v2, since it doesn't depend on A v2,
/// and then when B v2 is published, update A's dependency on B from v1 to v2, and then publish A
/// v2.
///
/// This function determines in what order we should update our workspaces packages, just like we
/// determined we needed to update B before we could do A in the example above.
pub fn get_topological_workspace<'g>(set: &'g PackageSet) -> Vec<PackageMetadata<'g>> {
    set.packages(DependencyDirection::Reverse)
        .filter(|pkg| pkg.in_workspace())
        .collect()
}
