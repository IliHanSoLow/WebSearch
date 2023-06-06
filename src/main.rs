use directories::UserDirs;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{env, fs};
use urlencoding;
use webbrowser;

fn main() {
    if let Some(user_dirs) = UserDirs::new() {
        let config_dir = user_dirs.home_dir();
        // Linux:   /home/alice/
        // Windows: C:\Users\Alice\
        // macOS:   /Users/Alice/
        let config_file = File::open(config_dir.join(".websearch_searchengines.json"));
        match config_file {
            Ok(_) => (),
            Err(_err) => {
                println!(
                    "failed to read .websearch_searchengines.json creating in {}",
                    &config_dir.to_str().unwrap()
                );
                fs::create_dir_all(config_dir).unwrap();
                File::create(config_dir.join(".websearch_searchengines.json"))
                    .expect("failed to create inp");
                let mut inp = OpenOptions::new()
                    .write(true)
                    .open(config_dir.join(".websearch_searchengines.json"))
                    .unwrap();
                inp.write_all(
                    br#"
                    {"google": "https://www.google.com/search?q=",
                    "bing": "https://www.bing.com/search?q=",
                    "brave": "https://search.brave.com/search?q=",
                    "yahoo": "https://search.yahoo.com/search?p=",
                    "duckduckgo": "https://www.duckduckgo.com/?q=",
                    "startpage": "https://www.startpage.com/do/search?q=",
                    "yandex": "https://yandex.ru/yandsearch?text=",
                    "github": "https://github.com/search?q=",
                    "baidu": "https://www.baidu.com/s?wd=",
                    "ecosia": "https://www.ecosia.org/search?q=",
                    "goodreads": "https://www.goodreads.com/search?q=",
                    "qwant": "https://www.qwant.com/?q=",
                    "givero": "https://www.givero.com/search?q=",
                    "stackoverflow": "https://stackoverflow.com/search?q=",
                    "wolframalpha": "https://www.wolframalpha.com/input/?i=",
                    "archive": "https://web.archive.org/web/*/",
                    "scholar": "https://scholar.google.com/scholar?q=",
                    "ask": "https://www.ask.com/web?q="}"#,
                )
                .expect("couldnt write to inp");
            }
        }
        let args: Vec<String> = env::args().collect();
        let engine = &args[1];
        let query = &args[2..].join(" ");
        web_search(&engine, &query).expect("This didnt work");
    }
}

fn web_search(engine: &str, query: &str) -> Result<(), String> {
    // Define search engine URLs

    let user_dirs = UserDirs::new().unwrap();
    let config_dir = user_dirs.home_dir();
    let file = fs::read_to_string(config_dir.join(".websearch_searchengines.json"))
        .expect("config file not found");

    let mut urls: HashMap<String, String> = HashMap::new();
    let json = serde_json::from_str(&file).expect("Failed to parse JSON");
    urls = serde_json::from_value(json).expect("Failed to convert to HashMap");

    // urls.insert("google", "https://www.google.com/search?q=");
    // urls.insert("bing", "https://www.bing.com/search?q=");
    // urls.insert("brave", "https://search.brave.com/search?q=");
    // urls.insert("yahoo", "https://search.yahoo.com/search?p=");
    // urls.insert("duckduckgo", "https://www.duckduckgo.com/?q=");
    // urls.insert("startpage", "https://www.startpage.com/do/search?q=");
    // urls.insert("yandex", "https://yandex.ru/yandsearch?text=");
    // urls.insert("github", "https://github.com/search?q=");
    // urls.insert("baidu", "https://www.baidu.com/s?wd=");
    // urls.insert("ecosia", "https://www.ecosia.org/search?q=");
    // urls.insert("goodreads", "https://www.goodreads.com/search?q=");
    // urls.insert("qwant", "https://www.qwant.com/?q=");
    // urls.insert("givero", "https://www.givero.com/search?q=");
    // urls.insert("stackoverflow", "https://stackoverflow.com/search?q=");
    // urls.insert("wolframalpha", "https://www.wolframalpha.com/input/?i=");
    // urls.insert("archive", "https://web.archive.org/web/*/");
    // urls.insert("scholar", "https://scholar.google.com/scholar?q=");
    // urls.insert("ask", "https://www.ask.com/web?q=");

    // let mut inp = inp::open("config.json")
    // .expect("config.json not found. Need a config.json in the same folder as the executable.");
    // let mut data = String::new();
    // inp.read_to_string(&mut data).unwrap();
    // let urls: HashMap<&str, &str> = serde_json::from_str(&data).unwrap();

    // Check whether the search engine is supported
    let url = urls.get(engine).ok_or(format!(
        "Search engine '{}' not supported. Add it tp your .websearch_searchengines.json",
        engine
    ))?;

    // Build search URL
    let search_url = format!("{}{}", url, urlencoding::encode(query));

    // Open the URL in the default browser
    let _ = webbrowser::open(&search_url);

    Ok(())
}
