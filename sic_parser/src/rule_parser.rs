/// The rule parser module has a goal to parse pairs/span from Pest data structures to image operations.
use pest::iterators::{Pair, Pairs};
use sic_image_engine::engine::{EnvironmentItem, EnvironmentKind, Instruction};
use sic_image_engine::wrapper::filter_type::FilterTypeWrap;
use sic_image_engine::ImgOp;

use super::Rule;
use crate::value_parser::ParseInputsFromIter;

// This function parses statements provided as a single 'script' to an image operations program.
// An image operations program is currently a linear list of image operations which are applied
// in a left-to-right order.
// Operations are parsed from Pairs and Rules, which are provided by the Pest parser library.
//
// In the event of any parse failure, an error shall be returned.
// The error currently usually contains into_inner(), to provide detailed information about the
// origins of the parsing rejection.
//
// FIXME: When the user facing errors will be reworked, the providing of or the how to providing of-
//        the into_inner() parsing details should be reconsidered
pub fn parse_image_operations(pairs: Pairs<'_, Rule>) -> Result<Vec<Instruction>, String> {
    pairs
        .filter(|pair| pair.as_rule() != Rule::EOI)
        .map(|pair| match pair.as_rule() {
            Rule::blur => parse_f32(pair).map(|v| Instruction::Operation(ImgOp::Blur(v))),
            Rule::brighten => {
                parse_i32(pair).map(|v| Instruction::Operation(ImgOp::Brighten(v)))
            }
            Rule::contrast => {
                parse_f32(pair).map(|v| Instruction::Operation(ImgOp::Contrast(v)))
            }
            Rule::crop => parse_crop(pair),
            Rule::filter3x3 => parse_filter3x3(pair),
            Rule::flip_horizontal => Ok(Instruction::Operation(ImgOp::FlipHorizontal)),
            Rule::flip_vertical => Ok(Instruction::Operation(ImgOp::FlipVertical)),
            Rule::grayscale => Ok(Instruction::Operation(ImgOp::GrayScale)),
            Rule::huerotate => {
                parse_i32(pair).map(|v| Instruction::Operation(ImgOp::HueRotate(v)))
            }
            Rule::invert => Ok(Instruction::Operation(ImgOp::Invert)),
            Rule::resize => parse_resize(pair),
            Rule::rotate90 => Ok(Instruction::Operation(ImgOp::Rotate90)),
            Rule::rotate180 => Ok(Instruction::Operation(ImgOp::Rotate180)),
            Rule::rotate270 => Ok(Instruction::Operation(ImgOp::Rotate270)),
            Rule::unsharpen => parse_unsharpen(pair),
            Rule::setopt => parse_set_environment(pair.into_inner().next().ok_or_else(|| {
                "Unable to parse `set` environment command. Error: expected a single `set` inner element.".to_string()
            })?),
            // this is called 'del' for users
            Rule::unsetopt => parse_unset_environment(pair.into_inner().next().ok_or_else(|| {
                "Unable to parse `del` environment command. Error: expected a single `del` inner element.".to_string()
            })?),
            _ => Err("Parse failed: Operation doesn't exist".to_string()),
        })
        .collect::<Result<Vec<_>, String>>()
}

macro_rules! parse_from_pair {
    ($pair:expr, $ty:ty) => {{
        let inner = $pair.into_inner();
        let ty: Result<$ty, String> = ParseInputsFromIter::parse(inner.map(|pair| pair.as_str()));
        ty
    }};
}

fn parse_set_environment(pair: Pair<'_, Rule>) -> Result<Instruction, String> {
    let environment_item = match pair.as_rule() {
        Rule::set_resize_sampling_filter => parse_set_resize_sampling_filter(pair)?,
        Rule::set_resize_preserve_aspect_ratio => EnvironmentItem::PreserveAspectRatio,
        _ => {
            return Err(format!(
                "Unable to parse `set` environment command. Error on element: {}",
                pair
            ));
        }
    };

    Ok(Instruction::AddToEnv(environment_item))
}

