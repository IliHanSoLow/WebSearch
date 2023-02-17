use directories::UserDirs;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::{env, fs};
use urlencoding;
use webbrowser;

fn main() {
    if let Some(user_dirs) = UserDirs::new() {
        let config_dir = user_dirs.home_dir();
        // Linux:   /home/alice/
        // Windows: C:\Users\Alice\
        // macOS:   /Users/Alice/
        let inp = File::open(config_dir.join(".websearch_searchengines"));
        match inp {
            Ok(_) => (),
            Err(_err) => {
                println!(
                    "failed to read inp creating in {}",
                    &config_dir.to_str().unwrap()
                );
                fs::create_dir_all(config_dir).unwrap();
                File::create(config_dir.join(".websearch_searchengines"))
                    .expect("failed to create inp");
                let mut inp = OpenOptions::new()
                    .write(true)
                    .open(config_dir.join(".websearch_searchengines"))
                    .unwrap();
                inp.write_all(
                    b"google\n
                https://www.googlw.com/search?q=\n
                bing\n
                https://www.bing.com/search?q=\n
                brave\n
                https://search.brave.com/search?q=\n
                yahoo\n
                https://search.yahoo.com/search?p=\n
                duckduckgo\n
                https://www.duckduckgo.com/?q=\n
                startpage\n
                https://www.startpage.com/do/search?q=\n
                yandex\n
                https://yandex.ru/yandsearch?text=\n
                github\n
                https://github.com/search?q=\n
                baidu\n
                https://www.baidu.com/s?wd=\n
                ecosia\n
                https://www.ecosia.org/search?q=\n
                goodreads\n
                https://www.goodreads.com/search?q=\n
                qwant\n
                https://www.qwant.com/?q=\n
                givero\n
                https://www.givero.com/search?q=\n
                stackoverflow\n
                https://stackoverflow.com/search?q=\n
                wolframalpha\n
                https://www.wolframalpha.com/input/?i=\n
                archive\n
                https://web.archive.org/web/*/\n
                scholar\n
                https://scholar.google.com/scholar?q=\n
                ask\n
                https://www.ask.com/web?q=",
                )
                .expect("couldnt write to inp");
            }
        }
    }

    let args: Vec<String> = env::args().collect();
    let engine = &args[1];
    let query = &args[2..].join(" ");
    web_search(&engine, &query).expect("This didnt work");
}

fn web_search(engine: &str, query: &str) -> Result<(), String> {
    // Define search engine URLs

    let user_dirs = UserDirs::new().unwrap();
    let config_dir = user_dirs.home_dir();
    let inp = File::open(config_dir.join(".websearch_searchengines")).unwrap();

    let mut urls: HashMap<&'static str, &'static str> = HashMap::new();
    let reader = BufReader::new(inp);
    let mut counter = 1;
    let mut last_i: Option<&str> = None;
    let x = reader.lines();
    for i in x {
        let tmp = i.unwrap().clone();
        if counter % 2 == 0 {
            let Some(last) = last_i;
            let Some(tmp) = Some(&tmp);
            urls.insert(last, tmp);
        } else {
            last_i = Some(&tmp);
        }
        counter += 1;
    }

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
    let url = urls
        .get(engine)
        .ok_or(format!("Search engine '{}' not supported.", engine))?;

    // Build search URL
    let search_url = format!("{}{}", url, urlencoding::encode(query));

    // Open the URL in the default browser
    let _ = webbrowser::open(&search_url);

    Ok(())
}
