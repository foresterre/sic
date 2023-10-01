use sic_core::image::ImageFormat;
use std::env;
use std::path::PathBuf;
use wax::{FileIterator, FilterTarget, Glob, WalkEntry};

#[derive(Default)]
pub struct GlobResolver {
    skip_unsupported: bool,
}

impl GlobResolver {
    pub fn with_skip_unsupported(mut self, value: bool) -> Self {
        self.skip_unsupported = value;
        self
    }

    pub fn resolve_glob(&self, expression: &str) -> anyhow::Result<Vec<PathBuf>> {
        // pre-process the glob expression, to a format `wax` can deal with
        let expression = Self::preprocess(expression);

        let glob = Glob::new(expression.as_ref())?;
        let (prefix, glob) = glob.partition();

        let cwd = env::current_dir()?;
        let canonicalized = dunce::canonicalize(cwd.join(&prefix))?;

        dbg!(&canonicalized, &glob);

        // TODO add back image crate identifier fallback
        // TODO merge input format from image crate and sic, or remove sic decider altogether in favour of image crate only

        let walk = glob.walk(canonicalized);
        let walk = walk
            .filter_tree(is_file)
            .filter_tree(|entry| is_supported_ext(self.skip_unsupported, entry))
            .inspect(|s| {
                dbg!(s);
            });

        let paths = walk
            .map(|result| {
                result
                    .map(|entry| entry.into_path())
                    .map_err(|err| anyhow::anyhow!(err))
            })
            .inspect(|e| {
                eprintln!("E: {:?}", e);
            })
            .collect::<anyhow::Result<Vec<PathBuf>>>()?;

        Ok(paths)
    }

    #[cfg(windows)]
    fn preprocess(expression: &str) -> impl AsRef<str> {
        // `wax`, the glob library used, only supports forward slashes in glob patterns!
        expression
            .chars()
            .map(|c| if c == '\\' { '/' } else { c })
            .collect::<String>()

        // TODO: drive letters are not supported here!
    }

    #[cfg(not(windows))]
    fn preprocess(expression: &str) -> impl AsRef<str> + '_ {
        expression
    }
}

fn is_file(entry: &WalkEntry) -> Option<FilterTarget> {
    (!entry.file_type().is_file()).then_some(FilterTarget::File)
}

fn is_supported_ext(enabled: bool, entry: &WalkEntry) -> Option<FilterTarget> {
    if !enabled {
        return None;
    }

    entry.path().extension().and_then(|ext| {
        ImageFormat::from_extension(ext)
            .is_none()
            .then_some(FilterTarget::File)
    })
}
