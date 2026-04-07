use std::{
  fs::{OpenOptions, read_to_string, write},
  io::Write,
  path::PathBuf,
  process::exit,
};

use reqwest::blocking::get;

const PROBLEM_URL_PREFIX: &str = "https://open.kattis.com/problems/";

/// Get path for problem name.
fn path(name: &str) -> PathBuf {
  let path_str = format!("src/{name}.rs");
  PathBuf::from(path_str)
}

/// Get URL for problem name.
fn url(name: &str) -> String {
  format!("{PROBLEM_URL_PREFIX}{name}")
}

/// Fetch problem statement from URL and return it as a string.
fn fetch(url: &str) -> String {
  get(url)
    .unwrap_or_else(|_| panic!("Failed to fetch '{url}'"))
    .text()
    .expect("Failed to read response text")
}

/// Parse the kattis page into a vec of testscases (input-output pairs).
fn parse(html: &str) -> Vec<(String, String)> {
  let mut testcases = Vec::new();
  let css_selector = "table.sample > tbody > tr > td";
  let document = scraper::Html::parse_document(html);
  // NOTE: the selector _should_ give us <td> elements in following order:
  //       input1, output1, input2, output2, ...
  //       so we can iterate with step_by(2) to get pairs of input and output
  let inouts = document
    .select(&scraper::Selector::parse(css_selector).expect("Failed to parse CSS selector"))
    .collect::<Vec<_>>();

  for i in (0..inouts.len()).step_by(2) {
    // the <td> has extraneous newline at start and end
    let input = inouts[i].text().collect::<String>().trim().to_string();
    let output = inouts[i + 1].text().collect::<String>().trim().to_string();

    testcases.push((input, output));
  }

  testcases
}

// NOTE: We don't format this function, because the push_str's are somewhat readable and aligned; rustfmt doesn't enjoy that.
#[rustfmt::skip]
/// Converts "testcases" to a mod tests string for a rust file.
fn to_tests_str(testcases: Vec<(String, String)>) -> String {
  let mut mod_tests = String::new();
  let spacing = "  ";

  mod_tests.push_str("#[cfg(test)]\n");
  mod_tests.push_str("mod tests {\n\n");
  mod_tests.push_str(&format!("{spacing}use super::*;\n\n"));
  for (i, (input, output)) in testcases.into_iter().enumerate() {
    let i = i + 1; // to make it 1-indexed
    mod_tests.push_str(&format!("{spacing}#[test]\n"));
    mod_tests.push_str(&format!("{spacing}fn sample_input_{i}() {{\n")); // open fn
    mod_tests.push_str(&format!("{spacing}{spacing}let input_str = r#\"{input}\"#.to_string();\n"));
    mod_tests.push_str(&format!("{spacing}{spacing}let expected_output = r#\"{output}\"#.to_string();\n"));
    mod_tests.push_str(&format!("{spacing}{spacing}let input = Scanner::new(std::io::Cursor::new(input_str));\n"));
    mod_tests.push_str(&format!("{spacing}{spacing}let mut output = Vec::new();\n"));
    mod_tests.push_str(&format!("{spacing}{spacing}solve(input, &mut output);\n"));
    mod_tests.push_str(&format!("{spacing}{spacing}let output_str = String::from_utf8(output).expect(\"Output is not valid UTF-8\");\n"));
    mod_tests.push_str(&format!("{spacing}{spacing}assert_eq!(output_str.trim(), expected_output.trim(), \"sample input {i}\");\n"));
    mod_tests.push_str(&format!("{spacing}}}\n")); // close fn
    mod_tests.push_str(&"\n");
  }
  mod_tests.push_str("}\n"); // close mod tests
  mod_tests
}

/// Overwrites the .cargo/config.toml file to work on `name`.
fn modify_dot_config(name: &str) {
  let config_path = format!("{}/.cargo/config.toml", env!("CARGO_MANIFEST_DIR"));
  let wanted_content = format!("[alias]\nt = \"test --bin {name}\"");
  write(config_path, wanted_content).expect("Failed to write to config.toml");
}

/// Adds a new [[bin]] entry to Cargo.toml for the given problem name, if it doesn't exist yet.
fn update_cargo_toml(name: &str) {
  let cargo_toml_path = format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"));
  let bin_entry = format!("[[bin]]\nname = \"{name}\"\npath = \"src/{name}.rs\"");
  let cargo_toml_content = read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
  if !cargo_toml_content.contains(&bin_entry) {
    OpenOptions::new()
      .append(true)
      .open(cargo_toml_path)
      .expect("Failed to open Cargo.toml")
      .write_all(bin_entry.as_bytes())
      .expect("Failed to write to Cargo.toml");
  }
}

