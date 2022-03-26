# OSSpecs

[EN / FR]

A little Rust crate for implementation of some cross-platform useful functionalities from OS-Specific things.

Une petite crate Rust pour l'implémentation de certaines fonctionnalités utiles cross-platform pour certaines choses spécifiques à chaque OS.

# Languages

* [English part](#english-part)
* [Partie en français](#partie-en-français)

# English part

# Summary

* [Sources](#sources)

# Sources

These few sources were a good help for this crate's development.

* [Struct std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
* [How can one detect the OS type using Rust?](https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust)
* [Constant std::env::consts::OS](https://doc.rust-lang.org/std/env/consts/constant.OS.html)
* [Rust - Modules](https://www.tutorialspoint.com/rust/rust_modules.htm)
* [Crate wmi](https://docs.rs/wmi/latest/wmi/)
* [Conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
* [Skipping deserialized values: any way to make it conditional?](https://www.reddit.com/r/rust/comments/bykv1o/skipping_deserialized_values_any_way_to_make_it/)

# Partie en français

# Sommaire

* [But](#but)
* [Sources](#sources)

# But

Cette crate a pour but de pouvoir utiliser des fonctionnalités qui sont implémentées de manière différente ou inexistantes
selon le système d'exploitation. Cela de manière harmonisée, dans une seule crate Rust.

# Sources

Ces quelques sources ci-dessous m'ont aidé au développement de cette crate.

* [Struct std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
* [How can one detect the OS type using Rust?](https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust)
* [Constant std::env::consts::OS](https://doc.rust-lang.org/std/env/consts/constant.OS.html)
* [Rust - Modules](https://www.tutorialspoint.com/rust/rust_modules.htm)
* [Crate wmi](https://docs.rs/wmi/latest/wmi/)
* [Conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
* [Skipping deserialized values: any way to make it conditional?](https://www.reddit.com/r/rust/comments/bykv1o/skipping_deserialized_values_any_way_to_make_it/)