use std::process::{Command, Output};

use serde_json::json;
use warp::Rejection;

use crate::Code;
fn stdout_from_out(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

pub async fn code_handler(body: Code) -> Result<impl warp::Reply, Rejection> {
    let s = format!(
        "docker run -v $(pwd):/home --rm rust sh -c 'echo \"{}\" | cat > /home/a.in; rustc /home/a.in > /home/compile.txt 2> /home/compile_err.txt; timeout 10 ./a > /home/run.txt 2> /home/run_err.txt; status=$?; if [ $status -eq 127 ]; then echo \"\" | cat > /home/run_err.txt; fi; if [ $status -eq 124 ]; then echo \"Timed out\" | cat > /home/run_err.txt; fi;'",
        &body.code.replace('\"', "\\\"")
    );
    let _out = Command::new("sh")
        .arg("-c")
        .arg(s)
        .output()
        .expect("failed");
    let out_compile = Command::new("sh")
        .arg("-c")
        .arg("cat compile.txt")
        .output()
        .expect("failed");
    let err_compile = Command::new("sh")
        .arg("-c")
        .arg("cat compile_err.txt")
        .output()
        .expect("failed");
    let out_run = Command::new("sh")
        .arg("-c")
        .arg("cat run.txt")
        .output()
        .expect("failed");
    let err_run = Command::new("sh")
        .arg("-c")
        .arg("cat run_err.txt")
        .output()
        .expect("failed");
    let errors = json!({
        "compile": stdout_from_out(&err_compile),
        "run": stdout_from_out(&err_run),
    });
    let outputs = json!({
        "compile": stdout_from_out(&out_compile),
        "run": stdout_from_out(&out_run),
    });
    let res = json!({
        "outputs": outputs,
        "errors": errors
    });
    Ok(warp::reply::with_header(
        res.to_string(),
        "Access-Control-Allow-Origin",
        "*",
    ))
}
