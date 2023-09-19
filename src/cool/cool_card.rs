use std::ops::RangeInclusive;

use crate::app::ICON_FONT;
use iced::advanced::graphics::core::Element;
use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::Tree;
use iced::advanced::{layout, Layout, Renderer, Widget};
use iced::alignment::Horizontal;
use iced::mouse::{Cursor, Interaction};
use iced::widget::{checkbox, container, text, Container, ProgressBar};
use iced::{Background, BorderRadius, Font, Length, Padding, Pixels, Rectangle, Size};

use crate::color_extension::RGB;
use crate::cool::Cool;

pub type OnPressed<Message> = fn(bool) -> Message;
pub type OnPressedInstall<Message> = fn() -> Message;
pub type OnPressedUpdate<Message> = fn() -> Message;
pub type OnProgressChanged<Message> = fn(u32) -> Message;

#[derive(Clone, Copy)]
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
    pub progress_bar: Element<'a, CoolMessage, iced::Renderer>,
}

impl<'a, Message> CoolCard<'a, Message> {
    pub fn new(
        cool: Cool,
        on_pressed: Option<OnPressed<Message>>,
        on_pressed_install: Option<OnPressedInstall<Message>>,
        on_pressed_update: Option<OnPressedUpdate<Message>>,
        on_progress_changed: Option<OnProgressChanged<Message>>,
    ) -> Self {
        let (content, progress_bar) = *Self::build(&cool, false);
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
            content: content.into(),
            progress_bar: progress_bar.into(),
        }
    }

    pub fn build(
        cool: &Cool,
        is_checked: bool,
    ) -> Box<(
        Container<'a, CoolMessage, iced::Renderer>,
        ProgressBar<iced::Renderer>,
    )> {
        let check_box =
            checkbox::<CoolMessage, _>(cool.name.as_str(), is_checked, CoolMessage::Pressed)
                // .text_line_height(24.0 / 20.0)
                .text_line_height(text::LineHeight::Absolute(Pixels(24.0)))
                .icon(checkbox::Icon {
                    font: ICON_FONT,
                    code_point: '\u{0080}',
                    size: Some(20.0),
                    line_height: text::LineHeight::Absolute(Pixels(24.0)),
                    shaping: text::Shaping::Basic,
                })
                .text_size(20.0);

        let text = container(
            text(cool.description.as_str())
                .size(16.0)
                .line_height(text::LineHeight::Absolute(Pixels(24.0))),
        )
        .max_height(48.0);

        let content = iced::widget::column![check_box, text].spacing(12.0);

        let update_button = iced::widget::button("Update")
            .on_press(CoolMessage::PressedUpdate)
            .padding(Padding::from([7.0, 12.0]));

        let install_button = iced::widget::button("Install")
            .on_press(CoolMessage::PressedInstall)
            .padding(Padding::from([7.0, 12.0]));

        let button_row = iced::widget::row![update_button, install_button]
            .spacing(12.0)
            .width(Length::Shrink);

        let button_container = container(button_row)
            .width(Length::Fill)
            .align_x(Horizontal::Right)
            .padding(Padding::from([4.0, 0.0]))
            .style(iced::theme::Container::from(|_: &'_ iced::Theme| {
                container::Appearance {
                    text_color: None,
                    background: Some(Background::Color(0x000000ff.to_rgba())),
                    border_radius: BorderRadius::default(),
                    border_color: Default::default(),
                    border_width: 0.0,
                }
            }));

        let content = iced::widget::column![content, button_container]
            .spacing(12.0)
            .width(Length::Fill);

        let container = container(content)
            .style(iced::theme::Container::from(|_: &'_ iced::Theme| {
                container::Appearance {
                    text_color: None,
                    background: Some(Background::Color(0x141414ff.to_rgba())),
                    border_radius: BorderRadius::from(12.0),
                    border_color: 0x434343ff.to_rgba(),
                    border_width: 1.0,
                }
            }))
            .padding(Padding::from([16.0, 20.0, 24.0, 24.0]))
            .width(420.0);

        let progress = iced::widget::progress_bar(RangeInclusive::new(0.0, 1.0), 1.0)
            .style(iced::theme::ProgressBar::from(|_: &'_ iced::Theme| {
                iced::widget::progress_bar::Appearance {
                    background: Background::Color(0x333333ff.to_rgba()),
                    bar: Background::Color(0x50a1ffff.to_rgba()),
                    border_radius: BorderRadius::from([0.0, 0.0, 12.0, 12.0]),
                }
            }))
            .width(420.0)
            .height(Length::Fill);

        Box::new((container, progress))
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

impl<'a, Message> Widget<Message, iced::Renderer> for CoolCard<'a, Message> {
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &iced::Renderer, limits: &Limits) -> Node {
        let content_layout = self.content.as_widget().layout(renderer, limits);

        layout::Node::with_children(
            Size::new(420.0, content_layout.size().height),
            vec![content_layout.clone(), content_layout],
        )
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
        let children = layout.children().collect::<Vec<_>>();
        let content_layout = children[0];
        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            viewport,
        );

        let progress_bar_layout = children[1];
        let mut rect = progress_bar_layout.bounds();
        let height = 4.0;
        rect.y += progress_bar_layout.bounds().height - height;
        rect.width = 0.5 * progress_bar_layout.bounds().width;
        rect.height = height;
        renderer.with_layer(rect, |it| {
            self.progress_bar.as_widget().draw(
                &state.children[1],
                it,
                theme,
                style,
                progress_bar_layout,
                cursor,
                viewport,
            );
        });
    }

    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(self.content.as_widget()),
            Tree::new(self.progress_bar.as_widget()),
        ]
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
