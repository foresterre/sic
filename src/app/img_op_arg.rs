use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum OperationId {
    Blur,
    Brighten,
    Contrast,
    Crop,
    Filter3x3,
    FlipH,
    FlipV,
    Grayscale,
    HueRotate,
    Invert,
    Resize,
    Rotate90,
    Rotate180,
    Rotate270,
    Unsharpen,
}

type Index = usize;

/// Represents an image operation which was obtained from CLI image operation commands.
///
/// Index := Position of an argument value.
/// OperationId := Type of operation we are dealing with, e.g. Blur or Rotate90.
/// Vec<String> := Vector of unverified string arguments; initially with multiple arguments
///              we will receive multiple [Op] as Clap provides multiple arguments individually.
///              The multiple Op will be unified where applicable.
///
/// The operation values are not parsed yet within this structure.
#[derive(Debug, Clone)]
pub(crate) enum Op {
    WithValues(Index, OperationId, Vec<String>),
    Bare(Index, OperationId),
}

/// An IndexTree represents the decided order in which operations should be applied.
/// Because the underlying data structure is a BTree, we can conveniently add
/// [Op] by their provided indices.
/// Note that unified [Op] could be given any index of the values they were originally unified
/// from.#[macro_export]

pub(crate) type IndexTree = BTreeMap<Index, Op>;

/// Nodes which contain tuples with arity 2, where the first value is the Index,
/// and the second value is an Operation
pub(crate) type IndexedOps = Vec<(Index, Op)>;

/// This tree extension function should be used if an image operation cli arguments takes more than 1
/// value.
/// Clap provides all values for an argument as a linear container (like a vector with values).
/// However, we don't want to allow --crop 0 0 --crop 1 1, but we do want --crop 0 0 1 1.
/// The resulting value we get from Clap do not differ for the two examples above.
/// Both will give a container along the lines of vec!("0", "0", "1", "1").
/// To ensure we only allow, for an image operation cli arguments which takes 4 values, 4 values
/// all at once, we use the indices of the values (which Clap does provide) to check whether they
/// are incrementally correct (i.e. index_{i+1} = index_{i} + 1).
///
/// Arguments:
///     tree: The BTree which we will extend with nodes for an operation type.
///     op_values: If the operation is not provided, we don't extend the tree. If it is,
///                we'll use the separate nodes and try to unify them into nodes which contain
///                each `size` values.
///     size: The amount of values which an image operation requires.
///
pub(crate) fn tree_extend_unifiable(
    tree: &mut IndexTree,
    op_values: Option<IndexedOps>,
    size: usize,
) -> Result<(), String> {
    match op_values {
        Some(nodes) => {
            let unified = unify_multiple_values(nodes, size)?;
            tree.extend(unified);
            Ok(())
        }
        // No image operation cli argument of this type given
        None => Ok(()),
    }
}

/// Chunk provided values and try to unify each chunk to a single [Op].
/// Requires each chunk to be of the size of the `size` argument.
fn unify_multiple_values(container: IndexedOps, size: usize) -> Result<IndexedOps, String> {
    assert_ne!(size, 0);

    let chunks = container.chunks(size).clone();
    let mut vec: IndexedOps = Vec::new();

    for chunk in chunks {
        if chunk.len() != size {
            return Err(format!(
                "Unification of multi valued argument(s) failed: arguments could't be \
                 partitioned in correct chunk sizes. Length of chunk: {}",
                chunk.len()
            ));
        }

        let unified_chunk = unify_chunk(chunk, None, size);
        vec.push(unified_chunk?);
    }

    Ok(vec)
}

const FAILED_UNIFICATION_MESSAGE: &str =
    "Unification of multi valued argument(s) failed: \
     When using an image operation cli argument which requires n values, \
     all values should be provided at once. For example, `--crop` takes 4 values \
     so, n=4. Now, `--crop 0 0 1 1` would be valid, but `--crop 0 0 --crop 1 1` would not.";

