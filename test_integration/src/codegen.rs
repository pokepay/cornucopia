use crate::{
    fixtures::{CodegenTest, TestSuite},
    utils::{reset_db, rustfmt_dir},
};

use cornucopia::{CodegenSettings, Error};
use owo_colors::OwoColorize;
use std::{env::set_current_dir, process::Command};

// Run codegen test, return true if all test are successful
pub(crate) fn run_codegen_test(
    client: &mut postgres::Client,
    apply: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut successful = true;
    let original_pwd = std::env::current_dir()?;
    let fixture_path = "fixtures/codegen";

    let test_suites = TestSuite::<CodegenTest>::read(fixture_path);
    for suite in test_suites {
        println!("{}", format!("[codegen] {}", suite.name).magenta());
        for test in suite.tests {
            // Reset DB
            reset_db(client)?;

            // Set current dir to test base path
            set_current_dir(format!("../{}", test.base_path))?;

            // Load schema
            cornucopia::load_schema(client, &["schema.sql"])?;

            // If `--apply`, then the code will be regenerated.
            // Otherwise, it is only checked.
            if apply {
                // Generate
                cornucopia::generate_live(
                    client,
                    &test.queries_path,
                    Some(&test.destination),
                    CodegenSettings::from(&test),
                )
                .map_err(Error::report)?;
                // Format the generated dir
                rustfmt_dir(&test.destination);
            } else {
                let temp = std::env::temp_dir().join(&test.destination);

                cornucopia::generate_live(
                    client,
                    &test.queries_path,
                    Some(&temp),
                    CodegenSettings::from(&test),
                )
                .map_err(Error::report)?;

                // Format the generated dir
                rustfmt_dir(&temp);

                let result =
                    folder_compare::FolderCompare::new(&test.destination, &temp, &Vec::new())
                        .unwrap();

                let changed_files = result.changed_files;
                let new_files = result.new_files;

                if !changed_files.is_empty() {
                    Err(format!(
                        "\"{}\" is changed",
                        changed_files[0].to_str().unwrap()
                    ))?;
                }

                if !new_files.is_empty() {
                    Err(format!("\"{}\" is new", new_files[0].to_str().unwrap()))?;
                }
            }
            println!("(generate) {} {}", test.name, "OK".green());

            if test.run {
                // Change current directory
                std::env::set_current_dir(&original_pwd)?;
                std::env::set_current_dir(&format!("../{}", test.base_path))?;
                // Run
                let result = Command::new("cargo").arg("run").output()?;
                if result.status.success() {
                    println!("(run) {} {}", test.name, "OK".green());
                } else {
                    successful = false;
                    println!(
                        " {}\n{}",
                        "ERR".red(),
                        String::from_utf8_lossy(&result.stderr)
                            .as_ref()
                            .bright_black()
                    );
                }
            }

            // Move back to original directory
            std::env::set_current_dir(&original_pwd)?;
        }
    }

    Ok(successful)
}
