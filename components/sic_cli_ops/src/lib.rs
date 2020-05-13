#[macro_use]
extern crate strum_macros;

use clap::ArgMatches;

use sic_image_engine::engine::Instr;

use crate::errors::SicCliOpsError;
use crate::operations::{
    extend_index_tree_with_unification, IndexTree, IndexedOps, Op, OperationId,
};

pub mod errors;
pub mod operations;

pub fn build_ast_from_matches(
    matches: &ArgMatches,
    tree: &mut IndexTree,
) -> Result<Vec<Instr>, SicCliOpsError> {
    use strum::IntoEnumIterator;

    let operations = OperationId::iter();
    ast_extend_with_operation(tree, matches, operations)?;

    // Build!
    ast_from_index_tree(tree)
}

fn ast_extend_with_operation<T: IntoIterator<Item = OperationId>>(
    tree: &mut IndexTree,
    matches: &ArgMatches,
    operations: T,
) -> Result<(), SicCliOpsError> {
    for operation in operations {
        let argc = operation.takes_number_of_arguments();
        let ops = mk_ops(operation, matches);
        extend_index_tree_with_unification(tree, ops, argc)?;
    }

    Ok(())
}

fn mk_ops(op: OperationId, matches: &ArgMatches) -> Option<IndexedOps> {
    let argc = op.takes_number_of_arguments();
    match argc {
        0 => op_valueless!(matches, op),
        _n => op_with_values!(matches, op),
    }
}

