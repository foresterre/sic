use sic_image_engine::engine::{EnvironmentItem, EnvironmentKind, Statement};
use sic_image_engine::wrapper::filter_type::FilterTypeWrap;
use sic_image_engine::Operation;
use sic_parser::value_parser::{Describable, ParseInputsFromIter};
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
    ModResizePreserveAspectRatio,
    ModResizeSamplingFilter,
}

macro_rules! parse_inputs_by_type {
    ($iterable:expr, $ty:ty) => {{
        let input: Result<$ty, String> = ParseInputsFromIter::parse($iterable);
        input
    }};
}

impl OperationId {
    pub fn mk_statement<'a, T>(&self, inputs: T) -> Result<Statement, String>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
    {
        let stmt = match self {
            OperationId::Blur => {
                Statement::Operation(Operation::Blur(parse_inputs_by_type!(inputs, f32)?))
            }
            OperationId::Brighten => {
                Statement::Operation(Operation::Brighten(parse_inputs_by_type!(inputs, i32)?))
            }
            OperationId::Contrast => {
                Statement::Operation(Operation::Contrast(parse_inputs_by_type!(inputs, f32)?))
            }
            OperationId::Crop => Statement::Operation(Operation::Crop(parse_inputs_by_type!(
                inputs,
                (u32, u32, u32, u32)
            )?)),
            OperationId::Filter3x3 => Statement::Operation(Operation::Filter3x3(
                parse_inputs_by_type!(inputs, [f32; 9])?,
            )),
            OperationId::FlipH => Statement::Operation(Operation::FlipHorizontal),
            OperationId::FlipV => Statement::Operation(Operation::FlipVertical),
            OperationId::Grayscale => Statement::Operation(Operation::GrayScale),
            OperationId::HueRotate => {
                Statement::Operation(Operation::HueRotate(parse_inputs_by_type!(inputs, i32)?))
            }
            OperationId::Invert => Statement::Operation(Operation::Invert),
            OperationId::Resize => Statement::Operation(Operation::Resize(parse_inputs_by_type!(
                inputs,
                (u32, u32)
            )?)),
            OperationId::Rotate90 => Statement::Operation(Operation::Rotate90),
            OperationId::Rotate180 => Statement::Operation(Operation::Rotate180),
            OperationId::Rotate270 => Statement::Operation(Operation::Rotate270),
            OperationId::Unsharpen => Statement::Operation(Operation::Unsharpen(
                parse_inputs_by_type!(inputs, (f32, i32))?,
            )),

            OperationId::ModResizePreserveAspectRatio => {
                let toggle = parse_inputs_by_type!(inputs, bool)?;
                if toggle {
                    Statement::RegisterEnvironmentItem(EnvironmentItem::PreserveAspectRatio)
                } else {
                    Statement::DeregisterEnvironmentItem(
                        EnvironmentKind::OptResizePreserveAspectRatio,
                    )
                }
            }
            OperationId::ModResizeSamplingFilter => {
                let input = parse_inputs_by_type!(inputs, String)?;
                let filter = FilterTypeWrap::try_from_str(&input)
                    .map_err(|_| "Error: resize sampling filter not found.".to_string())?;
                Statement::RegisterEnvironmentItem(EnvironmentItem::OptResizeSamplingFilter(filter))
            }
        };

        Ok(stmt)
    }
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

// Pair operations with the index, which can be used to find the order in which arguments were provided.
//
// usage:
//
// ```Index
// op_by_index!(matches, "clap arg name", OperationId::Blur)?;
// ```
#[macro_export]
macro_rules! op_with_values {
    ($matches:expr, $op_name:expr, $op_variant:expr) => {{
        let indices = $matches.indices_of($op_name);
        let values = $matches.values_of($op_name);
        let vec: Option<IndexedOps> = indices.and_then(|indices| {
            values.map(|values| {
                indices
                    .zip(values)
                    .map(|(i, v)| (i, Op::WithValues(i, $op_variant, vec![v.to_string()])))
                    .collect::<_>()
            })
        });

        vec
    }};
}

#[macro_export]
macro_rules! op_valueless {
    ($matches:expr, $op_name:expr, $op_variant:expr) => {{
        $matches.indices_of($op_name).map(|indices| {
            indices
                .map(|index| (index, Op::Bare(index, $op_variant)))
                .collect::<Vec<_>>()
        })
    }};
}

/// Extends the IndexTree with the found cli image operations.
/// Should be used one image operation at a time.
/// The amount of values ensures that cli arguments which take more than one value will be combined.
pub(crate) fn tree_extend(
    tree: &mut IndexTree,
    values_for_operation: Option<IndexedOps>,
    amount_of_values: usize,
) -> Result<(), String> {
    match (amount_of_values, values_for_operation) {
        (_, None) => Ok(()),
        (0, Some(values)) | (1, Some(values)) => {
            tree.extend(values);
            Ok(())
        }
        (n, Some(values)) => tree_extend_unifiable(tree, values, n),
    }
}

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
fn tree_extend_unifiable(
    tree: &mut IndexTree,
    nodes: IndexedOps,
    size: usize,
) -> Result<(), String> {
    let unified = unify_multiple_values(nodes, size)?;
    tree.extend(unified);
    Ok(())
}

/// Chunk provided values and try to unify each chunk to a single [Op].
/// Requires each chunk to be of the size of the `size` argument.
fn unify_multiple_values(nodes: IndexedOps, size: usize) -> Result<IndexedOps, String> {
    assert_ne!(size, 0);

    let chunks = nodes.chunks(size).clone();
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
mod test_unification {
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
            let res = tree_extend_unifiable(&mut tree, blur, 1);

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
            let res = tree_extend_unifiable(&mut tree, blur, 4);

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
            let res = tree_extend_unifiable(&mut tree, blur, 2);

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
            let res = tree_extend_unifiable(&mut tree, blur, 4);

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
            let res = tree_extend_unifiable(&mut tree, blur, 4);

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
            let res = tree_extend_unifiable(&mut tree, blur, 4);

            assert!(res.is_err())
        }
    }
}

