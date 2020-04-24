use std::collections::BTreeMap;

use crate::errors::SicCliOpsError;
use sic_image_engine::engine::{EnvItem, Instr, ItemName};
use sic_image_engine::wrapper::filter_type::FilterTypeWrap;
use sic_image_engine::wrapper::image_path::ImageFromPath;
use sic_image_engine::ImgOp;
use sic_parser::errors::SicParserError;
use sic_parser::value_parser::{Describable, ParseInputsFromIter};

/// The enumeration of all supported operations.
#[derive(Debug, Copy, Clone, Eq, PartialEq, AsStaticStr, EnumIter)]
pub enum OperationId {
    Blur,
    Brighten,
    Contrast,
    Crop,
    Diff,
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

impl OperationId {
    /// A string representation for each operation.
    pub fn as_str(&self) -> &str {
        use strum::AsStaticRef;
        self.as_static()
    }

    /// Provides the number of arguments an operation takes.
    /// Used to unify arguments together.
    /// E.g. (without accounting for the requirement of having incremental indices as well),
    ///     say we receive for resize the values 10, 20, 100 and 100. With the number of values we know
    ///     that each resize operation takes two arguments, not four. So it could be that there are
    ///     two operations, namely `resize 10 20` and `resize 100 100`. We do need to take some other
    ///     conditions into account, but they are not relevant for this particular method =).
    pub fn takes_number_of_arguments(self) -> usize {
        match self {
            OperationId::Blur => 1,
            OperationId::Brighten => 1,
            OperationId::Contrast => 1,
            OperationId::Crop => 4,
            OperationId::Diff => 1,
            OperationId::Filter3x3 => 9,
            OperationId::FlipH => 0,
            OperationId::FlipV => 0,
            OperationId::Grayscale => 0,
            OperationId::HueRotate => 1,
            OperationId::Invert => 0,
            OperationId::Resize => 2,
            OperationId::Rotate90 => 0,
            OperationId::Rotate180 => 0,
            OperationId::Rotate270 => 0,
            OperationId::Unsharpen => 2,
            OperationId::ModResizePreserveAspectRatio => 1,
            OperationId::ModResizeSamplingFilter => 1,
        }
    }
}

macro_rules! parse_inputs_by_type {
    ($iterable:expr, $ty:ty) => {{
        let input: Result<$ty, SicCliOpsError> =
            ParseInputsFromIter::parse($iterable).map_err(|err| {
                SicCliOpsError::UnableToParseValueOfType {
                    err: err,
                    typ: stringify!($ty).to_string(),
                }
            });
        input
    }};
}

impl OperationId {
    /// Constructs statements for image operations which are taken as input by the image engine.
    pub fn mk_statement<'a, T>(self, inputs: T) -> Result<Instr, SicCliOpsError>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
    {
        let stmt = match self {
            OperationId::Blur => Instr::Operation(ImgOp::Blur(parse_inputs_by_type!(inputs, f32)?)),
            OperationId::Brighten => {
                Instr::Operation(ImgOp::Brighten(parse_inputs_by_type!(inputs, i32)?))
            }
            OperationId::Contrast => {
                Instr::Operation(ImgOp::Contrast(parse_inputs_by_type!(inputs, f32)?))
            }
            OperationId::Crop => Instr::Operation(ImgOp::Crop(parse_inputs_by_type!(
                inputs,
                (u32, u32, u32, u32)
            )?)),
            OperationId::Diff => {
                Instr::Operation(ImgOp::Diff(parse_inputs_by_type!(inputs, ImageFromPath)?))
            }
            OperationId::Filter3x3 => {
                Instr::Operation(ImgOp::Filter3x3(parse_inputs_by_type!(inputs, [f32; 9])?))
            }
            OperationId::FlipH => Instr::Operation(ImgOp::FlipHorizontal),
            OperationId::FlipV => Instr::Operation(ImgOp::FlipVertical),
            OperationId::Grayscale => Instr::Operation(ImgOp::GrayScale),
            OperationId::HueRotate => {
                Instr::Operation(ImgOp::HueRotate(parse_inputs_by_type!(inputs, i32)?))
            }
            OperationId::Invert => Instr::Operation(ImgOp::Invert),
            OperationId::Resize => {
                Instr::Operation(ImgOp::Resize(parse_inputs_by_type!(inputs, (u32, u32))?))
            }
            OperationId::Rotate90 => Instr::Operation(ImgOp::Rotate90),
            OperationId::Rotate180 => Instr::Operation(ImgOp::Rotate180),
            OperationId::Rotate270 => Instr::Operation(ImgOp::Rotate270),
            OperationId::Unsharpen => {
                Instr::Operation(ImgOp::Unsharpen(parse_inputs_by_type!(inputs, (f32, i32))?))
            }

            OperationId::ModResizePreserveAspectRatio => {
                let toggle = parse_inputs_by_type!(inputs, bool)?;
                if toggle {
                    Instr::EnvAdd(EnvItem::PreserveAspectRatio)
                } else {
                    Instr::EnvRemove(ItemName::PreserveAspectRatio)
                }
            }
            OperationId::ModResizeSamplingFilter => {
                let input = parse_inputs_by_type!(inputs, String)?;
                let filter = FilterTypeWrap::try_from_str(&input)
                    .map_err(SicParserError::FilterTypeError)?;
                Instr::EnvAdd(EnvItem::CustomSamplingFilter(filter))
            }
        };

        Ok(stmt)
    }
}

pub type Index = usize;

/// Represents an image operation which was obtained from CLI image operation commands.
///
/// OperationId := Type of operation we are dealing with, e.g. Blur or Rotate90.
/// Vec<String> := Vector of unverified string arguments; initially with multiple arguments
///              we will receive multiple [Op] as Clap provides multiple
///              arguments individually. The multiple [Op] will be unified where applicable.
///
/// The operation argument values are not parsed yet within this structure.
/// The values are also not necessarily unified yet.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Op {
    WithValues(OperationId, Vec<String>),
    Bare(OperationId),
}

