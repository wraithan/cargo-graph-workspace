# cargo-graph-workspace

Tossed this together for fun, [cargo-dot](https://github.com/maxsnew/cargo-dot)
is deprecated and [cargo-graph](https://github.com/kbknapp/cargo-graph) doesn't
handle workspaces correctly, so I built a minimal thing that uses
[cargo_metadata](https://github.com/oli-obk/cargo_metadata) instead of
`Cargo.lock` parsing. From there working with workspaces is trivial.

I'm going to propose switching to a method like this for `cargo-graph` and maybe
toss up a PR. I wanted to understand the library and some of the problems first.

See [example.dot](example.dot) and [example.png](example.png) for a preview of
what output looks like.

I'm not planning on uploading this to crates.io, since I don't think I'll be
maintaining this. But if you are building a similar project and would like to
use my code, the MIT license should make that easy.

## installing

If you decide you want to install this you can do so with:

```
cargo install --git https://github.com/wraithan/cargo-graph-workspace.git
```

## usage

```
cargo graph-workspace
Creates a dependency graph in a graphviz file named output.dot

USAGE:
    cargo graph-workspace [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --manifest-path <PATH>    Path to the Cargo.lock you want to graph.
```

The command will output a `output.dot` use that with `dot` to create image files
with a command such as:

```
dot -Tpng output.dot > output.png
```