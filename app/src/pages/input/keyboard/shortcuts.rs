use cosmic::iced::{widget::horizontal_space, Length};
use cosmic::widget::settings;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

//crate::app::Message::Page

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("keyboard-shortcuts", "input-keyboard-symbolic")
            .title(fl!("keyboard-shortcuts"))
            .description(fl!("keyboard-shortcuts", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
}
