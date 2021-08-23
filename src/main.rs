use std::collections::HashMap;

use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};


fn main()  {
   match rank_all() {
       Ok(_) => {},
       Err(e) => println!("{}",&e)
   }
}

#[tokio::main]
async fn rank_all() -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let resp = client
        .get("https://www.bilibili.com/v/popular/rank/all")
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36")
        .send()
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&resp);
    let body_selector = Selector::parse("body").unwrap();
    let app_selector = Selector::parse(r#"div[id="app"]"#).unwrap();
    let rank_container_selector = Selector::parse(r#"div[class="rank-container"]"#).unwrap();
    let rank_list_warp_selector = Selector::parse(r#"div[class="rank-list-wrap"]"#).unwrap();
    let rank_list_selector = Selector::parse(r#"ul[class="rank-list"]"#).unwrap();
    let li_list_selector = Selector::parse("li").unwrap();
    let content_selector = Selector::parse(r#"div[class="content"]"#).unwrap();
    let info_selector = Selector::parse(r#"div[class="info"]"#).unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let mut href_map = HashMap::new();
    let mut rank_map = HashMap::new();
    let mut title_map = HashMap::new();

    let body = document.select(&body_selector).next().unwrap();
    let app = body.select(&app_selector).next().unwrap();
    let rank_container = app.select(&rank_container_selector).next().unwrap();
    let rank_list_warp = rank_container
        .select(&rank_list_warp_selector)
        .next()
        .unwrap();
    let rank_list = rank_list_warp.select(&rank_list_selector).next().unwrap();
    for li in rank_list.select(&li_list_selector) {
        let contant = li.select(&content_selector).next().unwrap();
        let info = contant.select(&info_selector).next().unwrap();
        let a = info.select(&a_selector).next().unwrap();
        let bv= a.value().attr("href").unwrap().split("/").find(|path| path.starts_with("BV")).unwrap();
        title_map.insert(bv.clone(), a.inner_html());
        rank_map.insert(bv.clone(), li.value().attr("data-rank").unwrap());
        href_map.insert(bv.clone(), a.value().attr("href").unwrap());
    }

    Ok(())
}