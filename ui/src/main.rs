mod handler;

use std::io::Read;

use egui::{self, Vec2};
use tokio;
use eframe::{self, NativeOptions, run_native};

use crate::handler::Event;

#[tokio::main]
async fn main() {

    let mut googlemap_data_file = std::fs::OpenOptions::new().create(true).read(true).write(true).open("./googlemap_data.txt").expect("cannot open googlemap data file");
    let mut info_scrap_file = std::fs::OpenOptions::new().create(true).read(true).write(true).open("./info_scrap.txt").expect("cannot open info scrap file");
    let mut link_file = std::fs::OpenOptions::new().create(true).read(true).write(true).open("./links.txt").expect("cannot open links file");

    let mut name_buffer = String::new();
    let _readed_name = googlemap_data_file.read_to_string(&mut name_buffer).unwrap();

    let mut info_scrap = String::new();
    let _readed_name = info_scrap_file.read_to_string(&mut info_scrap).unwrap();

    let mut links = String::new();
    let _readed_name = link_file.read_to_string(&mut links).unwrap();


    let app = Event{
        name_err: false,
        rawdata_err: false,
        links,
        googlemap_data: name_buffer,
        info_scrap,
        link_file,
        googlemap_data_file,
        info_scrap_file,
    };

    let mut native_option = NativeOptions::default();
    native_option.initial_window_size = std::option::Option::Some(Vec2 { x: 1000., y: 800. });
    native_option.resizable = false;

    run_native(Box::new(app), native_option);
}

/*

fn lines_from_file(file: File) -> Vec<String> {
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
*/
