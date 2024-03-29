use std::fs::write;
use std::fs::File;
use std::io::Read;

use eframe::epi::App;
use egui::Color32;
use reqwest;
use tokio::task;


pub struct Event {
    pub links: String,
    pub googlemap_data: String,
    pub info_scrap: String,
    pub link_file: File,
    pub googlemap_data_file: File,
    pub info_scrap_file: File,
    pub name_err: bool,
    pub rawdata_err: bool,
}

impl App for Event {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
            ui.add_sized([100.0, 100.0],  egui::Label::new("Google Map Scrapper").underline());
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.);
            let run_button = ui.button("RUN");
            if run_button.clicked() {
                write("links.txt", &self.links).unwrap();
                write("googlemap_data.txt", &self.googlemap_data).unwrap();
                write("info_scrap.txt",&self.info_scrap).unwrap();
                task::spawn(async move {
                    let _resp = reqwest::get("http://127.0.0.1:8080/googlemap")
                    .await.unwrap();
                });

            }
            ui.add_space(10.);
            let refresh_button = ui.button("REFRESH");
            if refresh_button.clicked() {
                let mut name_buffer = String::new();
                let _readed_name = self.googlemap_data_file.read_to_string(&mut name_buffer).unwrap();
                self.googlemap_data.push_str(&name_buffer);
            
                let mut info_scrap = String::new();
                let _readed_name = self.info_scrap_file.read_to_string(&mut info_scrap).unwrap();
                self.info_scrap.push_str(&info_scrap);
            
                let mut links = String::new();
                let _readed_name = self.link_file.read_to_string(&mut links).unwrap();
                self.links.push_str(&links);
            }

            let editall_button = ui.button("EDIT ALL");
            if editall_button.clicked() {
                write("links.txt", &self.links).unwrap();
                write("googlemap_data.txt", &self.googlemap_data).unwrap();
                write("info_scrap.txt",&self.info_scrap).unwrap();
            }
            /* 
            ui.add_space(5.);
            let master_button = ui.button("Edit All");
            if master_button.clicked() {
                if check_page_number(&self.min_page) {
                    self.page_err = Some(PageNumErr::MinErr);
                } else if check_page_number(&self.max_page) {
                    self.page_err = Some(PageNumErr::MaxErr);
                } else if check_value(&self.min_page, &self.max_page) {
                    self.page_err = Some(PageNumErr::ValueErr);
                } else {
                    self.page_err = None;
                    //write to page file
                    let _writed = write("minpage.txt", &self.min_page).unwrap();

                    let _writed = write("maxpage.txt", &self.max_page).unwrap();

                    write("name.txt", &self.name_buffer).unwrap();
                    write("rawdata.txt", &self.raw_data).unwrap();
                    
                }
            }
            */

            //referesh button
            ui.add_space(5.);
            egui::ScrollArea::vertical().show(ui, |ui| {
                /* 
                //name section
                ui.label("insert name");
                if self.name_err {
                    ui.colored_label(Color32::RED, "Something wrong with your name");
                }
                ui.add_sized([500., 10.],  egui::TextEdit::singleline(&mut self.name_buffer));
                let name_button = ui.button("Edit Name Only");
                if name_button.clicked() {
                    write("name.txt", &self.name_buffer).unwrap();
                }
                ui.add_space(20.);
                */

                //link section
                ui.label("Links");
                if self.rawdata_err {
                    ui.colored_label(Color32::RED, "Something wrong with your rawdata");
                }
                ui.add_sized([900., 10.],  egui::TextEdit::multiline(&mut self.links).desired_rows(20));
                let rawdata_button = ui.button("Edit links Only");
                if rawdata_button.clicked() {
                    write("links.txt", &self.links).unwrap();
                }

                ui.add_space(20.);


                /* 
                //page section
                ui.label("links");
                ui.colored_label(Color32::GRAY, "pls insert the link and divide the links by entering");
                if self.page_err == Some(PageNumErr::MaxErr) {
                    ui.colored_label(Color32::RED, "your min number should be all digit; eg: 100000");
                }
                if self.page_err == Some(PageNumErr::MinErr) {
                    ui.colored_label(Color32::RED, " you max number should be all digit; eg: 100000");
                }
                if self.page_err == Some(PageNumErr::ValueErr) {
                    ui.colored_label(Color32::RED, "min digit is higher than max digit");
                }
                ui.horizontal(|ui| {
                    ui.add_sized([100., 10.],  egui::TextEdit::singleline(&mut self.min_page));
                    ui.label("-");
                    ui.add_sized([100., 10.],  egui::TextEdit::singleline(&mut self.max_page));
                });
                let page_button = ui.button("Edit Number ID Only");
                if page_button.clicked() {
                    if check_page_number(&self.min_page) {
                        self.page_err = Some(PageNumErr::MinErr);
                    } else if check_page_number(&self.max_page) {
                        self.page_err = Some(PageNumErr::MaxErr);
                    } else if check_value(&self.min_page, &self.max_page) {
                        self.page_err = Some(PageNumErr::ValueErr);
                    } else {
                        self.page_err = None;
                        //write to page file
                        let _writed = write("minpage.txt", &self.min_page).unwrap();

                        let _writed = write("maxpage.txt", &self.max_page).unwrap();
                        
                    }
                }
                ui.add_space(20.);
                */

                //google_map_data section
                ui.label("googlemap data");
                if self.rawdata_err {
                    ui.colored_label(Color32::RED, "Something wrong with your rawdata");
                }
                ui.add_sized([900., 10.],  egui::TextEdit::multiline(&mut self.googlemap_data).desired_rows(20));
                let rawdata_button = ui.button("Edit links Only");
                if rawdata_button.clicked() {
                    write("googlemap_data.txt", &self.googlemap_data).unwrap();
                }

                ui.add_space(20.);

                
                //infoscrap section
                ui.label("Info Scrap data");
                if self.rawdata_err {
                    ui.colored_label(Color32::RED, "Something wrong with your rawdata");
                }
                ui.add_sized([900., 10.],  egui::TextEdit::multiline(&mut self.info_scrap).desired_rows(20));
                let rawdata_button = ui.button("Edit info scrap Only");
                if rawdata_button.clicked() {
                    write("info_scrap.txt",&self.info_scrap).unwrap();
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Property Agent Scrapper"
    }
}

fn _check_page_number(page: &str) -> bool {
    match page.trim().parse::<i32>() {
        Ok(_min_page) => {
            return false
        },
        Err(_) => {
            return true
        },
    }
    
}

fn _check_value(minpage: &str, maxpage: &str) -> bool {
    let minpage = minpage.trim().parse::<i32>().unwrap();
    let maxpage = maxpage.trim().parse::<i32>().unwrap(); 

    if minpage > maxpage {
        return true
    }

    false
}