fn parse_set_resize_sampling_filter(pair: Pair<'_, Rule>) -> Result<EnvironmentItem, String> {
    let mut inner = pair.into_inner();

    // skip over the compound atomic 'env_available' rule
    inner
        .next()
        .ok_or_else(|| "Unable to parse the 'set_resize_sampling_filter' option. No options exist for the command. ")?;

    inner
        .next()
        .ok_or_else(|| {
            format!(
                "Unable to parse the 'set_resize_sampling_filter' option. Error on element: {}",
                inner
            )
        })
        .map(|val| val.as_str())
        .and_then(|val| {
            FilterTypeWrap::try_from_str(val).map_err(|err| format!("Unable to parse: {}", err))
        })
        .map(EnvironmentItem::CustomSamplingFilter)
}

fn parse_unset_environment(pair: Pair<'_, Rule>) -> Result<Instruction, String> {
    let environment_item = match pair.as_rule() {
        Rule::env_resize_sampling_filter_name => EnvironmentKind::CustomSamplingFilter,
        Rule::env_resize_preserve_aspect_ratio_name => EnvironmentKind::PreserveAspectRatio,
        _ => {
            return Err(format!(
                "Unable to parse `del` environment command. Error on element: {}",
                pair
            ));
        }
    };

    Ok(Instruction::RemoveFromEnv(environment_item))
}

fn parse_f32(pair: Pair<'_, Rule>) -> Result<f32, String> {
    parse_from_pair!(pair, f32)
}

fn parse_i32(pair: Pair<'_, Rule>) -> Result<i32, String> {
    parse_from_pair!(pair, i32)
}

fn parse_crop(pair: Pair<'_, Rule>) -> Result<Instruction, String> {
    let tuple = parse_from_pair!(pair, (u32, u32, u32, u32))?;
    let stmt = Instruction::Operation(ImgOp::Crop(tuple));
    Ok(stmt)
}

fn parse_filter3x3(pair: Pair<'_, Rule>) -> Result<Instruction, String> {
    let inner = pair.into_inner();
    let arr = ParseInputsFromIter::parse(inner.map(|pair| pair.as_str()))?;
    let stmt = Instruction::Operation(ImgOp::Filter3x3(arr));
    Ok(stmt)
}

fn parse_resize(pair: Pair<'_, Rule>) -> Result<Instruction, String> {
    let tuple = parse_from_pair!(pair, (u32, u32))?;
    let stmt = Instruction::Operation(ImgOp::Resize(tuple));
    Ok(stmt)
}

