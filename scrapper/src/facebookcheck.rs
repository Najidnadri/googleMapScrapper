use std::io::{Read, BufWriter};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use rand::prelude::SliceRandom;
use thirtyfour::{DesiredCapabilities, WebDriver, By, Keys};



pub async fn facebook_check() {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("start-maximized").unwrap();
    let driver = WebDriver::new("http://localhost:9515", &caps)
    .await
    .unwrap();
    //sleep(Duration::from_secs(30));
    driver.get("https://www.google.com/search?q=chill+n+santai&oq=Chill+n+Santai&aqs=chrome.0.0i355i512j46i175i199i512j0i22i30j0i390l4.230j0j7&sourceid=chrome&ie=UTF-8").await.unwrap();
    
    let info_scrap = std::fs::OpenOptions::new().create(true).read(true).write(true).append(true).open("info_scrap.txt").expect("cannot open info_scrap file");
    let mut googlemap_data = std::fs::OpenOptions::new().create(true).read(true).write(true).append(true).open("googlemap_data.txt").expect("cannot open googlemap_data file");
    let mut buffer = String::new();
    let _readed = googlemap_data.read_to_string(&mut buffer).unwrap();
    //println!("{}", buffer);
    let vector: Vec<&str> = buffer.split("\n").collect();

    for companyname in vector {
        println!("{}", companyname);

        //filter i
        if filter_keywords(companyname.to_string()) == false {
            continue;
        }
        
        println!("after filter");
        //turn another page
        let search_element = driver.find_element(By::Tag("input")).await.unwrap();
        search_element.send_keys(Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace +Keys::Backspace).await.unwrap();
        search_element.send_keys(companyname.clone()).await.unwrap();
        search_element.send_keys(Keys::Enter).await.unwrap();

        //sleep
        sleep(Duration::from_secs(1));

        //check the right card
        let mut check_rightcard_error = " ".to_string();
        let mut phone_number = " ".to_string();
        
        match driver.find_element(By::ClassName("I6TXqe")).await {
            Ok(rightcard_element) => {
                let rightcard_html = rightcard_element.outer_html().await.unwrap();
                if !check_active(rightcard_html.clone()) {
                    //sleep
                    let mut rng = rand::thread_rng();
                    let mut nums: Vec<i32> = (3 ..= 5).collect();
                    nums.shuffle(&mut rng);
                    let n = nums[0] as u64;
                    sleep(Duration::from_secs(n));
                    continue;
                }
                //find phone number
                match rightcard_element.find_element(By::ClassName("zdqRlf")).await {
                    Ok(phone_element) => {
                        let phonenum = phone_element.text().await.unwrap();
                        phone_number = phonenum;
                    },
                    Err(_e) => {
                        println!("cannot find phone element: {}", &companyname);
                    },
                }
            },
            Err(e) => {
                println!("{:?}", e);
                check_rightcard_error = "error".to_string();
            },
        }
        

        //check facebook exist
        let mut facebook_exist = " ".to_string();
        let cite_elements = driver.find_elements(By::Tag("cite")).await.unwrap();
        for i in cite_elements {
            let text = i.text().await.unwrap();
            if text.contains("facebook") {
                facebook_exist = "exist".to_string();
                break;
            }
        }

        //write
        let companyname = companyname.trim();
        let string = format!("{}={}={}={}", companyname, phone_number, facebook_exist, check_rightcard_error);
        let mut writer = BufWriter::new(&info_scrap);
        writeln!(writer, "{}", string).expect("cannot write to info_scrap.txt");
        println!("done: {}", string);


        //sleep
        let mut rng = rand::thread_rng();
            let mut nums: Vec<i32> = (3 ..= 5).collect();
            nums.shuffle(&mut rng);
            let n = nums[0] as u64;

        sleep(Duration::from_secs(n));
    }
    

    driver.quit().await.unwrap();

}


fn check_active(html_string: String) -> bool {
    if html_string.contains("Permanently closed") {
        return false 
    } else if html_string.contains("Temporarily closed") {
        return false
    }
    true
}

fn filter_keywords(keyword: String) -> bool {
    let keyword = keyword.to_ascii_lowercase();
    
    if keyword.contains("maju") {
        return false
    } else if keyword.contains("starbucks") {
        return false
    } else if keyword.contains("nandos") {
        return false
    } else if keyword.contains("kfc") {
        return false
    } else if keyword.contains("kenny rogers") {
        return false
    } else if keyword.contains("dunkin' donuts") {
        return false
    } else if keyword.contains("bistro") {
        return false
    } else if keyword.contains("coffee bean") {
        return false
    } else if keyword.contains("tealive") {
        return false
    } else if keyword.contains("chatime") {
        return false
    } else if keyword.contains("burger king") {
        return false
    } else if keyword.contains("qsr stores") {
        return false
    } else if keyword.contains("pizza hut") {
        return false
    } else if keyword.contains("dominos") {
        return false
    } else if keyword.contains("sushi kin") {
        return false
    } else if keyword.contains("dommal") {
        return false
    } else if keyword.contains("oldtown kopitiam") {
        return false
    } else if keyword.contains("delivery") {
        return false
    } else if keyword.contains("cyber cafe") {
        return false
    } else if keyword.contains("oldtown white coffee") {
        return false
    } else if keyword.contains("sushi king") {
        return false
    } else if keyword.contains("a&w") {
        return false
    } 
    true
}