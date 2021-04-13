# Contributing

Contributions to this project of every form are very welcome! When
contributing, please be welcoming, polite, kind and inclusive.
You can contribute in various different forms, e.g. by committing
code, writing documentation and opening issues in the form of
bug reports or feature requests. Your contributions are very much
appreciated, thank you :)

If you intend to contribute something, which requires some more work
or where it is not entirely clear if it fits to this project, please
make sure beforehand that it goes into the right direction by either
opening a `discussion` issue or by making your code available before
you want it to be merged.

## Branching

This project mainly uses two branches. The branch `dev` is the development
branch which contains the latest development snapshot of the code.
It should be used for branching off into feature branches. These
feature branches are then, when finished, merged back into the
`dev` branch in the form of pull requests. After collecting some
functionality, bug fixes, etc... on the `dev` branch it gets merged
into the `main` branch. These commits are then tagged and published as
a library crate ([Ethane](https://crates.io/crates/ethane)) to
crates.io, the Rust crate registry.

## Conventions

In order to make life easy for users as well as contributors of
this library we should stick to some rules:
- Formatting: Use `rust fmt`.
- Lints: Use `cargo clippy --all`.
- Documentation:
  - Please document `///` or `//!` the public API.
  - Use line comments `//` where you think, your comment reduces
    the time for another developer to understand what is
    going on. This does **not** mean to use comments everywhere.
- Testing: Make sure to run `cargo t` successfully before handing in
your pull requests.
- Commits: Use meaningful commit messages. Ideally, use the first line to sum
up, what you changed, leave one line blank and then go into details.
  
I will probably add compile-time checks of some of these rules soon :)

**Happy Coding!**
