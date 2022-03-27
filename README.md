# OSSpecs

EN / [FR](LISEZMOI.md)

A little Rust crate for implementation of some cross-platform useful functionalities from OS-Specific things.

# Summary

* [Purpose](#purpose)
* [Warning](#warning)
* [Sources](#sources)

# Purpose

The purpose of this crate is to be able to use features that are implemented differently or not at all
depending on the operating system. This in a harmonized way, in a single Rust crate.

# Warning

To get CPU's information on Linux, this crate execute a specific command, which is `lscpu`. This command is already
pre-installed on a lot of Linux distribution. But, in case if it is not found, you can [visit this site](https://www.golinuxcloud.com/lscpu-command-in-linux/#:~:text=lscpu%20is%20a%20command%2Dline,Memory%20Access%20(NUMA)%20nodes.)
to get some informations on how install this command. *This warning will probably be removed on a future update of
the crate.*

# Sources

These few sources were a good help for this crate's development.

* [Struct std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
* [How can one detect the OS type using Rust?](https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust)
* [Constant std::env::consts::OS](https://doc.rust-lang.org/std/env/consts/constant.OS.html)
* [Rust - Modules](https://www.tutorialspoint.com/rust/rust_modules.htm)
* [Crate wmi](https://docs.rs/wmi/latest/wmi/)
* [Conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
* [Skipping deserialized values: any way to make it conditional?](https://www.reddit.com/r/rust/comments/bykv1o/skipping_deserialized_values_any_way_to_make_it/)