use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run --bin new_problem <number> <name>");
        return;
    }

    let number: u32 = args[1].parse().expect("Invalid number");
    let name = args[2].to_lowercase().replace(' ', "_");
    let file_name = format!("p{:05}_{}.rs", number, name);
    let test_file = format!("test_p{:05}_{}.rs", number, name);

    let problems_dir = Path::new("src/problems");
    let tests_dir = Path::new("tests");

    fs::create_dir_all(problems_dir).unwrap();
    fs::create_dir_all(tests_dir).unwrap();

    // 创建题目文件
    let problem_path = problems_dir.join(&file_name);
    if !problem_path.exists() {
        let template = "pub struct Solution;\n\nimpl Solution {\n    pub fn solve() {\n        // TODO\n    }\n}\n";
        fs::write(&problem_path, template).unwrap();
    }

    // 更新 mod.rs
    let mod_file = problems_dir.join("mod.rs");
    let mod_line = format!("pub mod {};\n", file_name.trim_end_matches(".rs"));
    let mut mod_content = fs::read_to_string(&mod_file).unwrap_or_default();
    if !mod_content.contains(&mod_line) {
        mod_content.push_str(&mod_line);
        fs::write(&mod_file, mod_content).unwrap();
    }

    // 创建测试文件
    let test_path = tests_dir.join(&test_file);
    if !test_path.exists() {
        let test_code = format!(
            "use algo_rs::problems::{}::Solution;\n\n#[test]\nfn test_case() {{\n    // TODO: add test cases\n}}\n",
            file_name.trim_end_matches(".rs")
        );
        fs::write(test_path, test_code).unwrap();
    }

    println!("✅ Created problem {} ({})", number, name);
}
