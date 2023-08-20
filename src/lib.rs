pub mod addon;
pub mod analyze;
pub mod definition;
pub mod hover;
pub mod semantic_token;

#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::Url;

    use super::*;

    #[test]
    fn bla() {
        let uri = Url::parse(
            "file:///home/azureuser/projects/arma/antistasi/Antistasi/fnc_AAFattackScore.sqf",
        )
        .unwrap();
        let (addon_path, functions) =
            if let Some((path, functions)) = addon::identify(&uri, "config.cpp") {
                (path, functions)
            } else if let Some((path, functions)) = addon::identify(&uri, "description.ext") {
                (path, functions)
            } else {
                panic!();
            };

        println!("processing");
        let (states, _) = addon::process(addon_path.clone(), &functions);

        println!("single file");
        let uri = Url::parse(
            "file:///home/azureuser/projects/arma/antistasi/Antistasi/initialization/client.sqf",
        )
        .unwrap();

        let mission = states
            .iter()
            .flat_map(|(path, (function_name, (state, _)))| state.globals(function_name.clone()))
            .collect();

        let path = uri.to_file_path().expect("utf-8 path");

        let content = std::fs::read_to_string(&path).unwrap();
        let configuration = sqf::analyzer::Configuration {
            file_path: path.into(),
            base_path: addon_path,
            ..Default::default()
        };

        let (state_semantic, errors) = match analyze::compute(&content, configuration, mission) {
            Ok((state, semantic, errors)) => (Some((state, semantic)), errors),
            Err(e) => (None, vec![e]),
        };

        let (state, _) = state_semantic.unwrap();

        println!("{:#?}", state.origins)
    }
}
