use cosmic_settings_page as page;

mod keyboard;
mod mouse;

#[derive(Clone, Copy, Debug)]
pub enum Message {
}

#[derive(Default)]
pub struct Page;

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        // XXX icon?
        page::Info::new("input", "input-keyboard-symbolic")
            .title(fl!("input"))
            .description(fl!("input", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<keyboard::Page>().sub_page::<mouse::Page>()
    }
}
