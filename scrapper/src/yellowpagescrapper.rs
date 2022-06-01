use std::{thread::sleep, time::Duration, io::BufWriter};

use rand::prelude::SliceRandom;
use thirtyfour::{WebDriver, DesiredCapabilities, By};
use std::io::Write;

//const BASE_URL: &str = "https://www.yellowpages.my/services/l/food-dining/restaurants?where=Kuala%20Lumpur";

pub async fn _scrap_yellowpage() {
        //START THE WEBDRIVER
        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("start-maximized").unwrap();
        /* 
        caps.add_chrome_option(
            "prefs",
            serde_json::json!({
                "goog:chromeOptions": {
                    "excludeSwitches": [ "enable-automation" ],
                    "useAutomationExtension": false
                },
            }),
        ).unwrap();
        */
        //caps.add_chrome_arg("--disable-blink-features=AutomationControlled").unwrap();
        //caps.add_chrome_arg("window-size=1280,800").unwrap();
        //caps.add_chrome_arg("user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.169 Safari/537.36").unwrap();

        /* 
        caps.add_chrome_option(
            "prefs",
            serde_json::json!({
                "profile.default_content_settings": {
                    "images": 2
                },
                "profile.managed_default_content_settings": {
                    "images": 2
                }
            }),
        ).unwrap();
        */
        //let proxy = Proxy::AutoConfig { url: "https://110.78.147.84:8080".to_string() };
        //caps.set_proxy(proxy).unwrap();
        //let path = Path::new("./adblock.crx");
        //let _extension_result = caps.add_extension(path).unwrap();
        let driver = WebDriver::new("http://localhost:9515", &caps)
        .await
        .unwrap();
        sleep(Duration::from_secs(30));
        driver.get("yellowpage").await.unwrap();

        let search_element = driver.find_element(By::Id("search")).await.unwrap();
        let _g_element = search_element.find_elements(By::ClassName("g")).await.unwrap();

        sleep(Duration::from_secs(6));

        let max_pages = 218;
        let rawdata_file = std::fs::OpenOptions::new().read(true).write(true).append(true).open("rawdata0.txt").expect("cannot open users file");
        for _page in 0 ..= max_pages {
            //let mut allrawdata = String::new();
            let grid_elements = driver.find_elements(By::Tag("app-expended-normal-listing")).await.unwrap();
            for i in grid_elements {
                let detail_element = i.find_element(By::ClassName("details")).await.unwrap();
                
                let name_element = detail_element.find_element(By::ClassName("title")).await.unwrap();
                let name = name_element.text().await.unwrap();
                let num_element = i.find_element(By::ClassName("phone-number")).await.unwrap();
                let num = num_element.text().await.unwrap();
                if num.len() < 8 {
                    continue;
                }
                let num = num.trim(); 
                let (_remove, keep) = num.split_at(5);
                let raw_data = format!("{},{}", name.trim(), keep);
                
                //write
                println!("{}", raw_data);
                let mut writer = BufWriter::new(&rawdata_file);
                writeln!(writer, "{}", raw_data).expect("cannot write to rawdata.txt");
            }
            
    
    
            
    
            //change page
            let page_elements = driver.find_element(By::ClassName("nav")).await.unwrap();
            //page_elements.scroll_into_view().await.unwrap();
            let a_elements = page_elements.find_elements(By::Tag("a")).await.unwrap();
            match a_elements.len() {
                1 => {
                    a_elements[0].click().await.unwrap();
                },
                2 => {
                    a_elements[1].click().await.unwrap();
                }
                _ => {
                    panic!()
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