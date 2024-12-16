use std::{env, path::PathBuf};

use iced::{
    alignment::Horizontal,
    widget::{button, center, column, container, scrollable, text},
    Size,
};
use iced::{Element, Font};
use itertools::Itertools;

const ICON_FONT: Font = Font::with_name("icons");

pub fn main() -> iced::Result {
    iced::application("Checkbox - Iced", State::update, State::view)
        .font(include_bytes!("../ignore/fonts/icons.ttf").as_slice())
        .window_size(Size::<f32> {
            height: 300.0,
            width: 400.0,
        })
        .decorations(true)
        .run()
}

#[derive(Default)]
struct State {
    picked_paths: Vec<PathBuf>,
    xlsx_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
enum Message {
    PickedPaths(Vec<PathBuf>),
    PickedXlsxPath(Option<PathBuf>),
    Pass,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::PickedPaths(paths) => self.picked_paths = paths.clone(),
            Message::PickedXlsxPath(path) => self.xlsx_path = path.clone(),
            Message::Pass => (),
        }
    }

    fn view(&self) -> Element<Message> {
        let get_files_button = button("Select epub files to process")
            .on_press_with(|| Message::PickedPaths(select_epub_files()));

        let selected_files_text = text(format_paths(&self.picked_paths))
            .size(16)
            .wrapping(text::Wrapping::WordOrGlyph);

        let cont = container("This text is centered inside a rounded box!")
            .padding(10)
            // .center(800)
            .style(container::rounded_box);

        let process_button = button("Process files").on_press(Message::Pass);

        // let styled_checkbox = |label| {
        //     checkbox(label, true).on_toggle_maybe(self.default.then_some(Message::StyledToggled))
        // };

        // let checkboxes = row![
        //     styled_checkbox("Primary").style(checkbox::primary),
        //     styled_checkbox("Secondary").style(checkbox::secondary),
        //     styled_checkbox("Success").style(checkbox::success),
        //     styled_checkbox("Danger").style(checkbox::danger),
        // ]
        // .spacing(20);

        // let custom_checkbox = checkbox("Custom", self.custom)
        //     // .on_toggle(Message::CustomToggled)
        //     .icon(checkbox::Icon {
        //         font: ICON_FONT,
        //         code_point: '\u{e901}',
        //         size: None,
        //         line_height: text::LineHeight::Relative(1.0),
        //         shaping: text::Shaping::Basic,
        //     });

        let content = scrollable(
            column![get_files_button, selected_files_text, cont, process_button]
                .spacing(20)
                .align_x(Horizontal::Center),
        );

        center(content).into()
    }
}

fn select_epub_files() -> Vec<PathBuf> {
    rfd::FileDialog::new()
        // .add_filter("epub", &["epub"])
        .set_directory(env::current_dir().unwrap_or(".".into()))
        .pick_files()
        .unwrap_or(vec![])
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
