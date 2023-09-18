use iced::widget::{checkbox, column, container, text};
use iced::{executor, font, Application, Command, Element, Font, Length, Theme};

use crate::cool::{cool_card, Cool};

const ICON_FONT: Font = Font::with_name("icons");

#[derive(Default)]
pub struct App {
    default_checkbox: bool,
    custom_checkbox: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
    DefaultChecked(bool),
    CustomChecked(bool),
    FontLoaded(Result<(), font::Error>),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<AppMessage>) {
        (
            Self::default(),
            font::load(include_bytes!("../fonts/icons.ttf").as_slice()).map(AppMessage::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("CoolBox")
    }

    fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        match message {
            AppMessage::DefaultChecked(value) => self.default_checkbox = value,
            AppMessage::CustomChecked(value) => self.custom_checkbox = value,
            AppMessage::FontLoaded(_) => (),
        }

        Command::none()
    }

    fn view(&self) -> Element<AppMessage> {
        let default_checkbox =
            checkbox("Default", self.default_checkbox, AppMessage::DefaultChecked);
        let custom_checkbox = checkbox("Custom", self.custom_checkbox, AppMessage::CustomChecked)
            .icon(checkbox::Icon {
                font: ICON_FONT,
                code_point: '\u{e901}',
                size: None,
                line_height: text::LineHeight::Relative(1.0),
                shaping: text::Shaping::Basic,
            });

        let cool_card: Element<_, _> = cool_card(
            Cool::new("zsh", "5.10.0", "A shell"),
            None,
            None,
            None,
            None,
        )
        .into();

        let content = column![default_checkbox, custom_checkbox, cool_card].spacing(22);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
