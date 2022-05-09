use fancy_regex::Regex;
use std::env::args;
use std::io::prelude::*;
use std::path::Path;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::Read,
    ops::Deref,
};

#[tokio::main]
async fn main() {
    if args().len() == 2 {
        if args().nth(1).unwrap() == "-d" {
            download().await.unwrap();
        }
    }
    read();
}

async fn download() -> Result<(), Box<dyn std::error::Error>> {
    let zip_file =
        reqwest::get("https://codeload.github.com/discord/discord-api-docs/zip/refs/heads/main")
            .await?
            .bytes()
            .await?;

    let mut file = File::create(Path::new("ddocs.zip")).expect("failed to create file");
    file.write_all(&zip_file)?;

    Ok(())
}

fn read() {
    let mut endpoints_map = BTreeMap::new();

    let endpoint_re = Regex::new(
        r"(?m:^#{1,} .* % (GET|PUT|PATCH|POST|DELETE|HEAD|CONNECT|OPTIONS|TRACE) (.*$))",
    )
    .unwrap();

    let variable_re = Regex::new(r#"\{([\w.]+)#.*?\}"#).unwrap();

    let mut zip =
        zip::ZipArchive::new(File::open("ddocs.zip").expect("ZIP file could not be opened"))
            .unwrap();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name();
        if name.starts_with("discord-api-docs-main/docs/") && name.ends_with(".md") {
            let mut data = String::from("");
            file.read_to_string(&mut data).unwrap();

            for endp in endpoint_re.captures_iter(&data) {
                let endp = endp.unwrap();
                let method = endp.get(1).unwrap().as_str().to_owned();
                let endpoint = variable_re
                    .replace_all(endp.get(2).unwrap().as_str(), "{$1}")
                    .deref()
                    .to_owned();

                if !endpoints_map.contains_key(&endpoint) {
                    endpoints_map.insert(endpoint, BTreeSet::from([method]));
                } else {
                    endpoints_map.get_mut(&endpoint).unwrap().insert(method);
                }
            }
        }
    }

    print_as_yaml(endpoints_map)
}

fn print_as_yaml(map: BTreeMap<String, BTreeSet<String>>) {
    for (endpoint, methods) in map {
        println!("{}:", endpoint);
        for method in methods {
            println!("  - {}", method)
        }
        // empty line
        println!("")
    }
}
