use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::Tree;
use iced::advanced::Layout;
use iced::mouse::Cursor;
use iced::widget::canvas::Path;
use iced::{BorderRadius, Element, Length, Point, Rectangle};

use crate::color_extension::RGB;

pub struct ClipContainer<'a, Message, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
    Renderer::Theme: iced::widget::container::StyleSheet,
{
    width: Length,
    height: Length,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> ClipContainer<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Renderer::Theme: iced::widget::container::StyleSheet,
{
    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            content: content.into(),
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<'a, Message, Renderer> iced::advanced::Widget<Message, Renderer>
    for ClipContainer<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Renderer::Theme: iced::widget::container::StyleSheet,
{
    fn width(&self) -> Length {
        self.width
    }
    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        println!("[clip] limits: {:?}", limits);
        let content_layout = self.content.as_widget().layout(renderer, limits);
        println!("content_layout: {:?}", content_layout.bounds());
        let size = limits
            .width(self.width)
            .height(self.height)
            .resolve(content_layout.size());
        println!("size: {:?}", size);
        Node::with_children(size, vec![content_layout])
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );

        let p = Path::rectangle(Point::new(0.0, 0.0), layout.bounds().size());

        renderer.fill_quad(
            iced::advanced::renderer::Quad {
                bounds: layout.bounds(),
                border_radius: BorderRadius::from(12.0),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
            },
            0xff0000ff.to_rgba(),
        );
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.content.as_widget())]
    }
}

impl<'a, Message, Renderer> From<ClipContainer<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + iced::advanced::Renderer,
    Renderer::Theme: iced::widget::container::StyleSheet,
{
    fn from(container: ClipContainer<'a, Message, Renderer>) -> Self {
        Element::new(container)
    }
}
