// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
A Rust library to work with CPS (Composite Primitive Schema) data and parse it from and serialize
it to several binary and textual representation formats, such as YAML, JSON, and CBOR.

A useful side effect of this bi-direction is that Compris can be used to convert between these
formats.

What is CPS? It's the implicit, common data schema underlying these representation formats. It
comprises primitive data types (numbers, booleans, strings, etc.) as well as list and map
collection types, which enable a nested (recursive) structure. Hence it is "composite" (a.k.a.
"algebraic").

And yet despite being so widely used, CPS has been unnamed... until now. You're welcome.

CPS is sometimes glossed as "JSON", but that's misleading and ultimately unhelpful because JSON is
merely one representation format for the data, and is actually comparatively quite limited (e.g.
implementations do not often preserve the distinction between integers and floats). So instead of
saying "let's just store it as JSON", say "let's just store it as CPS", and use Compris to handle
the representation. It will allow you and your users to select from all supported formats at
runtime.

See [here](https://github.com/tliron/compris/blob/main/CPS.md) for a full description of CPS.

Compris is pronounced "com-PREE". The name comes from shortening CompositePrimitiveSchema to
ComPriS.

For more information and usage examples see the
[home page](https://github.com/tliron/compris).

J'ai compris!
*/

mod commands;
mod run;

use run::*;

use std::process::*;

/// Main.
pub fn main() -> ExitCode {
    kutil::cli::run::run(run)
}
