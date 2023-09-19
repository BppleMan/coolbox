use crate::color_extension::RGB;
use iced::widget::{button, canvas, checkbox, column, container, text, Container, Text};
use iced::{
    executor, font, Application, Background, BorderRadius, Color, Command, Element, Font, Length,
    Theme,
};

use crate::cool::{cool_card, Cool};
use crate::widget;
use crate::widget::ClipContainer;

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

        // let cool_card: Element<_, _> = cool_card(
        //     Cool::new("zsh", "5.10.0", "A shell"),
        //     None,
        //     None,
        //     None,
        //     None,
        // )
        // .into();

        let cool_card = cool_card(
            Cool::new("zsh", "5.10.0", "A shell"),
            None,
            None,
            None,
            None,
        );

        // let clip: widget::ClipContainer<'_, AppMessage, iced::Renderer> = ClipContainer::new(
        //     container("Test")
        //         .width(Length::from(200))
        //         .height(Length::from(200))
        //         .style(iced::theme::Container::from(|_: &'_ iced::Theme| {
        //             iced::widget::container::Appearance {
        //                 text_color: None,
        //                 background: Some(Background::from(0xff0000ff.to_rgba())),
        //                 // background: None,
        //                 border_radius: Default::default(),
        //                 border_width: 0.0,
        //                 border_color: Default::default(),
        //             }
        //         }))
        //         .center_x()
        //         .center_y(),
        // );
        //
        // let temp: iced::widget::Container<'_, AppMessage, iced::Renderer> = container("Test")
        //     .width(Length::from(200))
        //     .height(Length::from(200))
        //     .style(iced::theme::Container::from(|_: &'_ iced::Theme| {
        //         iced::widget::container::Appearance {
        //             text_color: None,
        //             background: Some(Background::from(0x00ff00ff.to_rgba())),
        //             // background: None,
        //             border_radius: Default::default(),
        //             border_width: 0.0,
        //             border_color: Default::default(),
        //         }
        //     }))
        //     .center_x()
        //     .center_y();
        //
        // let progress_bar = container("Test")
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .style(iced::theme::Container::from(|_: &'_ iced::Theme| {
        //         iced::widget::container::Appearance {
        //             text_color: None,
        //             background: Some(Background::from(0x0000ffff.to_rgba())),
        //             // background: None,
        //             border_radius: Default::default(),
        //             border_width: 0.0,
        //             border_color: Default::default(),
        //         }
        //     }));
        let content = column![default_checkbox, custom_checkbox, cool_card].spacing(22);

        // let content = container(content)
        //     .width(420)
        //     .height(200)
        //     .align_x(iced::alignment::Horizontal::Center)
        //     .center_x()
        //     .center_y()
        //     .style(iced::theme::Container::from(|_: &'_ iced::Theme| {
        //         iced::widget::container::Appearance {
        //             text_color: None,
        //             background: Some(Background::from(Color::WHITE)),
        //             // background: None,
        //             border_radius: BorderRadius::from(12.0),
        //             border_width: 0.0,
        //             border_color: Default::default(),
        //         }
        //     }));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
