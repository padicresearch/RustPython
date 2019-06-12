use std::collections::hash_map::HashMap;

const HELLO: &str = "initialized = True
print(\"Hello world!\")
";

const IMPORTLIB_BOOTSTRAP: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../Lib/importlib/_bootstrap.py"
));
const IMPORTLIB_BOOTSTRAP_EXTERNAL: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../Lib/importlib/_bootstrap_external.py"
));

pub fn get_module_inits() -> HashMap<String, &'static str> {
    let mut modules = HashMap::new();
    modules.insert("__hello__".to_string(), HELLO);
    modules.insert("_frozen_importlib".to_string(), IMPORTLIB_BOOTSTRAP);
    modules.insert(
        "_frozen_importlib_external".to_string(),
        IMPORTLIB_BOOTSTRAP_EXTERNAL,
    );
    modules
}