fn main() {
  // get argument from command line
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: cargo run -- <problem_number_or_url>");
    exit(1);
  }

  // get problem-name from arg
  let arg = &args[1];
  let name = if arg.starts_with("http") {
    arg.trim_start_matches(PROBLEM_URL_PREFIX)
  } else {
    arg
  };

  // check if we already have a file
  let path = path(name);
  if path.exists() {
    modify_dot_config(name);
    exit(0);
  }

  // file doesn't exist
  let testcases = parse(&fetch(&url(name))); // scrape it
  let template_str = include_str!("../template.rs"); // get template
  let tests_str = to_tests_str(testcases); // convert testcases to a mod tests string
  let content = format!("{template_str}\n{tests_str}"); // combine template and tests for content of file
  write(path, content).expect("Failed to write problem file"); // write file
  update_cargo_toml(name); // add bin entry to Cargo.toml
  modify_dot_config(name); // modify .cargo/config.toml to work on the new problem
}

#[cfg(test)]
mod tests {

  use super::*;

  const HTML: &'static str = r#"<html><body>
    <table class="sample" summary="sample data">
      <tbody><tr>
        <th>Sample Input 1</th>

        <th>Sample Output 1</th>
      </tr>

      <tr>
        <td>
          <pre>1 5 3
ABC
</pre>
        </td>

        <td>
          <pre>1 3 5
</pre>
        </td>
      </tr>
    </tbody></table>

    <table class="sample" summary="sample data">
      <tbody><tr>
        <th>Sample Input 2</th>

        <th>Sample Output 2</th>
      </tr>

      <tr>
        <td>
          <pre>6 4 2
CAB
</pre>
        </td>

        <td>
          <pre>6 2 4
</pre>
        </td>
      </tr>
    </tbody></table>
    </body></html>"#;

  #[test]
  fn test_tests_str() {
    let testcases = vec![
      ("input1".to_string(), "output1".to_string()),
      ("input2".to_string(), "output2".to_string()),
      (
        "in with \"quotes\"".to_string(),
        "out with \"quotes\"".to_string(),
      ),
      ("a\nb\nc".to_string(), "x\ny\nz".to_string()),
    ];
    let tests_str = to_tests_str(testcases);
    let expected = r##"
#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn sample_input_1() {
    let input_str = r#"input1"#.to_string();
    let expected_output = r#"output1"#.to_string();
    let input = Scanner::new(std::io::Cursor::new(input_str));
    let mut output = Vec::new();
    solve(input, &mut output);
    let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), expected_output.trim(), "sample input 1");
  }

  #[test]
  fn sample_input_2() {
    let input_str = r#"input2"#.to_string();
    let expected_output = r#"output2"#.to_string();
    let input = Scanner::new(std::io::Cursor::new(input_str));
    let mut output = Vec::new();
    solve(input, &mut output);
    let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), expected_output.trim(), "sample input 2");
  }

  #[test]
  fn sample_input_3() {
    let input_str = r#"in with "quotes""#.to_string();
    let expected_output = r#"out with "quotes""#.to_string();
    let input = Scanner::new(std::io::Cursor::new(input_str));
    let mut output = Vec::new();
    solve(input, &mut output);
    let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), expected_output.trim(), "sample input 3");
  }

  #[test]
  fn sample_input_4() {
    let input_str = r#"a
b
c"#.to_string();
    let expected_output = r#"x
y
z"#.to_string();
    let input = Scanner::new(std::io::Cursor::new(input_str));
    let mut output = Vec::new();
    solve(input, &mut output);
    let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), expected_output.trim(), "sample input 4");
  }

}
        "##;

    assert_eq!(tests_str.trim(), expected.trim());
  }

  #[test]
  fn test_parse() {
    let parsed = parse(HTML);
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].0.trim(), "1 5 3\nABC\n".trim());
    assert_eq!(parsed[0].1.trim(), "1 3 5\n".trim());
    assert_eq!(parsed[1].0.trim(), "6 4 2\nCAB\n".trim());
    assert_eq!(parsed[1].1.trim(), "6 2 4\n".trim());
  }
}