/// Try to unify a chunk of values to a single value.
fn unify_chunk(
    left: &[(usize, Op)],
    last: Option<(usize, Op)>,
    size: usize,
) -> Result<(usize, Op), String> {
    // stop: complete unification of the chunk
    if left.is_empty() {
        match last {
            Some(ret) => Ok(ret),
            None => Err(FAILED_UNIFICATION_MESSAGE.to_string()),
        }
    } else {
        // continue (left.len() > 0):
        let current: (usize, Op) = left[0].clone();

        match last {
            Some(node) => {
                // is it incremental based on the last index <- (index, _) ?
                if (node.0 + 1) == current.0 {
                    // Here we create an [IndexedOpNode] tuple.
                    // with (index, op),
                    // `index` will be the index of the current node (i.e. de new last)
                    // `op` will be the node in unification, which is based on the last node,
                    // but here we add an un-parsed string value to the value vec.
                    let unified_op = node.1;
                    let current_op = current.1;
                    match (unified_op, current_op) {
                        (Op::WithValues(_, id, mut values), Op::WithValues(_, _, values2)) => {
                            values.extend(values2);

                            // the [Op] consist of:
                            // (
                            //   Index := update the index to the current (this could also be the first
                            //            index, or any node which was unified with the `last` node,
                            //            however we choose to update the index with the last, so the
                            //            index of the IndexedOpNode and Op will be consistent
                            //   OperationId := operation id, again from the first added node
                            //   Vec<String> := we add the value (new) of the `current` [Op] to the values
                            //                  already part of the unified [Op]; i.e. we extend
                            //                  the unified [Op] with a new provided image operation
                            //                  argument
                            // )
                            let updated_op = Op::WithValues(current.0, id, values);

                            // Package it as an [IndexedOpNode]
                            let new_last = (current.0, updated_op);

                            unify_chunk(left[1..].as_ref(), Some(new_last), size)
                        }
                        _ => Err("Can't unify bare values".to_string()),
                    }
                } else {
                    Err(FAILED_UNIFICATION_MESSAGE.to_string())
                }
            }
            // first value
            None => unify_chunk(left[1..].as_ref(), Some(current), size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cli_arg_values_unification {
        use super::*;
        use std::collections::BTreeMap;

        #[test]
        fn tree_extend_unifiable_n1() {
            let mut tree: IndexTree = BTreeMap::new();
            assert!(tree.is_empty());

            let blur: IndexedOps = vec![(
                0,
                Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
            )];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 1);

            assert!(res.is_ok());
            assert_eq!(tree.len(), 1);
        }

        #[test]
        fn tree_extend_unifiable_n4() {
            let mut tree: IndexTree = BTreeMap::new();

            assert!(tree.is_empty());

            // the `tree_extend_unifiable` function doesn't care about the operation id.
            // Nor about the Index within Op(index,..,..).
            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    1,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    3,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 4);

            assert!(res.is_ok());
            assert_eq!(tree.len(), 1);
        }

        #[test]
        fn tree_extend_unifiable_n2_multiple() {
            let mut tree: IndexTree = BTreeMap::new();

            assert!(tree.is_empty());

            // the `tree_extend_unifiable` function doesn't care about the operation id.
            // Nor about the Index within Op(index,..,..).
            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    1,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    3,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 2);

            assert!(res.is_ok());
            assert_eq!(tree.len(), 2);
        }

        #[test]
        fn tree_extend_unifiable_n4_fail() {
            let mut tree: IndexTree = BTreeMap::new();

            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    3,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 4);

            assert!(res.is_err())
        }

        #[test]
        fn tree_extend_unifiable_n4_too_few_provided() {
            let mut tree: IndexTree = BTreeMap::new();

            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    1,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 4);

            assert!(res.is_err())
        }

        #[test]
        fn tree_extend_unifiable_n4_too_many_provided() {
            let mut tree: IndexTree = BTreeMap::new();

            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    1,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    3,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    4,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 4);

            assert!(res.is_err())
        }
    }
}
