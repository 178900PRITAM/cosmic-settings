use cosmic::iced::{widget::horizontal_space, Length};
use cosmic::widget::settings;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;
use std::cell::Cell;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(input_sources()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("keyboard", "input-keyboard-symbolic")
            .title(fl!("keyboard"))
            .description(fl!("keyboard", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
}

fn input_sources() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("keyboard-sources"))
        .descriptions(vec![
            fl!("workspaces-behavior", "dynamic"),
            fl!("workspaces-behavior", "fixed"),
        ])
        .view::<Page>(|_binder, _page, section| {
            let descriptions = &section.descriptions;

            // TODO need something more custom
            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    horizontal_space(Length::Fill),
                ))
                .add(settings::item(
                    &descriptions[1],
                    horizontal_space(Length::Fill),
                ))
                .into()
        })
}
