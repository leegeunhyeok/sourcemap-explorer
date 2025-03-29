use std::{fs::File, io::Read};

const READ_FILE_ERR_MSG: &'static str = "cannot read file contents";

pub fn read_file(path: String) -> Result<String, &'static str> {
    let mut file = File::open(path).map_err(|_| READ_FILE_ERR_MSG)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| READ_FILE_ERR_MSG)?;
    Ok(contents)
}

pub fn to_valid_sm(contents: String) -> String {
    let mut json: serde_json::Value = serde_json::from_str(&contents).expect("invalid sourcemap");

    if let Some(sm_obj) = json.as_object_mut() {
        if !sm_obj.contains_key("names") {
            sm_obj.insert("names".to_string(), serde_json::Value::Array(vec![]));
        }
    }

    json.to_string()
}