/// An IndexTree represents the decided order in which operations should be applied.
/// Because the underlying data structure is a BTree, we can conveniently add
/// [Op] by their provided indices.
/// Note that unified [Op] could be given any index of the values they were originally unified
/// from.
pub type IndexTree = BTreeMap<Index, Op>;

/// Nodes which contain tuples with arity 2, where the first value is the Index,
/// and the second value is an Operation
pub type IndexedOps = Vec<(Index, Op)>;

// Pair operations with the index, which can be used to find the order in which arguments were provided.
// It should only be used for operations which take one or more arguments.
//
// usage:
//
// ```Index
// op_by_index!(matches, "clap arg name", OperationId::Blur)?;
// ```
#[macro_export]
macro_rules! op_with_values {
    ($matches:expr, $op_variant:expr) => {{
        let op_name = $op_variant.as_str();
        let indices = $matches.indices_of(op_name);
        let values = $matches.values_of(op_name);
        let vec: Option<IndexedOps> = indices.and_then(|indices| {
            values.map(|values| {
                indices
                    .zip(values)
                    .map(|(i, v)| (i, Op::WithValues($op_variant, vec![v.to_string()])))
                    .collect::<_>()
            })
        });

        vec
    }};
}

// This macro helps us to create Op::Bare values. Since always the enum variant Bare is used,
// this should only be used for operations which do not take arguments.
#[macro_export]
macro_rules! op_valueless {
    ($matches:expr, $op_variant:expr) => {{
        let op_name = $op_variant.as_str();
        $matches.indices_of(op_name).map(|indices| {
            indices
                .map(|index| (index, Op::Bare($op_variant)))
                .collect::<Vec<_>>()
        })
    }};
}

