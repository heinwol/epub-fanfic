use std::{env, path::PathBuf};

use iced::Element;
use iced::{
    alignment::Horizontal,
    color,
    widget::{button, center, column, container, scrollable, text},
    Size, Task,
};
use itertools::Itertools;
use log::info;

use crate::get_data::generate_workbook;

pub fn main() -> iced::Result {
    iced::application("Checkbox - Iced", State::update, State::view)
        .window_size(Size::<f32> {
            height: 300.0,
            width: 400.0,
        })
        .decorations(true)
        .run()
}

type GenerationResult = Result<(), String>;

#[derive(Default)]
struct State {
    picked_paths: Vec<PathBuf>,
    xlsx_path: Option<PathBuf>,
    processing: bool,
    generation_result: Option<GenerationResult>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    PickedPaths,
    PickedXlsxPath,
    Pass,
    Process,
    Generated(GenerationResult),
}

async fn gen_wb(xlsx_path: Option<PathBuf>, picked_paths: Vec<PathBuf>) -> GenerationResult {
    generate_workbook(xlsx_path.unwrap(), picked_paths.iter()).map_err(|e| e.to_string())
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PickedPaths => {
                self.picked_paths = select_epub_files();
                Task::none()
            }
            Message::PickedXlsxPath => {
                self.xlsx_path = select_xlsx_file();
                Task::none()
            }
            Message::Pass => Task::none(),
            Message::Process => {
                self.processing = true;
                info!("processing");
                Task::perform(
                    gen_wb(self.xlsx_path.clone(), self.picked_paths.clone()),
                    Message::Generated,
                )
            }
            Message::Generated(res) => {
                info!("generated!");
                self.processing = false;
                self.generation_result = Some(res.clone());
                match res {
                    Ok(()) => (),
                    Err(_) => (),
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let get_files_button =
            button(text("Select epub files to process")).on_press(Message::PickedPaths);
        let selected_files_text =
            container(scrollable(text(format_paths(&self.picked_paths)).size(12)))
                .padding(10)
                .max_height(100)
                .style(container::rounded_box);

        let get_xlsx_button =
            button(text("Select where to write result")).on_press(Message::PickedXlsxPath);

        let process_button = button(text("Process files")).on_press_maybe(
            if !self.processing && !self.picked_paths.is_empty() && self.xlsx_path.is_some() {
                Some(Message::Process)
            } else {
                None
            },
        );

        let result = {
            match self.generation_result.as_ref() {
                None => text("..."),
                Some(Ok(())) => text("Success").color(color!(0x118B50)),
                Some(Err(e)) => text!("Error encountered:\n{e}").color(color!(0xFF748B)),
            }
        };
        let content = scrollable(
            column![
                get_files_button,
                selected_files_text,
                get_xlsx_button,
                process_button,
                result
            ]
            .spacing(20)
            .align_x(Horizontal::Center),
        );

        center(content).into()
    }
}

fn select_epub_files() -> Vec<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("epub", &["epub"])
        .set_directory(env::current_dir().unwrap_or(".".into()))
        .pick_files()
        .unwrap_or(vec![])
}
fn select_xlsx_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("spreadsheet", &["xlsx", "xlx", "xls"])
        .set_file_name("fics_parsing_result.xlsx")
        .set_directory(env::current_dir().unwrap_or(".".into()))
        .save_file()
}

fn format_paths(v: &Vec<PathBuf>) -> String {
    v.iter()
        .map(|p| {
            p.clone()
                .into_os_string()
                .into_string()
                .unwrap_or("".into())
        })
        .join("\n")
}