fn ast_from_index_tree(tree: &mut IndexTree) -> Result<Vec<Instr>, SicCliOpsError> {
    tree.iter()
        .map(|(_index, op)| match op {
            Op::Bare(id) => {
                let empty: &[&str; 0] = &[];
                id.mk_statement(empty)
            }
            Op::WithValues(OperationId::Diff, values) => {
                // HACK: From Pest we receive back a string including quotation marks, but
                //       the terminal doesn't; we'll have to figure out how we can instruct Pest
                //       to just give back the string contents without quotation marks
                let mut values = values.to_vec();

                #[allow(clippy::needless_range_loop)]
                for i in 0..values.len() {
                    values[i] = format!("\"{}\"", values[i]);
                }

                (OperationId::Diff).mk_statement(&values)
            }
            Op::WithValues(id, values) => id.mk_statement(values),
        })
        .collect::<Result<Vec<Instr>, SicCliOpsError>>()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use sic::cli::app::create_app as cli;
    use sic_image_engine::engine::Instr;
    use sic_image_engine::ImgOp;

    use super::*;

    macro_rules! assert_match {
        ($iter:expr, $clause:pat, $assert:expr) => {{
            match $iter.next().unwrap() {
                $clause => $assert,
                err => panic!(format!(
                    "Assertion: {} failed. Value found was: {:?}",
                    stringify!($assert),
                    err
                )),
            }
        }};
    }

    #[test]
    fn build_from_args_all() {
        let input = "sic -i in -o out \
                     --blur 1 \
                     --brighten 2 \
                     --contrast 3 \
                     --crop 0 0 2 2 \
                     --filter3x3 0 1 2 3 4 5 6 7 8 \
                     --flip-horizontal \
                     --flip-vertical \
                     --grayscale \
                     --hue-rotate -90 \
                     --invert \
                     --resize 10 10 \
                     --rotate90 \
                     --rotate180 \
                     --rotate270 \
                     --unsharpen 1.5 1";

        let input = input.split_ascii_whitespace();
        let matches = cli("", "", "").get_matches_from(input);
        let mut tree: IndexTree = BTreeMap::new();
        let ast = build_ast_from_matches(&matches, &mut tree);
        let ast = ast.unwrap();
        let mut iter = ast.iter();

        assert_match!(
            iter,
            Instr::Operation(ImgOp::Blur(n)),
            sic_testing::approx_eq_f32!(*n, 1f32)
        );

        assert_match!(
            iter,
            Instr::Operation(ImgOp::Brighten(n)),
            assert_eq!(*n, 2i32)
        );

        assert_match!(
            iter,
            Instr::Operation(ImgOp::Contrast(n)),
            sic_testing::approx_eq_f32!(*n, 3f32)
        );

        assert_match!(
            iter,
            Instr::Operation(ImgOp::Crop(n)),
            assert_eq!(*n, (0u32, 0u32, 2u32, 2u32))
        );

        assert_match!(
            iter,
            Instr::Operation(ImgOp::Filter3x3(n)),
            assert_eq!(*n, [0f32, 1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32])
        );

        assert_match!(iter, Instr::Operation(ImgOp::FlipHorizontal), ());

        assert_match!(iter, Instr::Operation(ImgOp::FlipVertical), ());

        assert_match!(iter, Instr::Operation(ImgOp::GrayScale), ());

        assert_match!(
            iter,
            Instr::Operation(ImgOp::HueRotate(n)),
            assert_eq!(*n, -90i32)
        );

        assert_match!(iter, Instr::Operation(ImgOp::Invert), ());

        assert_match!(
            iter,
            Instr::Operation(ImgOp::Resize(n)),
            assert_eq!(*n, (10u32, 10u32))
        );

        assert_match!(iter, Instr::Operation(ImgOp::Rotate90), ());

        assert_match!(iter, Instr::Operation(ImgOp::Rotate180), ());

        assert_match!(iter, Instr::Operation(ImgOp::Rotate270), ());

        assert_match!(
            iter,
            Instr::Operation(ImgOp::Unsharpen(n)),
            assert_eq!(*n, (1.5f32, 1i32))
        );

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mk_ops_0() {
        let input = "sic -i in -o out \
                     --rotate180";

        let input = input.split_ascii_whitespace();
        let matches = cli("", "", "").get_matches_from(input);

        let ops = mk_ops(OperationId::Rotate180, &matches);
        let ops = ops.unwrap();
        let (i, v) = ops.get(0).unwrap();
        assert_eq!(*i, 5usize);
        assert_eq!(*v, Op::Bare(OperationId::Rotate180))
    }

    #[test]
    fn mk_ops_n() {
        let input = "sic -i in -o out --unsharpen 1.5 2";

        let input = input.split_ascii_whitespace();
        let matches = cli("", "", "").get_matches_from(input);

        // note that at mk_ops no unification of arguments has taken place.
        let ops = mk_ops(OperationId::Unsharpen, &matches);

        let ops = ops.unwrap();
        let (i, v) = ops.get(0).unwrap();
        assert_eq!(*i, 6usize);
        assert_eq!(
            *v,
            Op::WithValues(OperationId::Unsharpen, vec![String::from("1.5")])
        );

        let (i, v) = ops.get(1).unwrap();
        assert_eq!(*i, 7usize);
        assert_eq!(
            *v,
            Op::WithValues(OperationId::Unsharpen, vec![String::from("2")])
        );
    }

    #[test]
    fn ast_from_index_tree_empty() {
        let mut tree: IndexTree = BTreeMap::new();
        let ast = ast_from_index_tree(&mut tree);

        assert!(ast.unwrap().is_empty())
    }

    #[test]
    fn ast_from_index_tree_with_vals() {
        let mut tree: IndexTree = BTreeMap::new();
        tree.insert(
            1,
            Op::WithValues(OperationId::Brighten, vec![String::from("10")]),
        );
        tree.insert(2, Op::Bare(OperationId::FlipV));
        tree.insert(
            3,
            Op::WithValues(OperationId::HueRotate, vec![String::from("-90")]),
        );
        let ast = ast_from_index_tree(&mut tree);

        let iter = ast.unwrap();
        let mut iter = iter.iter();

        // can't assert eq, because Operation does not implement Eq, since f32 doesn't support it
        assert_match!(
            iter,
            Instr::Operation(ImgOp::Brighten(n)),
            assert_eq!(*n, 10)
        );

        assert_match!(iter, Instr::Operation(ImgOp::FlipVertical), ());

        assert_match!(
            iter,
            Instr::Operation(ImgOp::HueRotate(n)),
            assert_eq!(*n, -90)
        );

        assert_eq!(iter.next(), None);
    }
}
