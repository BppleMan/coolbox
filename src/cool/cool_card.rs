use iced::advanced::graphics::core::Element;
use iced::advanced::widget::Widget;
use iced::widget::container::Appearance;
use iced::widget::{component, container, text, Component, Container};
use iced::{theme, Event, Renderer, Theme};

use crate::cool::{Cool, CoolMessage};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CoolCard {
    pub cool: Cool,
    pub selected: bool,
    pub dependent_selected: bool,
    pub installed: bool,
    pub progress: Option<u32>,
}

impl CoolCard {
    pub fn new(cool: Cool) -> Self {
        CoolCard {
            cool,
            selected: false,
            dependent_selected: false,
            installed: false,
            progress: None,
        }
    }
}

pub fn cool_card(cool: Cool) -> CoolCard {
    CoolCard::new(cool)
}

impl<Message> Component<Message, Renderer> for CoolCard {
    type State = ();
    type Event = CoolMessage;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        todo!()
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        container(text("123456"))
            .style(iced::theme::Container::Custom(Box::new(
                RoundedContainerStyle,
            )))
            .into()
    }
}

struct RoundedContainerStyle;

impl container::StyleSheet for RoundedContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        iced::widget::container::Appearance::default()
    }
}
