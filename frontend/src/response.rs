use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde()]
pub struct SpecificResponse {
    pub compile: String,
    pub run: String,
}
#[derive(Serialize, Deserialize)]
#[serde()]
pub struct Res {
    pub errors: SpecificResponse,
    pub outputs: SpecificResponse,
}

impl Res {
    pub fn to_message(&self) -> String {
        let mut output = String::from("Result:\n");
        if !self.outputs.run.is_empty() {
            output = format!("{}{}", output, self.outputs.run);
        }
        if !self.errors.compile.is_empty() {
            return format!("Compilation message:\n{}\n{output}", self.errors.compile);
        }
        if !self.errors.run.is_empty() {
            return format!(
                "Partial result:\n{}\nRuntime error:\n{}",
                self.outputs.run, self.errors.run
            );
        }
        output
    }
}
