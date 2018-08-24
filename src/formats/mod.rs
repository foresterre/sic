use pest::Parser;

#[derive(Parser)]
#[grammar = "formats/multiformat.pest"]
struct ImageMultiFormatParser;