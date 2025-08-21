use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, Button};

pub struct HomeController {
    root: GtkBox,
    pub btn_play: Button,
    pub btn_settings: Button,
}

impl HomeController {
    pub fn new() -> Self {
        let root = GtkBox::new(Orientation::Vertical, 16);
        root.add_css_class("home-container");
        root.set_margin_top(32);
        root.set_margin_bottom(32);
        root.set_margin_start(32);
        root.set_margin_end(32);
        root.set_halign(gtk4::Align::Center);
        root.set_valign(gtk4::Align::Center);

        let title = Label::new(Some("Bienvenue sur chess.rs"));
        title.add_css_class("title-1");
        title.set_margin_bottom(24);

        let btn_play = Button::with_label("Jouer");
        let btn_settings = Button::with_label("Paramètres");
        for b in [&btn_play, &btn_settings] {
            b.add_css_class("suggested-action");
            b.add_css_class("home-button");
            b.set_halign(gtk4::Align::Center);
            b.set_valign(gtk4::Align::Center);
        }
        btn_play.remove_css_class("suggested-action"); // only style via CSS

        root.append(&title);
        root.append(&btn_play);
        root.append(&btn_settings);

        Self { root, btn_play, btn_settings }
    }

    pub fn root(&self) -> &GtkBox {
        &self.root
    }
}
