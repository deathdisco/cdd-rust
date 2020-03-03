const FN_TEMPLATE:&str = r#"pub fn $NAME($PARAMS)$RETURN {
$BODY
}
"#;

const PARAM_TEMPLATE:&str = r#"$NAME: $TYPE"#;

pub fn function(name: &str, params: &str, rtn: &str, body: &str) -> String {
    FN_TEMPLATE
        .replace("$NAME", &name)
        .replace("$PARAMS", &params)
        .replace("$RETURN", &rtn)
        .replace("$BODY", &body)
        .to_string()
}

pub fn param(param: (String, String)) -> String {
    PARAM_TEMPLATE
        .replace("$NAME", &param.0)
        .replace("$TYPE", &param.1)
        .to_string()
}