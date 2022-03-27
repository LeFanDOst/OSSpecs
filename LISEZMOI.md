# OSSpecs

[EN](README.md) / FR

Une petite crate Rust pour l'implémentation de certaines fonctionnalités utiles cross-platform pour certaines choses spécifiques à chaque OS.

# Sommaire

* [But](#but)
* [Avertissement](#avertissement)
* [Sources](#sources)

# But

Cette crate a pour but de pouvoir utiliser des fonctionnalités qui sont implémentées de manière différente ou inexistantes
selon le système d'exploitation. Cela de manière harmonisée, dans une seule crate Rust.

# Avertissement

Pour récupérer les informations des CPUs sous Linux, cette crate exécute une commande spécifique, qui est `lscpu`. Cette commande
est déjà pré-installée sur beaucoup de distributions Linux. Mais, dans le cas où cette commande n'est pas trouvée, vous pouvez
[visiter ce site](https://www.golinuxcloud.com/lscpu-command-in-linux/#:~:text=lscpu%20is%20a%20command%2Dline,Memory%20Access%20(NUMA)%20nodes.) pour avoir
quelques informations sur comment installer cette commande. *Cet avertissement sera probablement supprimé sur une future mise à jour
de cette crate.*

# Sources

Ces quelques sources ci-dessous m'ont aidé au développement de cette crate.

* [Struct std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
* [How can one detect the OS type using Rust?](https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust)
* [Constant std::env::consts::OS](https://doc.rust-lang.org/std/env/consts/constant.OS.html)
* [Rust - Modules](https://www.tutorialspoint.com/rust/rust_modules.htm)
* [Crate wmi](https://docs.rs/wmi/latest/wmi/)
* [Conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
* [Skipping deserialized values: any way to make it conditional?](https://www.reddit.com/r/rust/comments/bykv1o/skipping_deserialized_values_any_way_to_make_it/)