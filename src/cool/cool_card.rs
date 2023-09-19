use std::ops::RangeInclusive;

use iced::advanced::graphics::core::Element;
use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::Tree;
use iced::advanced::{layout, Layout, Renderer, Widget};
use iced::mouse::{Cursor, Interaction};
use iced::widget::{checkbox, container, Container};
use iced::{Background, BorderRadius, Length, Rectangle, Size};

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

// #[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CoolCard<'a, Message> {
    pub cool: Cool,
    pub selected: bool,
    pub dependent_selected: bool,
    pub installed: bool,
    pub progress: Option<u32>,
    pub on_pressed: Option<OnPressed<Message>>,
    pub on_pressed_install: Option<OnPressedInstall<Message>>,
    pub on_pressed_update: Option<OnPressedUpdate<Message>>,
    pub on_progress_changed: Option<OnProgressChanged<Message>>,

    pub content: Element<'a, CoolMessage, iced::Renderer>,
}

impl<'a, Message> CoolCard<'a, Message> {
    pub fn new(
        cool: Cool,
        on_pressed: Option<OnPressed<Message>>,
        on_pressed_install: Option<OnPressedInstall<Message>>,
        on_pressed_update: Option<OnPressedUpdate<Message>>,
        on_progress_changed: Option<OnProgressChanged<Message>>,
    ) -> Self {
        let name = cool.name.clone();
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
            content: Self::build(name.as_str(), false).into(),
        }
    }

    pub fn build(name: &str, is_checked: bool) -> Container<'a, CoolMessage, iced::Renderer> {
        let check_box = checkbox::<CoolMessage, _>(name, is_checked, CoolMessage::Pressed);

        let default_checkbox = checkbox("Default", false, CoolMessage::Pressed);

        // let progress = iced::widget::progress_bar(RangeInclusive::new(0.0, 100.0), 50.0)
        //     .style(iced::theme::ProgressBar::from(|_: &'_ iced::Theme| {
        //         iced::widget::progress_bar::Appearance {
        //             background: Background::Color(0x333333ff.to_rgba()),
        //             bar: Background::Color(0x50a1ffff.to_rgba()),
        //             border_radius: BorderRadius::from([0.0, 0.0, 4.0, 4.0]),
        //         }
        //     }))
        //     .height(4);

        let content = iced::widget::column![check_box, default_checkbox].spacing(8);

        container(content)
            .style(iced::theme::Container::from(|_: &'_ iced::Theme| {
                container::Appearance {
                    text_color: None,
                    // background: Some(Background::Color(0x141414ff.to_rgba())),
                    background: Some(Background::Color(0x00ff00ff.to_rgba())),
                    border_radius: BorderRadius::from(12.0),
                    border_color: 0x434343ff.to_rgba(),
                    border_width: 1.0,
                }
            }))
            .width(420.0)
    }
}

pub fn cool_card<'a, Message>(
    cool: Cool,
    on_pressed: Option<OnPressed<Message>>,
    on_pressed_install: Option<OnPressedInstall<Message>>,
    on_pressed_update: Option<OnPressedUpdate<Message>>,
    on_progress_changed: Option<OnProgressChanged<Message>>,
) -> CoolCard<'a, Message> {
    CoolCard::new(
        cool,
        on_pressed,
        on_pressed_install,
        on_pressed_update,
        on_progress_changed,
    )
}

// impl<'a, Message> iced::widget::Component<Message, iced::Renderer> for CoolCard<'a, Message> {
//     type State = ();
//     type Event = CoolMessage;
//
//     fn update(&mut self, _: &mut Self::State, event: Self::Event) -> Option<Message> {
//         match event {
//             CoolMessage::Pressed(value) => self.on_pressed.map(|f| f(value)),
//             CoolMessage::PressedInstall => self.on_pressed_install.map(|f| f()),
//             CoolMessage::PressedUpdate => self.on_pressed_update.map(|f| f()),
//             CoolMessage::ProgressChanged(value) => self.on_progress_changed.map(|f| f(value)),
//         }
//     }
//
//     fn view(&self, _: &Self::State) -> Element<'a, Self::Event, iced::Renderer> {
//         CoolCard::<'_, CoolMessage>::build("123", false).into()
//     }
// }

// impl<'a, Message> From<CoolCard<Message>> for Element<'a, Message, Renderer>
//     where
//         Message: 'a,
// {
//     fn from(value: CoolCard<Message>) -> Self {
//         component(value)
//     }
// }

impl<'a, Message> Widget<Message, iced::Renderer> for CoolCard<'a, Message> {
    fn width(&self) -> Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> Length {
        self.content.as_widget().height()
    }

    fn layout(&self, renderer: &iced::Renderer, limits: &Limits) -> Node {
        let content_layout = self.content.as_widget().layout(renderer, limits);

        layout::Node::with_children(Size::new(420.0, 100.0), vec![content_layout])
    }
    fn draw(
        &self,
        state: &Tree,
        renderer: &mut iced::Renderer,
        theme: &<iced::Renderer as iced::advanced::Renderer>::Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        println!("{:?}", layout.bounds());

        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );

        let progress: Element<CoolMessage, iced::Renderer> =
            iced::widget::progress_bar(RangeInclusive::new(0.0, 100.0), 50.0)
                .style(iced::theme::ProgressBar::from(|_: &'_ iced::Theme| {
                    iced::widget::progress_bar::Appearance {
                        background: Background::Color(0x333333ff.to_rgba()),
                        bar: Background::Color(0x50a1ffff.to_rgba()),
                        border_radius: BorderRadius::from([0.0, 0.0, 12.0, 12.0]),
                    }
                }))
                .height(layout.bounds().height)
                .into();

        let mut rect = layout.bounds();
        rect.y += rect.height - 4.0;
        rect.height = 20.0;
        renderer.with_layer(rect, |it| {
            progress.as_widget().draw(
                &state.children[0],
                it,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor,
                viewport,
            );
        });
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.content.as_widget())]
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> Interaction {
        self.content.as_widget().mouse_interaction(
            &state.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }
}

impl<'a, Message> From<CoolCard<'a, Message>> for Element<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    fn from(cool_card: CoolCard<'a, Message>) -> Self {
        Self::new(cool_card)
    }
}
