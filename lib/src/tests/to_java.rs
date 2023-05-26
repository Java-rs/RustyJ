use crate::types::*;

pub fn prg_to_java(prg: &Prg) -> String {
    let mut s: String = String::new();
    for class in prg {
        s = format!("{}\n\n", class_to_java(class));
    }
    s
}

pub fn class_to_java(class: &Class) -> String {
    let mut s: String = format!("class {} ", class.name);
    s += "{";
    s
}
