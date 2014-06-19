#![crate_id="cargo-git-checkout"]

extern crate cargo;
extern crate serialize;
extern crate hammer;
extern crate url;

use hammer::FlagConfig;
use cargo::{execute_main_without_stdin};
use cargo::core::source::{Source,SourceId};
use cargo::sources::git::{GitSource};
use cargo::util::{Config, CliResult, CliError, Require};
use url::Url;

#[deriving(PartialEq,Clone,Decodable)]
struct Options {
    url: String,
    reference: String,
    verbose: bool
}

impl FlagConfig for Options {}

fn main() {
    execute_main_without_stdin(execute);
}

fn execute(options: Options) -> CliResult<Option<()>> {
    let Options { url, reference, .. } = options;

    let url: Url = try!(from_str(url.as_slice())
                        .require(|| format!("The URL `{}` you passed was not a valid URL", url))
                        .map_err(|e| CliError::from_boxed(e, 1)));

    let source_id = SourceId::for_git(&url, reference.as_slice());

    let mut source = GitSource::new(&source_id, &try!(Config::new().map_err(|e| CliError::from_boxed(e, 1))));

    try!(source.update().map_err(|e| CliError::new(format!("Couldn't update {}: {}", source, e), 1)));

    Ok(None)
}