#[cfg(test)]
mod test_tree_extend {
    use super::*;
    use crate::app::cli::arg_names::*;
    use crate::app::cli::cli;
    use clap::ArgMatches;
    use sic_testing::{setup_output_path, setup_test_image};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn setup(cmd: &str) -> (ArgMatches, String) {
        let out = output(cmd);

        let command = format!("sic -i {} -o {} {}", input().as_str(), out, cmd);
        dbg!(command.clone());

        let split = command.split_ascii_whitespace().collect::<Vec<_>>();
        (cli().get_matches_from_safe(&split).unwrap(), out)
    }

    fn input() -> String {
        setup_test_image("rainbow_8x6.bmp")
            .to_string_lossy()
            .to_string()
    }

    fn output<T: Hash>(id: T) -> String {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let likely_unique = hasher.finish();
        setup_output_path(&format!("{}.bmp", likely_unique))
            .to_string_lossy()
            .to_string()
    }

    // 1) Individual uses of: op_with_values! and op_valueless!
    // FIXME: Quite a bit duplication currently.

    mod case_blur {
        use super::*;

        #[test]
        fn blur_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--blur 1.5");
            let blur: Option<IndexedOps> = op_with_values!(setup.0, OP_BLUR, OperationId::Blur);
            tree_extend(&mut tree, blur, 1).unwrap();

            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Blur);
                    assert_eq!(*values, vec!["1.5".to_string()]);
                }
                _ => panic!("test err"),
            }
        }

        #[test]
        fn blur_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--blur -1.5");
            let blur: Option<IndexedOps> = op_with_values!(setup.0, OP_BLUR, OperationId::Blur);
            tree_extend(&mut tree, blur, 1).unwrap();

            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Blur);
                    assert_eq!(*values, vec!["-1.5".to_string()]);
                }
                _ => panic!("test err"),
            }
        }
    }

    mod case_brighten {
        use super::*;

        #[test]
        fn brighten_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--brighten 1");
            let matches = setup.0;
            let brighten = op_with_values!(matches, OP_BRIGHTEN, OperationId::Brighten);
            let has = brighten.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Brighten);
                    assert_eq!(*values, vec!["1".to_string()]);
                }
                _ => panic!("test err"),
            }
        }

        #[test]
        fn brighten_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--brighten -1");
            let matches = setup.0;
            let brighten = op_with_values!(matches, OP_BRIGHTEN, OperationId::Brighten);
            let has = brighten.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Brighten);
                    assert_eq!(*values, vec!["-1".to_string()]);
                }
                _ => panic!("test err"),
            }
        }
    }

    mod case_contrast {
        #[test]
        fn contrast_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--contrast 1.5");
            let matches = setup.0;
            let contrast = op_with_values!(matches, OP_CONTRAST, OperationId::Contrast);
            let has = contrast.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Contrast);
                    assert_eq!(*values, vec!["1.5".to_string()]);
                }
                _ => panic!("test err"),
            }
        }

        use super::*;

        #[test]
        fn contrast_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--contrast -1.5");
            let matches = setup.0;
            let contrast = op_with_values!(matches, OP_CONTRAST, OperationId::Contrast);
            let has = contrast.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Contrast);
                    assert_eq!(*values, vec!["-1.5".to_string()]);
                }
                _ => panic!("test err"),
            }
        }
    }

    mod case_crop {
        use super::*;

        #[test]
        fn crop_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--crop 1 2 3 4");
            let matches = setup.0;
            let crop = op_with_values!(matches, OP_CROP, OperationId::Crop);
            tree_extend(&mut tree, crop, 4).unwrap();

            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Crop);
                    assert_eq!(
                        *values,
                        vec!["1", "2", "3", "4"]
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                    );
                }
                _ => panic!("test err"),
            }
        }

        #[test]
        #[should_panic]
        fn crop_x1_neg() {
            setup("--crop -1 -2 -3 -4");
        }
    }

    mod case_filter3x3 {
        use super::*;

        #[test]
        fn filter3x3_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--filter3x3 1 2 3 4 5.5 6 7 8 9");
            let matches = setup.0;
            let filter3x3 = op_with_values!(matches, OP_FILTER3X3, OperationId::Filter3x3);
            tree_extend(&mut tree, filter3x3, 9).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Filter3x3);
            assert_eq!(
                *values,
                vec!["1", "2", "3", "4", "5.5", "6", "7", "8", "9"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn filter3x3_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--filter3x3 1 2 3 4 -5.5 6 7 8 9");
            let matches = setup.0;
            let filter3x3 = op_with_values!(matches, OP_FILTER3X3, OperationId::Filter3x3);
            tree_extend(&mut tree, filter3x3, 9).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Filter3x3);
            assert_eq!(
                *values,
                vec!["1", "2", "3", "4", "-5.5", "6", "7", "8", "9"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }
    }

    mod case_flip_horizontal {
        use super::*;

        #[test]
        fn flip_horizontal_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--flip-horizontal");
            let matches = setup.0;
            let fliph = op_valueless!(matches, OP_FLIP_HORIZONTAL, OperationId::FlipH);
            let has = fliph.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };

            assert_eq!(id, OperationId::FlipH);
        }
    }

    mod case_flip_vertical {
        use super::*;

        #[test]
        fn flip_vertical_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--flip-vertical");
            let matches = setup.0;
            let flipv = op_valueless!(matches, OP_FLIP_VERTICAL, OperationId::FlipV);
            let has = flipv.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };

            assert_eq!(id, OperationId::FlipV);
        }
    }

    mod case_grayscale {
        use super::*;

        #[test]
        fn grayscale_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--grayscale");
            let matches = setup.0;
            let grayscale = op_valueless!(matches, OP_GRAYSCALE, OperationId::Grayscale);
            let has = grayscale.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };

            assert_eq!(id, OperationId::Grayscale);
        }
    }

    mod case_hue_rotate {
        use super::*;

        #[test]
        fn hue_rotate_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--hue-rotate 1");
            let matches = setup.0;
            let hue_rotate = op_with_values!(matches, OP_HUE_ROTATE, OperationId::HueRotate);
            tree_extend(&mut tree, hue_rotate, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::HueRotate);
            assert_eq!(
                *values,
                vec!["1"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn hue_rotate_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--hue-rotate -1");
            let matches = setup.0;
            let hue_rotate = op_with_values!(matches, OP_HUE_ROTATE, OperationId::HueRotate);
            tree_extend(&mut tree, hue_rotate, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::HueRotate);
            assert_eq!(
                *values,
                vec!["-1"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }
    }

    mod case_invert {
        use super::*;

        #[test]
        fn invert_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let op_id = OperationId::Invert;
            let setup = setup("--invert");
            let matches = setup.0;
            let op = op_valueless!(matches, OP_INVERT, op_id);
            tree_extend(&mut tree, op, 0).unwrap();

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };

            assert_eq!(id, op_id);
        }
    }

    mod case_resize {
        use super::*;

        #[test]
        fn resize_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--resize 1 2");
            let matches = setup.0;
            let resize = op_with_values!(matches, OP_RESIZE, OperationId::Resize);
            tree_extend(&mut tree, resize, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Resize);
            assert_eq!(
                *values,
                vec!["1", "2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        #[should_panic]
        fn resize_x1_neg() {
            setup("--resize -1 2");
        }

        #[test]
        #[should_panic]
        fn resize_x1_neg2() {
            setup("--resize 1 -2");
        }

        #[test]
        #[should_panic]
        fn resize_x1_neg3() {
            setup("--resize -1 -2");
        }
    }

    mod case_rotate90 {
        use super::*;

        #[test]
        fn rotate90_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let op_id = OperationId::Rotate90;
            let setup = setup("--rotate90");
            let matches = setup.0;

            let rotate90 = op_valueless!(matches, OP_ROTATE90, OperationId::Rotate90);
            let has = rotate90.map(|nodes| tree.extend(nodes));
            assert!(has.is_some());

            let out = tree.iter().next().unwrap();
            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };
            assert_eq!(id, op_id);
        }
    }

    mod case_rotate180 {
        use super::*;

        #[test]
        fn rotate180_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let op_id = OperationId::Rotate180;
            let setup = setup("--rotate180");
            let matches = setup.0;

            let rotate180 = op_valueless!(matches, OP_ROTATE180, OperationId::Rotate180);
            let has = rotate180.map(|nodes| tree.extend(nodes));
            assert!(has.is_some());

            let out = tree.iter().next().unwrap();
            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };
            assert_eq!(id, op_id);
        }
    }

    mod case_rotate270 {
        use super::*;

        #[test]
        fn rotate270_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let op_id = OperationId::Rotate270;
            let setup = setup("--rotate270");
            let matches = setup.0;

            let rotate270 = op_valueless!(matches, OP_ROTATE270, OperationId::Rotate270);
            let has = rotate270.map(|nodes| tree.extend(nodes));
            assert!(has.is_some());

            let out = tree.iter().next().unwrap();
            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };
            assert_eq!(id, op_id);
        }
    }

    mod case_unsharpen {
        use super::*;

        #[test]
        fn unsharpen_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--unsharpen 1 2");
            let matches = setup.0;
            let unsharpen = op_with_values!(matches, OP_UNSHARPEN, OperationId::Unsharpen);
            tree_extend(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Unsharpen);
            assert_eq!(
                *values,
                vec!["1", "2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn unsharpen_x1_neg1() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--unsharpen -1.5 2");
            let matches = setup.0;
            let unsharpen = op_with_values!(matches, OP_UNSHARPEN, OperationId::Unsharpen);
            tree_extend(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Unsharpen);
            assert_eq!(
                *values,
                vec!["-1.5", "2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn unsharpen_x1_neg2() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--unsharpen 1.5 -2");
            let matches = setup.0;
            let unsharpen = op_with_values!(matches, OP_UNSHARPEN, OperationId::Unsharpen);
            tree_extend(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Unsharpen);
            assert_eq!(
                *values,
                vec!["1.5", "-2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn unsharpen_x1_neg3() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--unsharpen -1.5 -2");
            let matches = setup.0;
            let unsharpen = op_with_values!(matches, OP_UNSHARPEN, OperationId::Unsharpen);
            tree_extend(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Unsharpen);
            assert_eq!(
                *values,
                vec!["-1.5", "-2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }
    }

    mod case_opmod_resize_keep_aspect_ratio {
        use super::*;

        #[test]
        fn set_true() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--set-resize-preserve-aspect-ratio true");
            let matches = setup.0;
            let op = op_with_values!(
                matches,
                OPMOD_RESIZE_PRESERVE_ASPECT_RATIO,
                OperationId::ModResizePreserveAspectRatio
            );
            tree_extend(&mut tree, op, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::ModResizePreserveAspectRatio);
            assert_eq!(
                *values,
                vec!["true"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn set_false() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--set-resize-preserve-aspect-ratio false");
            let matches = setup.0;
            let op = op_with_values!(
                matches,
                OPMOD_RESIZE_PRESERVE_ASPECT_RATIO,
                OperationId::ModResizePreserveAspectRatio
            );
            tree_extend(&mut tree, op, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::ModResizePreserveAspectRatio);
            assert_eq!(
                *values,
                vec!["false"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        #[should_panic]
        fn not_allowed_value() {
            setup("--set-resize-preserve-aspect-ratio yes");
        }
    }

    mod case_opmod_resize_sampling_filter {
        use super::*;

        fn test<'a>(setup: (ArgMatches, String), expect: &str) {
            let mut tree: IndexTree = BTreeMap::new();
            let matches = setup.0;
            let op = op_with_values!(
                matches,
                OPMOD_RESIZE_SAMPLING_FILTER,
                OperationId::ModResizeSamplingFilter
            );
            tree_extend(&mut tree, op, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (a, b) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*a, OperationId::ModResizeSamplingFilter);
            assert_eq!(
                *b,
                vec![expect]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        // catmullrom, gaussian, lanczos3, nearest, triangle

        #[test]
        fn set_catmullrom() {
            let setup = setup("--set-resize-sampling-filter catmullrom");
            test(setup, "catmullrom");
        }

        #[test]
        fn set_gaussian() {
            let setup = setup("--set-resize-sampling-filter gaussian");
            test(setup, "gaussian");
        }

        #[test]
        fn set_default() {
            let setup = setup("--set-resize-sampling-filter");
            test(setup, "gaussian");
        }

        #[test]
        fn set_lanczos3() {
            let setup = setup("--set-resize-sampling-filter lanczos3");
            test(setup, "lanczos3");
        }

        #[test]
        fn set_nearest() {
            let setup = setup("--set-resize-sampling-filter nearest");
            test(setup, "nearest");
        }

        #[test]
        fn set_triangle() {
            let setup = setup("--set-resize-sampling-filter triangle");
            test(setup, "triangle");
        }

        #[test]
        #[should_panic]
        fn not_allowed_value() {
            setup("--set-resize-sampling-filter yes");
        }

    }
}