/// Extends the IndexTree with the found cli image operations.
/// Should be used one image operation at a time.
/// The amount of values ensures that cli arguments which take more than one value will be combined.
///     The total amount of arguments will be partitioned in equal sized partitions (if possible,
///     otherwise the partitioning is invalid and will be rejected). The size of each partition
///     is the 'amount of values'. The values of each partition will be unified to represent
///     a the arguments of an operation.
///     Examples regarding the amount of values this function expects:
///     Operation 'blur' takes one argument so the amount of values for the blur operation is 1.
///     Operation 'crop' takes four arguments, so the amount of values for the crop operation is 4.
pub(crate) fn extend_index_tree_with_unification(
    tree: &mut IndexTree,
    values_for_operation: Option<IndexedOps>,
    amount_of_values: usize,
) -> Result<(), SicCliOpsError> {
    match (amount_of_values, values_for_operation) {
        // The operation is not available within our input.
        (_, None) => Ok(()),
        // The operation is available, and requires 0 or 1 arguments.
        // We do not need to unify these values, since operations which 'take' 0 arguments
        // will be of enum type Op::Bare and operations which take 1 argument, are
        // already in their correct state (after all, we receive one argument of an operation at
        // a time).
        (0, Some(values)) | (1, Some(values)) => {
            tree.extend(values);
            Ok(())
        }
        // The operation is available, and requires 2 or more arguments.
        // Since we receive the available values for an operation one at a time (but they will
        // have an index); you can look at them as a stand alone list of values, we need to combine
        // the values to the amount of arguments which each operation takes.
        // So, if we have an operation X which takes two arguments, and we receive the values
        // [1, 2, 3, 4] for that operation, it could be that the operation was 'called' twice by the
        // user (namely, once with arguments 1 and 2, and once with 3 and 4. We check whether that is
        // actually the cases by checking whether the indices of these arguments are incremental.
        // That is, if the argument value 1 has index 6, argument value 2 should have index 6+1=7.
        // And then if argument value 3 has index 21, argument value 4 should have index 21+1=22.
        // The input by a user could look like `sic -i in -o out --my-operation-X 1 2 (...) --my-operation-X 3 4 (...)`.
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
) -> Result<(), SicCliOpsError> {
    let unified = unify_arguments_of_operation(nodes, size)?;
    tree.extend(unified);
    Ok(())
}

/// Chunk provided values and try to unify each chunk to a single [Op].
/// Requires each chunk to be of the size of the `size` argument.
fn unify_arguments_of_operation(
    nodes: IndexedOps,
    size: usize,
) -> Result<IndexedOps, SicCliOpsError> {
    assert_ne!(size, 0);

    let chunks = nodes.chunks(size).clone();
    let mut vec: IndexedOps = Vec::new();

    for chunk in chunks {
        if chunk.len() != size {
            return Err(SicCliOpsError::UnableToCorrectlyPartitionMultiParamArguments(chunk.len()));
        }

        let unified_chunk = unify_chunk(chunk, None, size);
        vec.push(unified_chunk?);
    }

    Ok(vec)
}

/// Try to unify a chunk of values to a single value.
fn unify_chunk(
    left: &[(usize, Op)],
    last: Option<(usize, Op)>,
    size: usize,
) -> Result<(usize, Op), SicCliOpsError> {
    // stop: complete unification of the chunk
    if left.is_empty() {
        match last {
            Some(ret) => Ok(ret),
            None => Err(SicCliOpsError::UnableToUnifyMultiValuedArguments),
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
                        (Op::WithValues(id, mut values), Op::WithValues(_id2, values2)) => {
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
                            let updated_op = Op::WithValues(id, values);

                            // Package it as an [IndexedOpNode]
                            let new_last = (current.0, updated_op);

                            unify_chunk(left[1..].as_ref(), Some(new_last), size)
                        }
                        _ => Err(SicCliOpsError::UnableToUnifyBareValues),
                    }
                } else {
                    Err(SicCliOpsError::UnableToUnifyMultiValuedArguments)
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
        use std::collections::BTreeMap;

        use super::*;

        #[test]
        fn tree_extend_unifiable_n1() {
            let mut tree: IndexTree = BTreeMap::new();
            assert!(tree.is_empty());

            let blur: IndexedOps =
                vec![(0, Op::WithValues(OperationId::Blur, vec!["1".to_string()]))];
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
                (0, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (1, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (2, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (3, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
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
                (0, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (1, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (2, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (3, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
            ];
            let res = tree_extend_unifiable(&mut tree, blur, 2);

            assert!(res.is_ok());
            assert_eq!(tree.len(), 2);
        }

        #[test]
        fn tree_extend_unifiable_n4_fail() {
            let mut tree: IndexTree = BTreeMap::new();

            let blur: IndexedOps = vec![
                (0, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (2, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (2, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (3, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
            ];
            let res = tree_extend_unifiable(&mut tree, blur, 4);

            assert!(res.is_err())
        }

        #[test]
        fn tree_extend_unifiable_n4_too_few_provided() {
            let mut tree: IndexTree = BTreeMap::new();

            let blur: IndexedOps = vec![
                (0, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (1, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (2, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
            ];
            let res = tree_extend_unifiable(&mut tree, blur, 4);

            assert!(res.is_err())
        }

        #[test]
        fn tree_extend_unifiable_n4_too_many_provided() {
            let mut tree: IndexTree = BTreeMap::new();

            let blur: IndexedOps = vec![
                (0, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (1, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (2, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (3, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
                (4, Op::WithValues(OperationId::Blur, vec!["1".to_string()])),
            ];
            let res = tree_extend_unifiable(&mut tree, blur, 4);

            assert!(res.is_err())
        }
    }
}

#[cfg(test)]
mod test_tree_extend {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use clap::ArgMatches;

    use sic_cli::cli::cli;
    use sic_testing::{setup_output_path, setup_test_image};

    use super::*;

    fn setup(cmd: &str) -> (ArgMatches, String) {
        let out = output(cmd);
        let command = format!("sic -i {} -o {} {}", input().as_str(), out, cmd);
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
            let blur: Option<IndexedOps> = op_with_values!(setup.0, OperationId::Blur);
            extend_index_tree_with_unification(&mut tree, blur, 1).unwrap();

            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(id, values) => {
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
            let blur: Option<IndexedOps> = op_with_values!(setup.0, OperationId::Blur);
            extend_index_tree_with_unification(&mut tree, blur, 1).unwrap();

            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(id, values) => {
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
            let brighten = op_with_values!(matches, OperationId::Brighten);
            let has = brighten.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(id, values) => {
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
            let brighten = op_with_values!(matches, OperationId::Brighten);
            let has = brighten.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(id, values) => {
                    assert_eq!(*id, OperationId::Brighten);
                    assert_eq!(*values, vec!["-1".to_string()]);
                }
                _ => panic!("test err"),
            }
        }
    }

    mod case_contrast {
        use super::*;

        #[test]
        fn contrast_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--contrast 1.5");
            let matches = setup.0;
            let contrast = op_with_values!(matches, OperationId::Contrast);
            let has = contrast.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(id, values) => {
                    assert_eq!(*id, OperationId::Contrast);
                    assert_eq!(*values, vec!["1.5".to_string()]);
                }
                _ => panic!("test err"),
            }
        }

        #[test]
        fn contrast_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--contrast -1.5");
            let matches = setup.0;
            let contrast = op_with_values!(matches, OperationId::Contrast);
            let has = contrast.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(id, values) => {
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
            let crop = op_with_values!(matches, OperationId::Crop);
            extend_index_tree_with_unification(&mut tree, crop, 4).unwrap();

            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(id, values) => {
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
            let filter3x3 = op_with_values!(matches, OperationId::Filter3x3);
            extend_index_tree_with_unification(&mut tree, filter3x3, 9).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
            let filter3x3 = op_with_values!(matches, OperationId::Filter3x3);
            extend_index_tree_with_unification(&mut tree, filter3x3, 9).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
            let fliph = op_valueless!(matches, OperationId::FlipH);
            let has = fliph.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(id)) => *id,
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
            let flipv = op_valueless!(matches, OperationId::FlipV);
            let has = flipv.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(id)) => *id,
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
            let grayscale = op_valueless!(matches, OperationId::Grayscale);
            let has = grayscale.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(id)) => *id,
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
            let hue_rotate = op_with_values!(matches, OperationId::HueRotate);
            extend_index_tree_with_unification(&mut tree, hue_rotate, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
            let hue_rotate = op_with_values!(matches, OperationId::HueRotate);
            extend_index_tree_with_unification(&mut tree, hue_rotate, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
            let op = op_valueless!(matches, op_id);
            extend_index_tree_with_unification(&mut tree, op, 0).unwrap();

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(id)) => *id,
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
            let resize = op_with_values!(matches, OperationId::Resize);
            extend_index_tree_with_unification(&mut tree, resize, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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

            let rotate90 = op_valueless!(matches, OperationId::Rotate90);
            let has = rotate90.map(|nodes| tree.extend(nodes));
            assert!(has.is_some());

            let out = tree.iter().next().unwrap();
            let id = match out {
                (_, Op::Bare(id)) => *id,
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

            let rotate180 = op_valueless!(matches, OperationId::Rotate180);
            let has = rotate180.map(|nodes| tree.extend(nodes));
            assert!(has.is_some());

            let out = tree.iter().next().unwrap();
            let id = match out {
                (_, Op::Bare(id)) => *id,
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

            let rotate270 = op_valueless!(matches, OperationId::Rotate270);
            let has = rotate270.map(|nodes| tree.extend(nodes));
            assert!(has.is_some());

            let out = tree.iter().next().unwrap();
            let id = match out {
                (_, Op::Bare(id)) => *id,
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
            let unsharpen = op_with_values!(matches, OperationId::Unsharpen);
            extend_index_tree_with_unification(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
            let unsharpen = op_with_values!(matches, OperationId::Unsharpen);
            extend_index_tree_with_unification(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
            let unsharpen = op_with_values!(matches, OperationId::Unsharpen);
            extend_index_tree_with_unification(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
            let unsharpen = op_with_values!(matches, OperationId::Unsharpen);
            extend_index_tree_with_unification(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
            let op = op_with_values!(matches, OperationId::ModResizePreserveAspectRatio);
            extend_index_tree_with_unification(&mut tree, op, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
            let op = op_with_values!(matches, OperationId::ModResizePreserveAspectRatio);
            extend_index_tree_with_unification(&mut tree, op, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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

        fn test(setup: (ArgMatches, String), expect: &str) {
            let mut tree: IndexTree = BTreeMap::new();
            let matches = setup.0;
            let op = op_with_values!(matches, OperationId::ModResizeSamplingFilter);
            extend_index_tree_with_unification(&mut tree, op, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (a, b) = match out {
                (_, Op::WithValues(id, values)) => (id, values),
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
