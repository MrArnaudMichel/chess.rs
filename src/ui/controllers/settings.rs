use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Adjustment, SpinButton, Switch, Align};
use adw::{PreferencesPage, PreferencesGroup, ActionRow};
use adw::prelude::*;

pub struct SettingsController {
    root: GtkBox,
}

impl SettingsController {
    pub fn new() -> Self {
        let root = GtkBox::new(Orientation::Vertical, 0);
        root.set_margin_top(12);
        root.set_margin_bottom(12);
        root.set_margin_start(12);
        root.set_margin_end(12);

        // Adwaita preferences page
        let page = PreferencesPage::new();

        // Time group
        let group_time = PreferencesGroup::new();
        group_time.set_title("Horloge");
        let row_time = ActionRow::new();
        row_time.set_title("Temps initial (minutes)");
        let adj = Adjustment::new(5.0, 1.0, 180.0, 1.0, 5.0, 0.0);
        let spin = SpinButton::new(Some(&adj), 1.0, 0);
        spin.set_width_chars(4);
        spin.set_valign(Align::Center);
        spin.set_vexpand(false);
        row_time.add_suffix(&spin);
        row_time.set_activatable_widget(Some(&spin));
        group_time.add(&row_time);

        // Appearance group
        let group_theme = PreferencesGroup::new();
        group_theme.set_title("Apparence");
        let row_theme = ActionRow::new();
        row_theme.set_title("Thème sombre");
        let theme_switch = Switch::new();
        theme_switch.set_valign(Align::Center);
        theme_switch.set_vexpand(false);
        theme_switch.set_hexpand(false);
        theme_switch.add_css_class("normal-switch");
        row_theme.add_suffix(&theme_switch);
        row_theme.set_activatable_widget(Some(&theme_switch));
        group_theme.add(&row_theme);

        page.add(&group_time);
        page.add(&group_theme);

        root.append(&page);

        Self { root }
    }

    pub fn root(&self) -> &GtkBox { &self.root }
}
