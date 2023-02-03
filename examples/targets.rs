fn main() {
    let targets = rustc_targets::from_cli().unwrap();
    for (i,target) in targets.iter().unwrap().enumerate() {
        println!("{i}. {target}");
    }
}