fn parse_unsharpen(pair: Pair<'_, Rule>) -> Result<Instruction, String> {
    let tuple = parse_from_pair!(pair, (f32, i32))?;
    let stmt = Instruction::Operation(ImgOp::Unsharpen(tuple));
    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use crate::SICParser;
    use pest::Parser;
    use sic_core::image;
    use sic_image_engine::engine::EnvironmentItem;

    use super::*;

    #[test]
    fn test_parse_next_line_versions_fin_with_eoi() {
        let pairs = SICParser::parse(Rule::main, "blur 1;\nbrighten 2")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::Blur(1.0)),
                Instruction::Operation(ImgOp::Brighten(2))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_next_line_versions_fin_with_sep_eoi() {
        let pairs = SICParser::parse(Rule::main, "blur 1;\nbrighten 2;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::Blur(1.0)),
                Instruction::Operation(ImgOp::Brighten(2))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_next_line_versions_fin_with_sep_with_trailing_spaces_eoi() {
        let pairs = SICParser::parse(Rule::main, "blur 1;\nbrighten 2;    ")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::Blur(1.0)),
                Instruction::Operation(ImgOp::Brighten(2))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_single_line_versions_fin_with_eoi() {
        let pairs = SICParser::parse(Rule::main, "blur 1; brighten 2")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::Blur(1.0)),
                Instruction::Operation(ImgOp::Brighten(2))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_single_line_versions_fin_with_eoi_2() {
        let pairs = SICParser::parse(Rule::main, "blur 1;brighten 2")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::Blur(1.0)),
                Instruction::Operation(ImgOp::Brighten(2))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_single_line_versions_fin_with_sep_with_trailing_spaces_eoi() {
        let pairs = SICParser::parse(Rule::main, "blur 1; brighten 2;   ")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::Blur(1.0)),
                Instruction::Operation(ImgOp::Brighten(2))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_single_line_versions_require_sep() {
        SICParser::parse(Rule::main, "blur 4 blur 3").unwrap_or_else(|e| panic!("error: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_parse_single_line_versions_require_sep_2() {
        SICParser::parse(Rule::main, "blur 4\nblur 3").unwrap_or_else(|e| panic!("error: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_parse_single_line_versions_require_sep_3() {
        SICParser::parse(Rule::main, "blur 4 blur 3;").unwrap_or_else(|e| panic!("error: {:?}", e));
    }

    #[test]
    fn test_parse_single_line_versions_fin_with_sep_eoi() {
        let pairs = SICParser::parse(Rule::main, "blur 1;brighten 2;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::Blur(1.0)),
                Instruction::Operation(ImgOp::Brighten(2))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_require_space_between_operation_id_and_value() {
        SICParser::parse(Rule::main, "blur1; brighten 2")
            .unwrap_or_else(|e| panic!("error: {:?}", e));
    }

    #[test]
    fn test_blur_with_int_accept() {
        let pairs = SICParser::parse(Rule::main, "blur 15;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Blur(15.0))]),
            parse_image_operations(pairs)
        );
    }

    // related issue: https://github.com/foresterre/sic/issues/130
    #[test]
    fn test_blur_with_fp_accept() {
        let pairs = SICParser::parse(Rule::main, "blur 15.0;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Blur(15.0))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_blur_with_fp_neg_accept() {
        let pairs = SICParser::parse(Rule::main, "blur -15.0;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Blur(-15.0))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    #[should_panic]
    fn test_blur_with_fp_reject() {
        SICParser::parse(Rule::main, "blur 15.;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    fn test_crop_in_order_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "crop 1 2 3 4;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Crop((1, 2, 3, 4)))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_crop_ones_parse_correct() {
        // Here we don't check that rX > lX and rY > lY
        // We only check that the values are uint and in range (u32)
        // lX = upper left X coordinate
        // lY = upper left Y coordinate
        // rX = bottom right X coordinate
        // rY = bottom right Y coordinate

        let pairs = SICParser::parse(Rule::main, "crop 1 1 1 1;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Crop((1, 1, 1, 1)))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_crop_zeros_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "crop 0 0 0 0;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Crop((0, 0, 0, 0)))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    #[should_panic]
    fn test_crop_args_negative_parse_err() {
        SICParser::parse(Rule::main, "crop -1 -1 -1 -1;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_crop_arg_negative_p1_parse_err() {
        SICParser::parse(Rule::main, "crop -1 0 0 0;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_crop_arg_negative_p2_parse_err() {
        SICParser::parse(Rule::main, "crop 0 -1 0 0;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_crop_arg_negative_p3_parse_err() {
        SICParser::parse(Rule::main, "crop 0 0 -1 0;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_crop_arg_negative_p4_parse_err() {
        SICParser::parse(Rule::main, "crop 0 0 0 -1;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    fn test_crop_arg_to_big_p4_parse_err() {
        // 4294967296 == std::u32::MAX + 1
        let pairs = SICParser::parse(Rule::main, "crop 0 0 0 4294967296")
            .unwrap_or_else(|_| panic!("Unable to parse sic image operations script."));

        assert!(parse_image_operations(pairs).is_err())
    }

    #[test]
    fn test_crop_arg_just_in_range_p4_parse_ok() {
        // 4294967296 == std::u32::MAX
        let pairs = SICParser::parse(Rule::main, "crop 0 0 0 4294967295")
            .unwrap_or_else(|_| panic!("Unable to parse sic image operations script."));

        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Crop((
                0,
                0,
                0,
                std::u32::MAX,
            )))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_contrast_single_stmt_int_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "contrast 15;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Contrast(15.0))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_contrast_single_stmt_f32_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "contrast 15.8;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Contrast(15.8))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_contrast_single_stmt_parse_fail_end_in_dot() {
        let pairs = SICParser::parse(Rule::main, "contrast 15.;");
        assert!(pairs.is_err());
    }

    #[test]
    fn test_contrast_single_stmt_parse_fail_max_f32_1() {
        let pairs = SICParser::parse(Rule::main, "340282200000000000000000000000000000000.0;");
        assert!(pairs.is_err());
    }

    #[test]
    fn test_brighten_pos_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "brighten 3579;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Brighten(3579))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_brighten_neg_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "brighten -3579;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Brighten(-3579))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_triplets_f3_with_end_triplet_sep_fail() {
        SICParser::parse(Rule::main, "filter3x3 0 0 0 | 1 1 1 | 2 2 2 |")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    fn test_filter3x3_triplets_f3_no_end_triplet_sep() {
        let pairs = SICParser::parse(
            Rule::main,
            "filter3x3 0 0.1 0.2 | 1.3 1.4 1.5 | 2.6 2.7 2.8",
        )
        .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Filter3x3([
                0.0, 0.1, 0.2, 1.3, 1.4, 1.5, 2.6, 2.7, 2.8
            ]))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_filter3x3_triplets_f3_ensure_f32() {
        let pairs = SICParser::parse(Rule::main, "filter3x3 0 0 0 | 1 1 1 | 2 2 2")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Filter3x3([
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0
            ]))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_filter3x3_triplets_f3_no_sep() {
        let pairs = SICParser::parse(Rule::main, "filter3x3 0 0 0 1 1 1 2 2 3.0")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Filter3x3([
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 3.0
            ]))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_filter3x3_triplets_f3_end_op_sep() {
        let pairs = SICParser::parse(Rule::main, "filter3x3 0 0 0 1 1 1 2 2 3.0;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Filter3x3([
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 3.0
            ]))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_filter3x3_triplets_f3_sep_newline() {
        let pairs = SICParser::parse(Rule::main, "filter3x3\n0 0 0\n1 1 1\n2 2 3.0;");

        assert!(pairs.is_err())
    }

    #[test]
    fn test_filter3x3_triplets_f3_tabbed_spacing() {
        let pairs = SICParser::parse(Rule::main, "filter3x3 0 0 0\t1 1 1\t2 2 3;");

        assert!(pairs.is_err())
    }

    #[test]
    fn test_filter3x3_triplets_f3_indented_newlines() {
        let pairs = SICParser::parse(Rule::main, "filter3x3\n\t0 0 0\n\t1 1 1\n\t2 2 3");

        assert!(pairs.is_err())
    }

    #[test]
    fn test_filter3x3_duo_filter3x3() {
        let pairs = SICParser::parse(
            Rule::main,
            "filter3x3 1.9 2 3 | 4 5.9 6 | 7 8 9.9;\nfilter3x3 10.9 2 3 4 11.9 6 7 8 12.9",
        )
        .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::Filter3x3([
                    1.9, 2.0, 3.0, 4.0, 5.9, 6.0, 7.0, 8.0, 9.9
                ])),
                Instruction::Operation(ImgOp::Filter3x3([
                    10.9, 2.0, 3.0, 4.0, 11.9, 6.0, 7.0, 8.0, 12.9
                ])),
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_triplets_f3_require_spacing_on_triplet_sep_1() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 | 1 1.1 1|2.0 2 2")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_triplets_f3_require_spacing_on_triplet_sep_2() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 | 1 1.1 1 |2.0 2 2")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_triplets_f3_require_spacing_on_triplet_sep_3() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 | 1 1.1 1| 2.0 2 2")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_triplets_f3_require_spacing_on_triplet_sep_end_fail_1() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 | 1 1.1 1 | 2.0 2 2|")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_triplets_f3_require_spacing_on_triplet_sep_end_fail_2() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 | 1 1.1 1 | 2.0 2 2 | ")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_triplets_f3_require_all_triplet_sep_1() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 1 1.1 1 | 2.0 2 2 | ")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_triplets_f3_require_all_triplet_sep_2() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 | 1 1.1 1 2.0 2 2 | ")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_insufficient_args() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 | 1 1.1 1 999 | 2.0 2 2 | ")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_insufficient_triplet_count_4() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 | 1 2.2 3 | 2.0 2 2 | 0 1 2")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_filter3x3_insufficient_triplet_count_2() {
        SICParser::parse(Rule::main, "filter3x3 0 0.9 0 | 1 2.2 3")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
    }

    #[test]
    fn test_flip_horizontal_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "fliph;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::FlipHorizontal)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_flip_horizontal_removed() {
        let pairs = SICParser::parse(Rule::main, "flip_horizontal;");

        assert!(pairs.is_err());
    }

    #[test]
    fn test_flip_vertical_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "flipv;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::FlipVertical)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_flip_vertical_removed() {
        let pairs = SICParser::parse(Rule::main, "flip_vertical;");

        assert!(pairs.is_err());
    }

    #[test]
    fn test_hue_rotate_pos_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "huerotate 3579;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::HueRotate(3579))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_hue_rotate_neg_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "huerotate -3579;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::HueRotate(-3579))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_invert_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "invert;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Invert)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_resize_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "resize 99 88;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Resize((99, 88)))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_rotate90_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "rotate90;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Rotate90)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_rotate180_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "rotate180;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Rotate180)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_rotate270_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "rotate270;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Rotate270)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_unsharpen_single_stmt_parse_correct_ints() {
        let pairs = SICParser::parse(Rule::main, "unsharpen 99 88;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Unsharpen((99.0, 88)))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_unsharpen_single_stmt_parse_correct_fp_int() {
        let pairs = SICParser::parse(Rule::main, "unsharpen 99.0 88;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Unsharpen((99.0, 88)))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_unsharpen_single_stmt_parse_correct_fp_int_neg() {
        let pairs = SICParser::parse(Rule::main, "unsharpen -99.0 -88;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Instruction::Operation(ImgOp::Unsharpen((-99.0, -88)))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_unsharpen_single_stmt_parse_correct_fp_fp_fail() {
        let pairs = SICParser::parse(Rule::main, "unsharpen -99.0 -88.0;");
        assert!(pairs.is_err());
    }

    #[test]
    fn test_multi_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "blur 10;fliph;flipv;resize 100 200;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::Blur(10.0)),
                Instruction::Operation(ImgOp::FlipHorizontal),
                Instruction::Operation(ImgOp::FlipVertical),
                Instruction::Operation(ImgOp::Resize((100, 200)))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_multi_stmt_parse_diff_order_correct() {
        let pairs = SICParser::parse(Rule::main, "fliph;flipv;resize 100 200;blur 10;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::FlipHorizontal),
                Instruction::Operation(ImgOp::FlipVertical),
                Instruction::Operation(ImgOp::Resize((100, 200))),
                Instruction::Operation(ImgOp::Blur(10.0))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_multi_whitespace() {
        let pairs = SICParser::parse(Rule::main, "fliph; flipv; resize 100 200; blur 10;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::FlipHorizontal),
                Instruction::Operation(ImgOp::FlipVertical),
                Instruction::Operation(ImgOp::Resize((100, 200))),
                Instruction::Operation(ImgOp::Blur(10.0))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_multi_whitespace_2() {
        let pairs = SICParser::parse(
            Rule::main,
            "fliph    ; flipv   ;      resize 100 200; blur 10;",
        )
        .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::FlipHorizontal),
                Instruction::Operation(ImgOp::FlipVertical),
                Instruction::Operation(ImgOp::Resize((100, 200))),
                Instruction::Operation(ImgOp::Blur(10.0))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_multi_whitespace_3() {
        let pairs = SICParser::parse(Rule::main, "fliph;\nflipv;\nresize 100 200;\nblur 10;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::FlipHorizontal),
                Instruction::Operation(ImgOp::FlipVertical),
                Instruction::Operation(ImgOp::Resize((100, 200))),
                Instruction::Operation(ImgOp::Blur(10.0))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_multi_should_no_longer_end_with_sep() {
        let pairs = SICParser::parse(Rule::main, "fliph; flipv; resize 100 200; blur 10")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::FlipHorizontal),
                Instruction::Operation(ImgOp::FlipVertical),
                Instruction::Operation(ImgOp::Resize((100, 200))),
                Instruction::Operation(ImgOp::Blur(10.0))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_multi_sep() {
        let pairs = SICParser::parse(Rule::main, "fliph; flipv;  resize 100 200;\nblur 10")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![
                Instruction::Operation(ImgOp::FlipHorizontal),
                Instruction::Operation(ImgOp::FlipVertical),
                Instruction::Operation(ImgOp::Resize((100, 200))),
                Instruction::Operation(ImgOp::Blur(10.0))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_setopt_resize_sampling_filter_catmullrom() {
        let pairs = SICParser::parse(Rule::main, "set resize sampling_filter CatmullRom;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![Instruction::AddToEnv(
                EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::CatmullRom
                ))
            )]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_setopt_resize_sampling_filter_gaussian() {
        let pairs = SICParser::parse(Rule::main, "set resize sampling_filter GAUSSIAN;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![Instruction::AddToEnv(
                EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Gaussian
                ))
            ),]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_setopt_resize_sampling_filter_lanczos3() {
        let pairs = SICParser::parse(Rule::main, "set resize sampling_filter Lanczos3;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![Instruction::AddToEnv(
                EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Lanczos3
                ))
            ),]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_setopt_resize_sampling_filter_nearest() {
        let pairs = SICParser::parse(Rule::main, "set resize sampling_filter nearest;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![Instruction::AddToEnv(
                EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Nearest
                ))
            ),]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_setopt_resize_sampling_filter_triangle() {
        let pairs = SICParser::parse(Rule::main, "set resize sampling_filter triangle;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![Instruction::AddToEnv(
                EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Triangle
                ))
            ),]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_setopt_resize_sampling_filter_with_resize() {
        let pairs = SICParser::parse(
            Rule::main,
            "set   resize  sampling_filter   GAUSSIAN;\nresize 100 200",
        )
        .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Gaussian
                ))),
                Instruction::Operation(ImgOp::Resize((100, 200)))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_setopt_resize_sampling_filter_multi() {
        let pairs = SICParser::parse(
            Rule::main,
            "set resize sampling_filter catmullrom;\
             set resize sampling_filter gaussian;\
             set resize sampling_filter lanczos3;\
             set resize sampling_filter nearest;\
             set resize sampling_filter triangle;",
        )
        .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::CatmullRom
                ))),
                Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Gaussian
                ))),
                Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Lanczos3
                ))),
                Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Nearest
                ))),
                Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Triangle
                ))),
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_setopt_resize_preserve_aspect_ratio() {
        let pairs = SICParser::parse(
            Rule::main,
            "set resize preserve_aspect_ratio;\nresize 100 200",
        )
        .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::AddToEnv(EnvironmentItem::PreserveAspectRatio),
                Instruction::Operation(ImgOp::Resize((100, 200)))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_setopt_resize_preserve_aspect_ratio_no_value() {
        SICParser::parse(
            Rule::main,
            "set resize preserve_aspect_ratio true;\nresize 100 200",
        )
        .unwrap_or_else(|e| panic!("error: {:?}", e));
    }

    #[test]
    fn test_parse_delopt_resize_sampling_filter_single() {
        let pairs = SICParser::parse(Rule::main, "del resize sampling_filter;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![Instruction::RemoveFromEnv(
                EnvironmentKind::CustomSamplingFilter
            ),]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_set_and_del_opt_resize_sampling_filter_multi() {
        let pairs = SICParser::parse(
            Rule::main,
            "set resize sampling_filter catmullrom;\
             set resize sampling_filter gaussian;\
             del resize sampling_filter;\
             del resize sampling_filter;",
        )
        .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::CatmullRom
                ))),
                Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                    image::FilterType::Gaussian
                ))),
                Instruction::RemoveFromEnv(EnvironmentKind::CustomSamplingFilter),
                Instruction::RemoveFromEnv(EnvironmentKind::CustomSamplingFilter),
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_delopt_resize_preserve_aspect_ratio_single() {
        let pairs = SICParser::parse(Rule::main, "del resize preserve_aspect_ratio;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![Instruction::RemoveFromEnv(
                EnvironmentKind::PreserveAspectRatio
            ),]),
            parse_image_operations(pairs)
        );
    }
}
