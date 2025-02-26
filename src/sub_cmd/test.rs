use crate::metadata;
use crate::wes;

use anyhow::{anyhow, bail, Result};
use log::{debug, info};
use std::env::current_dir;
use std::fs;
use std::io::{BufWriter, Write};
use std::thread;
use std::time;
use url::Url;
use uuid::Uuid;

pub fn test(
    meta: &metadata::types::Metadata,
    wes_loc: &Url,
    write_log: bool,
    fetch_ro_crate: bool,
) -> Result<()> {
    let mut test_results = vec![];
    for test_case in &meta.workflow.testing {
        info!("Testing test case: {}", test_case.id);

        let form = wes::api::test_case_to_form(meta, test_case)?;
        debug!("Form:\n{:#?}", &form);
        let run_id = wes::api::post_run(wes_loc, form)?;
        info!("WES run_id: {}", run_id);

        let mut status = wes::api::RunStatus::Running;
        let mut iter_num = 0;
        while status == wes::api::RunStatus::Running {
            sleep(iter_num);
            status = wes::api::get_run_status(wes_loc, &run_id)?;
            debug!("WES run status: {:?}", status);
            iter_num += 1;
        }

        let run_log = serde_json::to_string_pretty(&wes::api::get_run_log(wes_loc, &run_id)?)?;
        if write_log {
            write_test_log(&meta.id, &meta.version, &test_case.id, &run_log)?;
        }
        match status {
            wes::api::RunStatus::Complete => {
                info!("Complete test case: {}", test_case.id);
                debug!("Run log:\n{}", run_log);
            }
            wes::api::RunStatus::Failed => {
                info!(
                    "Failed test case: {} with run_log:\n{}",
                    test_case.id, run_log
                );
            }
            _ => {
                unreachable!("WES run status: {:?}", status);
            }
        }

        match wes::api::fetch_ro_crate(wes_loc, &run_id) {
            Ok(ro_crate) => {
                if fetch_ro_crate || write_log {
                    let ro_crate_dir = current_dir()?.join("test-logs");
                    fs::create_dir_all(&ro_crate_dir)?;
                    let ro_crate_path = ro_crate_dir.join(format!(
                        "ro-crate-metadata_{}_{}_{}.json",
                        &meta.id, &meta.version, &test_case.id
                    ));
                    let mut file = BufWriter::new(fs::File::create(&ro_crate_path)?);
                    file.write_all(serde_json::to_string_pretty(&ro_crate)?.as_bytes())?;
                }
            }
            Err(e) => {
                if fetch_ro_crate {
                    bail!("Failed to fetch RO-Crate with error: {}", e)
                }
            }
        };

        test_results.push(TestResult {
            id: test_case.id.clone(),
            status,
        });
    }
    match check_test_results(test_results) {
        Ok(()) => info!(
            "Passed all test cases in workflow_id: {}, version: {}",
            meta.id, meta.version
        ),
        Err(e) => bail!(e),
    };
    Ok(())
}

struct TestResult {
    pub id: String,
    pub status: wes::api::RunStatus,
}

fn write_test_log(
    id: &Uuid,
    version: impl AsRef<str>,
    test_id: impl AsRef<str>,
    run_log: impl AsRef<str>,
) -> Result<()> {
    let test_log_file = current_dir()?.join(format!(
        "test-logs/{}_{}_{}.log",
        id,
        version.as_ref(),
        test_id.as_ref()
    ));
    fs::create_dir_all(
        test_log_file
            .parent()
            .ok_or_else(|| anyhow!("Failed to create dir"))?,
    )?;
    let mut buffer = BufWriter::new(fs::File::create(&test_log_file)?);
    buffer.write_all(run_log.as_ref().as_bytes())?;
    Ok(())
}

fn check_test_results(test_results: Vec<TestResult>) -> Result<()> {
    let failed_tests = test_results
        .iter()
        .filter(|r| r.status == wes::api::RunStatus::Failed)
        .collect::<Vec<_>>();
    if !failed_tests.is_empty() {
        bail!(
            "Some tests failed. Failed tests: {}",
            failed_tests
                .iter()
                .map(|r| r.id.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(())
}

/// Up to 1 minute, every 10 seconds: 10 * 6
/// Up to 5 minutes, every 30 seconds: 10 * 6 + 30 * 8
/// Up to 60 minutes, every 1 minute: 10 * 6 + 30 * 8 + 60 * 55
/// Beyond that, every 2 minutes
fn sleep(iter_num: usize) {
    if iter_num < 6 {
        thread::sleep(time::Duration::from_secs(10));
    } else if iter_num < 15 {
        thread::sleep(time::Duration::from_secs(30));
    } else if iter_num < 69 {
        thread::sleep(time::Duration::from_secs(60));
    } else {
        thread::sleep(time::Duration::from_secs(120));
    }
}
