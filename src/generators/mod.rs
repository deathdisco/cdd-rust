use cdd::*;

pub fn update(project: &Project, _code: &str) -> Result<String, failure::Error> {
    let requests = project.requests
        .iter()
        .map(request_to_code)
        .collect::<Vec<String>>()
        .join("\n\n");

        // crate::writers::functions::function(
        //     &request.name,
        //     &crate::writers::functions::params(request.params.clone().into_iter().map(|param| (param.name, format!("{:?}", param.variable_type))).collect()),
        //     "",
        //     "")
    // }).collect::<Vec<String>>().join("\n\n");

    let models = crate::writers::print_models(&project.models);

    let file = [models, requests].join("\n\n");

    // todo: actually update instead of generate
    // Ok(crate::writers::print_models(&project.models))
    Ok(file)
}

fn request_to_code(request: &Request) -> String {
    let params = request.params
        .iter()
        .map(param_to_code)
        .collect::<Vec<String>>()
        .join(", ");

    let body = format!(r#"    ApiBase::call("{}", "{}", vec![{}])"#,
        format!("{:?}", request.method).to_uppercase(),
        request.path,
        request.params.clone().into_iter().map(|param| {
            format!(r#"("{}", {})"#,
                param.name,
                param.name)
        }).collect::<Vec<String>>().join(", "));


    let response_type:String = request.response_type.clone()
        .map(|rt| format!(" -> ApiResult<{}>",
            crate::writers::variable_type::variable_type_to_rust_type(&rt)))
        .unwrap_or("".to_string());

    crate::writers::functions::function(
        &request.name,
        &params,
        &response_type,
        &body)
}

fn param_to_code(param: &std::boxed::Box<Variable>) -> String {
    crate::writers::functions::param(
        (param.name.clone(), crate::writers::variable_type::variable_type_to_rust_type(&param.variable_type)))
}
