// main.rs - Comprehensive Rust starter script
struct Project {
    name: String,
    version: String,
}
fn main() {
    let p = Project {
        name: String::from("GitHub Automated Repository Project"),
        version: String::from("1.0.0"),
    };
    println!("Project: {}, Version: {}", p.name, p.version);
    println!("\nFeatures: Structs, Ownership, Loops");
    for i in 0..5 {
        println!("  - Iteration {}", i + 1);
    }
}

# Modified on 2025-07-11T00:00:00