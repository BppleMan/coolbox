use iced::advanced::graphics::core::Element;
use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::Tree;
use iced::advanced::{Layout, Widget};
use iced::mouse::{Cursor, Interaction};
use iced::widget::{checkbox, column, component, container, progress_bar, text, Component};
use iced::{theme, Background, BorderRadius, Color, Length, Rectangle, Renderer, Theme};
use std::ops::RangeInclusive;

use crate::color_extension::RGB;
use crate::cool::Cool;

pub type OnPressed<Message> = fn(bool) -> Message;
pub type OnPressedInstall<Message> = fn() -> Message;
pub type OnPressedUpdate<Message> = fn() -> Message;
pub type OnProgressChanged<Message> = fn(u32) -> Message;

pub enum CoolMessage {
    Pressed(bool),
    PressedInstall,
    PressedUpdate,
    ProgressChanged(u32),
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CoolCard<Message> {
    pub cool: Cool,
    pub selected: bool,
    pub dependent_selected: bool,
    pub installed: bool,
    pub progress: Option<u32>,
    pub on_pressed: Option<OnPressed<Message>>,
    pub on_pressed_install: Option<OnPressedInstall<Message>>,
    pub on_pressed_update: Option<OnPressedUpdate<Message>>,
    pub on_progress_changed: Option<OnProgressChanged<Message>>,
}

impl<Message> CoolCard<Message> {
    pub fn new(
        cool: Cool,
        on_pressed: Option<OnPressed<Message>>,
        on_pressed_install: Option<OnPressedInstall<Message>>,
        on_pressed_update: Option<OnPressedUpdate<Message>>,
        on_progress_changed: Option<OnProgressChanged<Message>>,
    ) -> Self {
        CoolCard {
            cool,
            selected: false,
            dependent_selected: false,
            installed: false,
            progress: None,
            on_pressed,
            on_pressed_install,
            on_pressed_update,
            on_progress_changed,
        }
    }
}

pub fn cool_card<Message>(
    cool: Cool,
    on_pressed: Option<OnPressed<Message>>,
    on_pressed_install: Option<OnPressedInstall<Message>>,
    on_pressed_update: Option<OnPressedUpdate<Message>>,
    on_progress_changed: Option<OnProgressChanged<Message>>,
) -> CoolCard<Message> {
    CoolCard::new(
        cool,
        on_pressed,
        on_pressed_install,
        on_pressed_update,
        on_progress_changed,
    )
}

impl<Message> Component<Message, Renderer> for CoolCard<Message> {
    type State = ();
    type Event = CoolMessage;

    fn update(&mut self, _: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            CoolMessage::Pressed(value) => self.on_pressed.map(|f| f(value)),
            CoolMessage::PressedInstall => self.on_pressed_install.map(|f| f()),
            CoolMessage::PressedUpdate => self.on_pressed_update.map(|f| f()),
            CoolMessage::ProgressChanged(value) => self.on_progress_changed.map(|f| f(value)),
        }
    }

    fn view(&self, _: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let check_box =
            checkbox::<CoolMessage, _>(&self.cool.name, self.selected, CoolMessage::Pressed);

        let default_checkbox = checkbox("Default", false, CoolMessage::Pressed);

        let column = column![check_box, default_checkbox].spacing(22);

        let rect = container(column);

        let progress = progress_bar(RangeInclusive::new(0.0, 100.0), 50.0)
            .style(theme::ProgressBar::from(|_: &'_ Theme| {
                progress_bar::Appearance {
                    background: Background::Color(0x333333ff.to_rgba()),
                    bar: Background::Color(0x50a1ffff.to_rgba()),
                    border_radius: BorderRadius::from([0.0, 0.0, 4.0, 4.0]),
                }
            }))
            .height(4);

        let content = column![rect, progress];

        container(content)
            .style(iced::theme::Container::from(|_: &'_ Theme| {
                container::Appearance {
                    text_color: None,
                    background: Some(Background::Color(0x141414ff.to_rgba())),
                    border_radius: BorderRadius::from(12.0),
                    border_color: 0x434343ff.to_rgba(),
                    border_width: 1.0,
                }
            }))
            .width(420.0)
            .into()
    }
}

impl<Message> Widget<Message, Renderer> for CoolCard<Message> {
    fn width(&self) -> Length {}

    fn height(&self) -> Length {
        todo!()
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        todo!()
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        todo!()
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> Interaction {
        todo!()
    }
}

impl<'a, Message> From<CoolCard<Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(value: CoolCard<Message>) -> Self {
        component(value)
    }
}
