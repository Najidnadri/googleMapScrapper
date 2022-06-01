use std::fs::File;
use std::{thread::sleep, time::Duration, io::BufWriter};
use std::io::{Write, BufReader, BufRead};
use rand::prelude::SliceRandom;
use thirtyfour::{DesiredCapabilities, WebDriver, By, Capabilities};

use crate::facebookcheck::facebook_check;



pub async fn google_map_scrapper() {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("start-maximized").unwrap();
    caps.set_element_scroll_behaviour(thirtyfour::ScrollBehaviour::Bottom).unwrap();
    let driver = WebDriver::new("http://localhost:9515", &caps)
    .await
    .unwrap();
    let link_file = std::fs::OpenOptions::new().read(true).open("links.txt").expect("cannot open info link file");
    let links = lines_from_file(link_file);
    let googlemap_data = std::fs::OpenOptions::new().read(true).write(true).append(true).open("googlemap_data.txt").expect("cannot open users file");
    let mut buffer: Vec<String> = Vec::new();

    for link in links {
        driver.get(link).await.unwrap();


        loop {
            //check buffer
            if buffer.len() > 100 {
                //write
                let mut buffer_index = 0;
                let mut writer = BufWriter::new(&googlemap_data);
                while buffer_index < buffer.len() {
                    let data = buffer[buffer_index].clone();
                    writeln!(writer, "{}", data).expect("cannot write to rawdata.txt");
                    writer.flush().unwrap();
                    println!("done: {}", data);
                    buffer_index += 1;
                }
                buffer.clear();
            }

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
            let mut card_index = 3;
            //scroll max down
            for _i in 1 ..= 100 {
                let card_elements_len = card_elements.len();
                if card_index >= card_elements_len {
                    break;
                }
                let selected_card = &card_elements[card_index];
                selected_card.scroll_into_view().await.unwrap();
                
                let latest_cards = driver.find_elements(By::Tag("a")).await.unwrap();
                card_elements = latest_cards;
                card_index += 1;
                sleep(Duration::from_millis(100));
            }
            for i in card_elements.clone() {
                println!("{}", i.outer_html().await.unwrap());
            }
            sleep(Duration::from_secs(1));
            for i in card_elements.clone() {
                let html = i.outer_html().await.unwrap();
                if html.contains("aria-label") {
                    if html.contains("jsaction") {
                        println!("{}", html);
                        let name = filter_name(html).await;
                        println!("{}", name);
                        //push to buffer
                        buffer.push(name);
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

    //write
    let mut buffer_index = 0;
    let mut writer = BufWriter::new(&googlemap_data);
    while buffer_index < buffer.len() {
        let data = buffer[buffer_index].clone();
        writeln!(writer, "{}", data).expect("cannot write to rawdata.txt");
        writer.flush().unwrap();
        println!("done: {}", data);
        buffer_index += 1;
    }
    buffer.clear();
    

    driver.quit().await.unwrap();
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

fn lines_from_file(file: File) -> Vec<String> {
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}