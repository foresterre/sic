use pest::iterators::{Pair, Pairs};
use sic_image_engine::engine::{EnvironmentItem, EnvironmentKind, Program, Statement};
use sic_image_engine::wrapper::filter_type::FilterTypeWrap;
use sic_image_engine::Operation;

use super::Rule;

// This function parses Operations from the Pest parsed syntax, as defined by
// [grammar.pest].
// It returns an error (Err) in case of any parse failure.
// The error currently contains a String, but this will need to be reworked to proper error types.
// The function is supposed to never panic.
pub fn parse_image_operations(pairs: Pairs<'_, Rule>) -> Result<Program, String> {
    pairs
        .filter(|pair| pair.as_rule() != Rule::EOI)
        .map(|pair| match pair.as_rule() {
            Rule::blur => parse_unop_f32(pair).map(|v| Statement::Operation(Operation::Blur(v))),
            Rule::brighten => {
                parse_unop_i32(pair).map(|v| Statement::Operation(Operation::Brighten(v)))
            }
            Rule::contrast => {
                parse_unop_f32(pair).map(|v| Statement::Operation(Operation::Contrast(v)))
            }
            Rule::crop => parse_4_tuple_crop_result(parse_quad_op_u32(pair)),
            Rule::filter3x3 => {
                parse_triplet3x_f32(pair).map(|v| Statement::Operation(Operation::Filter3x3(v)))
            }
            Rule::flip_horizontal => Ok(Statement::Operation(Operation::FlipHorizontal)),
            Rule::flip_vertical => Ok(Statement::Operation(Operation::FlipVertical)),
            Rule::grayscale => Ok(Statement::Operation(Operation::GrayScale)),
            Rule::huerotate => {
                parse_unop_i32(pair).map(|v| Statement::Operation(Operation::HueRotate(v)))
            }
            Rule::invert => Ok(Statement::Operation(Operation::Invert)),
            Rule::resize => {
                let (x, y) = parse_binop_u32(pair);
                x.and_then(|ux| y.map(|uy| Statement::Operation(Operation::Resize(ux, uy))))
            }
            Rule::rotate90 => Ok(Statement::Operation(Operation::Rotate90)),
            Rule::rotate180 => Ok(Statement::Operation(Operation::Rotate180)),
            Rule::rotate270 => Ok(Statement::Operation(Operation::Rotate270)),
            Rule::unsharpen => {
                let (x, y) = parse_binop_f32_i32(pair);
                x.and_then(|ux| y.map(|uy| Statement::Operation(Operation::Unsharpen(ux, uy))))
            }
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

fn parse_set_environment(pair: Pair<'_, Rule>) -> Result<Statement, String> {
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

    Ok(Statement::RegisterEnvironmentItem(environment_item))
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
        .map(EnvironmentItem::OptResizeSamplingFilter)
}

fn parse_unset_environment(pair: Pair<'_, Rule>) -> Result<Statement, String> {
    let environment_item = match pair.as_rule() {
        Rule::env_resize_sampling_filter_name => EnvironmentKind::OptResizeSamplingFilter,
        Rule::env_resize_preserve_aspect_ratio_name => {
            EnvironmentKind::OptResizePreserveAspectRatio
        }
        _ => {
            return Err(format!(
                "Unable to parse `del` environment command. Error on element: {}",
                pair
            ));
        }
    };

    Ok(Statement::DeregisterEnvironmentItem(environment_item))
}

// generalizing this to T1/T2 would be nice, but gave me a lot of headaches. Using this for now.
fn parse_unop_f32(pair: Pair<'_, Rule>) -> Result<f32, String> {
    let mut inner = pair.into_inner();

    inner
        .next()
        .ok_or_else(|| format!("Unable to parse {}, too many arguments: {}", inner, 1))
        .map(|val| val.as_str())
        .and_then(|it: &str| it.parse::<f32>().map_err(|err| err.to_string()))
}

fn parse_unop_i32(pair: Pair<'_, Rule>) -> Result<i32, String> {
    pair.into_inner()
        .next()
        .ok_or_else(|| "Unable to parse UnOp::i32, Expected 2 arguments.".to_string())
        .map(|val| val.as_str())
        .and_then(|it: &str| it.parse::<i32>().map_err(|err| err.to_string()))
}

fn parse_binop_u32(pair: Pair<'_, Rule>) -> (Result<u32, String>, Result<u32, String>) {
    let mut inner = pair.into_inner();

    let x_text = inner
        .next()
        .ok_or_else(|| "Unable to parse BinOp::<u32, u32> _1".to_string())
        .map(|val| val.as_str());

    let x = x_text.and_then(|it: &str| it.parse::<u32>().map_err(|err| err.to_string()));

    let y_text = inner
        .next()
        .ok_or_else(|| "Unable to parse BinOp::<u32, u32> _2".to_string())
        .map(|val| val.as_str());

    let y = y_text.and_then(|it: &str| it.parse::<u32>().map_err(|err| err.to_string()));

    (x, y)
}

fn parse_binop_f32_i32(pair: Pair<'_, Rule>) -> (Result<f32, String>, Result<i32, String>) {
    let mut inner = pair.into_inner();

    let x_text = inner
        .next()
        .ok_or_else(|| "Unable to parse BinOp::<f32, i32> _1".to_string())
        .map(|val| val.as_str());

    let x = x_text.and_then(|it: &str| it.parse::<f32>().map_err(|err| err.to_string()));

    let y_text = inner
        .next()
        .ok_or_else(|| "Unable to parse BinOp::<f32, i32> _2".to_string())
        .map(|val| val.as_str());

    let y = y_text.and_then(|it: &str| it.parse::<i32>().map_err(|err| err.to_string()));

    (x, y)
}

type QuadOpU32 = (
    Result<u32, String>,
    Result<u32, String>,
    Result<u32, String>,
    Result<u32, String>,
);

// simplify tuple with individual error messages
fn parse_4_tuple_crop_result(tuple: QuadOpU32) -> Result<Statement, String> {
    let (xl, yl, xr, yr) = tuple;
    xl.and_then(|lxu| {
        yl.and_then(|lyu| {
            xr.and_then(|rxu| {
                yr.map(|ryu| Statement::Operation(Operation::Crop(lxu, lyu, rxu, ryu)))
            })
        })
    })
}

fn parse_quad_op_u32(pair: Pair<'_, Rule>) -> QuadOpU32 {
    let mut inner = pair.into_inner();

    let left_top_corner_x = inner
        .next()
        .ok_or_else(|| "Unable to parse QuadOp::<u32, u32, u32, u32> (n=1)".to_string())
        .map(|val| val.as_str())
        .and_then(|it: &str| it.parse::<u32>().map_err(|err| err.to_string()));

    let left_top_corner_y = inner
        .next()
        .ok_or_else(|| "Unable to parse QuadOp::<u32, u32, u32, u32> (n=2)".to_string())
        .map(|val| val.as_str())
        .and_then(|it: &str| it.parse::<u32>().map_err(|err| err.to_string()));

    let right_bottom_corner_x = inner
        .next()
        .ok_or_else(|| "Unable to parse QuadOp::<u32, u32, u32, u32> (n=3)".to_string())
        .map(|val| val.as_str())
        .and_then(|it: &str| it.parse::<u32>().map_err(|err| err.to_string()));

    let right_bottom_corner_y = inner
        .next()
        .ok_or_else(|| "Unable to parse QuadOp::<u32, u32, u32, u32> (n=3)".to_string())
        .map(|val| val.as_str())
        .and_then(|it: &str| it.parse::<u32>().map_err(|err| err.to_string()));

    (
        left_top_corner_x,
        left_top_corner_y,
        right_bottom_corner_x,
        right_bottom_corner_y,
    )
}

// The code below, should work for parsing the 9 elements of a 3x3 fp32 triplet structure, but
// let's be honest; this code can't be called beautiful. This should be refactored.
fn parse_triplet3x_f32(pair: Pair<'_, Rule>) -> Result<[f32; 9], String> {
    let mut inner = pair.into_inner();

    fn parse_fp32(it: &str) -> Result<f32, String> {
        it.parse::<f32>().map_err(|err| err.to_string())
    }

    fn err(inner: &str, i: u32) -> String {
        format!("Unable to parse {}, arguments #: {}", inner, i)
    }

    // A bit packed, but type of Pair<?> is unclear, therefor we map to &str first.
    // Since arrays are limited and no collect exists for an array, we create the array from individually
    // constructed elements.
    let arr: [f32; 9] = [
        inner
            .next()
            .ok_or_else(|| err(&inner.to_string(), 1))
            .map(|pair| pair.as_str())
            .and_then(parse_fp32)?,
        inner
            .next()
            .ok_or_else(|| err(&inner.to_string(), 2))
            .map(|pair| pair.as_str())
            .and_then(parse_fp32)?,
        inner
            .next()
            .ok_or_else(|| err(&inner.to_string(), 3))
            .map(|pair| pair.as_str())
            .and_then(parse_fp32)?,
        inner
            .next()
            .ok_or_else(|| err(&inner.to_string(), 4))
            .map(|pair| pair.as_str())
            .and_then(parse_fp32)?,
        inner
            .next()
            .ok_or_else(|| err(&inner.to_string(), 5))
            .map(|pair| pair.as_str())
            .and_then(parse_fp32)?,
        inner
            .next()
            .ok_or_else(|| err(&inner.to_string(), 6))
            .map(|pair| pair.as_str())
            .and_then(parse_fp32)?,
        inner
            .next()
            .ok_or_else(|| err(&inner.to_string(), 7))
            .map(|pair| pair.as_str())
            .and_then(parse_fp32)?,
        inner
            .next()
            .ok_or_else(|| err(&inner.to_string(), 8))
            .map(|pair| pair.as_str())
            .and_then(parse_fp32)?,
        inner
            .next()
            .ok_or_else(|| err(&inner.to_string(), 9))
            .map(|pair| pair.as_str())
            .and_then(parse_fp32)?,
    ];

    Ok(arr)
}

#[cfg(test)]
mod tests {
    use crate::SICParser;
    use pest::Parser;
    use sic_core::combostew::image;
    use sic_image_engine::engine::EnvironmentItem;

    use super::*;

    #[test]
    fn test_parse_next_line_versions_fin_with_eoi() {
        let pairs = SICParser::parse(Rule::main, "blur 1;\nbrighten 2")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![
                Statement::Operation(Operation::Blur(1.0)),
                Statement::Operation(Operation::Brighten(2))
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
                Statement::Operation(Operation::Blur(1.0)),
                Statement::Operation(Operation::Brighten(2))
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
                Statement::Operation(Operation::Blur(1.0)),
                Statement::Operation(Operation::Brighten(2))
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
                Statement::Operation(Operation::Blur(1.0)),
                Statement::Operation(Operation::Brighten(2))
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
                Statement::Operation(Operation::Blur(1.0)),
                Statement::Operation(Operation::Brighten(2))
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
                Statement::Operation(Operation::Blur(1.0)),
                Statement::Operation(Operation::Brighten(2))
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
                Statement::Operation(Operation::Blur(1.0)),
                Statement::Operation(Operation::Brighten(2))
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
    fn test_blur_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "blur 15;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Blur(15.0))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_crop_in_order_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "crop 1 2 3 4;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Crop(1, 2, 3, 4))]),
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
            Ok(vec![Statement::Operation(Operation::Crop(1, 1, 1, 1))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_crop_zeros_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "crop 0 0 0 0;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Crop(0, 0, 0, 0))]),
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
            Ok(vec![Statement::Operation(Operation::Crop(
                0,
                0,
                0,
                std::u32::MAX,
            ))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_contrast_single_stmt_int_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "contrast 15;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Contrast(15.0))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_contrast_single_stmt_f32_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "contrast 15.8;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Contrast(15.8))]),
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
            Ok(vec![Statement::Operation(Operation::Brighten(3579))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_brighten_neg_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "brighten -3579;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Brighten(-3579))]),
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
            Ok(vec![Statement::Operation(Operation::Filter3x3([
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
            Ok(vec![Statement::Operation(Operation::Filter3x3([
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
            Ok(vec![Statement::Operation(Operation::Filter3x3([
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
            Ok(vec![Statement::Operation(Operation::Filter3x3([
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
                Statement::Operation(Operation::Filter3x3([
                    1.9, 2.0, 3.0, 4.0, 5.9, 6.0, 7.0, 8.0, 9.9
                ])),
                Statement::Operation(Operation::Filter3x3([
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
            Ok(vec![Statement::Operation(Operation::FlipHorizontal)]),
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
            Ok(vec![Statement::Operation(Operation::FlipVertical)]),
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
            Ok(vec![Statement::Operation(Operation::HueRotate(3579))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_hue_rotate_neg_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "huerotate -3579;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::HueRotate(-3579))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_invert_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "invert;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Invert)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_resize_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "resize 99 88;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Resize(99, 88))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_rotate90_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "rotate90;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Rotate90)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_rotate180_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "rotate180;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Rotate180)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_rotate270_single_stmt_parse_correct() {
        let pairs = SICParser::parse(Rule::main, "rotate270;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Rotate270)]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_unsharpen_single_stmt_parse_correct_ints() {
        let pairs = SICParser::parse(Rule::main, "unsharpen 99 88;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Unsharpen(99.0, 88))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_unsharpen_single_stmt_parse_correct_fp_int() {
        let pairs = SICParser::parse(Rule::main, "unsharpen 99.0 88;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Unsharpen(99.0, 88))]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_unsharpen_single_stmt_parse_correct_fp_int_neg() {
        let pairs = SICParser::parse(Rule::main, "unsharpen -99.0 -88;")
            .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));
        assert_eq!(
            Ok(vec![Statement::Operation(Operation::Unsharpen(-99.0, -88))]),
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
                Statement::Operation(Operation::Blur(10.0)),
                Statement::Operation(Operation::FlipHorizontal),
                Statement::Operation(Operation::FlipVertical),
                Statement::Operation(Operation::Resize(100, 200))
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
                Statement::Operation(Operation::FlipHorizontal),
                Statement::Operation(Operation::FlipVertical),
                Statement::Operation(Operation::Resize(100, 200)),
                Statement::Operation(Operation::Blur(10.0))
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
                Statement::Operation(Operation::FlipHorizontal),
                Statement::Operation(Operation::FlipVertical),
                Statement::Operation(Operation::Resize(100, 200)),
                Statement::Operation(Operation::Blur(10.0))
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
                Statement::Operation(Operation::FlipHorizontal),
                Statement::Operation(Operation::FlipVertical),
                Statement::Operation(Operation::Resize(100, 200)),
                Statement::Operation(Operation::Blur(10.0))
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
                Statement::Operation(Operation::FlipHorizontal),
                Statement::Operation(Operation::FlipVertical),
                Statement::Operation(Operation::Resize(100, 200)),
                Statement::Operation(Operation::Blur(10.0))
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
                Statement::Operation(Operation::FlipHorizontal),
                Statement::Operation(Operation::FlipVertical),
                Statement::Operation(Operation::Resize(100, 200)),
                Statement::Operation(Operation::Blur(10.0))
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
                Statement::Operation(Operation::FlipHorizontal),
                Statement::Operation(Operation::FlipVertical),
                Statement::Operation(Operation::Resize(100, 200)),
                Statement::Operation(Operation::Blur(10.0))
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_setopt_resize_sampling_filter_catmullrom() {
        let pairs = SICParser::parse(Rule::main, "set resize sampling_filter CatmullRom;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![Statement::RegisterEnvironmentItem(
                EnvironmentItem::OptResizeSamplingFilter(FilterTypeWrap::Inner(
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
            Ok(vec![Statement::RegisterEnvironmentItem(
                EnvironmentItem::OptResizeSamplingFilter(FilterTypeWrap::Inner(
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
            Ok(vec![Statement::RegisterEnvironmentItem(
                EnvironmentItem::OptResizeSamplingFilter(FilterTypeWrap::Inner(
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
            Ok(vec![Statement::RegisterEnvironmentItem(
                EnvironmentItem::OptResizeSamplingFilter(FilterTypeWrap::Inner(
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
            Ok(vec![Statement::RegisterEnvironmentItem(
                EnvironmentItem::OptResizeSamplingFilter(FilterTypeWrap::Inner(
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
                Statement::RegisterEnvironmentItem(EnvironmentItem::OptResizeSamplingFilter(
                    FilterTypeWrap::Inner(image::FilterType::Gaussian)
                )),
                Statement::Operation(Operation::Resize(100, 200))
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
                Statement::RegisterEnvironmentItem(EnvironmentItem::OptResizeSamplingFilter(
                    FilterTypeWrap::Inner(image::FilterType::CatmullRom)
                )),
                Statement::RegisterEnvironmentItem(EnvironmentItem::OptResizeSamplingFilter(
                    FilterTypeWrap::Inner(image::FilterType::Gaussian)
                )),
                Statement::RegisterEnvironmentItem(EnvironmentItem::OptResizeSamplingFilter(
                    FilterTypeWrap::Inner(image::FilterType::Lanczos3)
                )),
                Statement::RegisterEnvironmentItem(EnvironmentItem::OptResizeSamplingFilter(
                    FilterTypeWrap::Inner(image::FilterType::Nearest)
                )),
                Statement::RegisterEnvironmentItem(EnvironmentItem::OptResizeSamplingFilter(
                    FilterTypeWrap::Inner(image::FilterType::Triangle)
                )),
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
                Statement::RegisterEnvironmentItem(EnvironmentItem::PreserveAspectRatio),
                Statement::Operation(Operation::Resize(100, 200))
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
            Ok(vec![Statement::DeregisterEnvironmentItem(
                EnvironmentKind::OptResizeSamplingFilter
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
                Statement::RegisterEnvironmentItem(EnvironmentItem::OptResizeSamplingFilter(
                    FilterTypeWrap::Inner(image::FilterType::CatmullRom)
                )),
                Statement::RegisterEnvironmentItem(EnvironmentItem::OptResizeSamplingFilter(
                    FilterTypeWrap::Inner(image::FilterType::Gaussian)
                )),
                Statement::DeregisterEnvironmentItem(EnvironmentKind::OptResizeSamplingFilter),
                Statement::DeregisterEnvironmentItem(EnvironmentKind::OptResizeSamplingFilter),
            ]),
            parse_image_operations(pairs)
        );
    }

    #[test]
    fn test_parse_delopt_resize_preserve_aspect_ratio_single() {
        let pairs = SICParser::parse(Rule::main, "del resize preserve_aspect_ratio;")
            .unwrap_or_else(|e| panic!("error: {:?}", e));

        assert_eq!(
            Ok(vec![Statement::DeregisterEnvironmentItem(
                EnvironmentKind::OptResizePreserveAspectRatio
            ),]),
            parse_image_operations(pairs)
        );
    }
}
