use std::fs;

use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, MultiSelect,
};
use reqwest::{Error, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let options: Vec<String> = get_list().await?;
    let formatter: MultiOptionFormatter<'_, String> =
        &|a| format!("- {} models selected (ᵔ ᵕ ᵔ)", a.len());

    let validator = |a: &[ListOption<&String>]| {
        if a.len() < 1 {
            return Ok(Validation::Invalid(
                "SELECT AT LEAST ONE MODEL ( ｡ •̀ ᴖ •́ ｡)".into(),
            ));
        }
        return Ok(Validation::Valid);
    };

    let ans = match MultiSelect::new("Select at least one model", options)
        .with_formatter(formatter)
        .with_validator(validator)
        .prompt()
    {
        Ok(answer) => answer,
        Err(_) => return Ok(()),
    };

    let file: String = get_gitignore(&ans).await?;
    write_file(&file);

    Ok(())
}

async fn get_list() -> Result<Vec<String>, Error> {
    let request: Response =
        reqwest::get("https://www.toptal.com/developers/gitignore/api/list").await?;
    let options_string: String = request.text().await?.replace("\n", ",");
    let options: Vec<String> = options_string.split(",").map(|s| s.to_string()).collect();
    Ok(options)
}

async fn get_gitignore(options: &Vec<String>) -> Result<String, Error> {
    let query = options.join(",");
    let url = format!("https://www.toptal.com/developers/gitignore/api/{query}");
    let response: String = reqwest::get(url).await?.text().await?;
    Ok(response)
}

fn write_file(data: &String) {
    match fs::write(".gitignore", data) {
        Ok(value) => value,
        Err(_) => {
            return;
        }
    };
    println!(".gitignore file generated. ⸜(｡˃ ᵕ ˂ )⸝♡")
}
