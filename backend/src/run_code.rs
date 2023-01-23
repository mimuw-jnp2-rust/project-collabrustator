use std::process::{Command, Output};

use serde_json::json;
use warp::Rejection;

use crate::Code;
fn stdout_from_out(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}
fn stderr_from_out(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).unwrap()
}
const TIMED_OUT_STATUS: i32 = 124;
pub async fn code_handler(key: String, body: Code) -> Result<impl warp::Reply, Rejection> {
    let _move_program = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo \"{}\" | cat > {}.in",
            &body.code.replace('\"', r#"\""#),
            key
        ))
        .output()
        .unwrap();
    let compile_program = Command::new("sh")
        .arg("-c")
        .arg(format!("rustc {key}.in"))
        .output()
        .unwrap();
    let run_program = Command::new("sh")
        .arg("-c")
        .arg(format!("timeout 10 ./{key}"))
        .output()
        .unwrap();
    let errors = json!({
        "compile": stderr_from_out(&compile_program),
        "run": stderr_from_out(&run_program) + (if run_program.status.code().unwrap_or(0) == TIMED_OUT_STATUS { "\nTimed out" } else { "" }),
    });
    let outputs = json!({
        "compile": stdout_from_out(&compile_program),
        "run": stdout_from_out(&run_program),
    });
    let res = json!({
        "outputs": outputs,
        "errors": errors
    });
    let _clear = Command::new("sh")
        .arg("-c")
        .arg(format!("rm {key}.in; rm {key}"))
        .output();
    Ok(warp::reply::with_header(
        res.to_string(),
        "Access-Control-Allow-Origin",
        "*",
    ))
}
