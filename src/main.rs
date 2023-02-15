use std::collections::HashMap;
use std::env;
use urlencoding;
use webbrowser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let engine = &args[1];
    let query = &args[2..].join(" ");
    web_search(&engine, &query).expect("This didnt work");
}

fn web_search(engine: &str, query: &str) -> Result<(), String> {
    // Define search engine URLs
    let mut urls: HashMap<&str, &str> = HashMap::new();
    urls.insert("google", "https://www.google.com/search?q=");
    urls.insert("bing", "https://www.bing.com/search?q=");
    urls.insert("brave", "https://search.brave.com/search?q=");
    urls.insert("yahoo", "https://search.yahoo.com/search?p=");
    urls.insert("duckduckgo", "https://www.duckduckgo.com/?q=");
    urls.insert("startpage", "https://www.startpage.com/do/search?q=");
    urls.insert("yandex", "https://yandex.ru/yandsearch?text=");
    urls.insert("github", "https://github.com/search?q=");
    urls.insert("baidu", "https://www.baidu.com/s?wd=");
    urls.insert("ecosia", "https://www.ecosia.org/search?q=");
    urls.insert("goodreads", "https://www.goodreads.com/search?q=");
    urls.insert("qwant", "https://www.qwant.com/?q=");
    urls.insert("givero", "https://www.givero.com/search?q=");
    urls.insert("stackoverflow", "https://stackoverflow.com/search?q=");
    urls.insert("wolframalpha", "https://www.wolframalpha.com/input/?i=");
    urls.insert("archive", "https://web.archive.org/web/*/");
    urls.insert("scholar", "https://scholar.google.com/scholar?q=");
    urls.insert("ask", "https://www.ask.com/web?q=");

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
