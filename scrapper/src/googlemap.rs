use std::{thread::sleep, time::Duration, io::BufWriter};
use std::io::Write;
use rand::prelude::SliceRandom;
use thirtyfour::error::WebDriverError;
use thirtyfour::{DesiredCapabilities, WebDriver, By, Capabilities};

use crate::facebookcheck::facebook_check;



pub async fn google_map_scrapper() {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("start-maximized").unwrap();
    caps.set_element_scroll_behaviour(thirtyfour::ScrollBehaviour::Bottom).unwrap();
    let driver = WebDriver::new("http://localhost:9515", &caps)
    .await
    .unwrap();
    let url_collection = vec![
        "https://www.google.com/maps/search/komputer+shah+alam/@3.0077858,101.5412183,11z/data=!3m1!4b1",
        "https://www.google.com/maps/search/komputer+subang+jaya/@3.0077926,101.6112646,12z/data=!3m1!4b1"
    ];

    for i in url_collection {
        //sleep(Duration::from_secs(30));
        driver.get(i).await.unwrap();
        let rawdata_file3 = std::fs::OpenOptions::new().read(true).write(true).append(true).open("rawdata3.txt").expect("cannot open users file");


        loop {

            //scrapping
            let card_elements_result = driver.find_elements(By::Tag("a")).await;

            match card_elements_result {
                Ok(_) => {
                    ()
                },
                Err(_e) => {
                    break;
                }
            }

            let mut card_elements = card_elements_result.unwrap();
            //scroll max down
            for _i in 1 ..= 5 {
                let numbers = card_elements.len();
                let last_card = &card_elements[numbers - 1];
                last_card.scroll_into_view().await.unwrap();
                //sleep(Duration::from_secs(2));
                let latest_cards = driver.find_elements(By::Tag("a")).await.unwrap();
                card_elements = latest_cards;
            }
            sleep(Duration::from_secs(1));
            for i in card_elements.clone() {
                let html = i.outer_html().await.unwrap();
                if html.contains("aria-label") {
                    if html.contains("jsaction") {
                        println!("{}", html);
                        let name = filter_name(html).await;
                        println!("{}", name);
                        //write
                        let mut writer = BufWriter::new(&rawdata_file3);
                        writeln!(writer, "{}", name).expect("cannot write to rawdata.txt");
                        println!("done: {}", name);
                    }
                }
            }

            //nextpage
            let next_button = driver.find_element(By::Id("ppdPk-Ej1Yeb-LgbsSe-tJiF1e")).await.unwrap();
            let next_button_html = next_button.outer_html().await.unwrap();
            if next_button_html.contains("disabled=") {
                break;
            }
            match next_button.click().await {
                Ok(_) => {
                    ()
                },
                Err(_e) => {
                    break;
                }
            }

            //sleep
            let mut rng = rand::thread_rng();
            let mut nums: Vec<i32> = (3 ..= 5).collect();
            nums.shuffle(&mut rng);
            let n = nums[0] as u64;
            sleep(Duration::from_secs(n));
        }
    }
    
    facebook_check().await;

}

async fn filter_name(html: String) -> String {
    let string: Vec<&str> = html.split('"').collect();
    string[1].to_string()
    /* 
    let mut n: usize = 0;
    let mut quote_location = vec![];
    html.ceil_char_boundary(index)
    for i in html.chars() {
        if i == '"' {
            quote_location.push(n);
        }
        n += 1;
    }
    let mut initial = html.find("aria-label").unwrap();
    initial += 11;
    let index = quote_location.iter().position(|&r| r == initial).unwrap();
    let ending = quote_location[index + 1];

    let result: String = html.clone().drain(initial + 1 .. ending).collect();
    result
    */
}