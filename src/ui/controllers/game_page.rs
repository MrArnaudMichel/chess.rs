use gtk4::prelude::*;
use gtk4::{Align, AspectFrame, Box as GtkBox, Button, Label, ListBox, Orientation, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

use crate::ui::controllers::game_controller::GameController;

pub struct GamePageController {
    root: GtkBox,
}

impl GamePageController {
    pub fn new(controller_ref: Rc<RefCell<Option<GameController>>>) -> Self {
        let hbox = GtkBox::new(Orientation::Horizontal, 12);
        hbox.set_margin_top(12);
        hbox.set_margin_bottom(12);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);

        // Aspect frame for square board
        let aspect = AspectFrame::new(0.5, 0.5, 1.0, false);
        {
            let controller = controller_ref.borrow();
            let chessboard_ref = &controller.as_ref().unwrap().ui;
            let grid = chessboard_ref.widget();
            grid.set_hexpand(true);
            grid.set_vexpand(true);
            aspect.set_child(Some(grid));
        }
        aspect.set_hexpand(true);
        aspect.set_vexpand(true);
        aspect.set_halign(Align::Fill);
        aspect.set_valign(Align::Fill);

        // Sidebar
        let sidebar = GtkBox::new(Orientation::Vertical, 10);
        sidebar.set_width_request(320);
        sidebar.add_css_class("sidebar");

        let turn_label = Label::new(Some("Au trait: Blancs"));
        let info_label = Label::new(Some(""));

        // Timers
        let timer_title = Label::new(Some("Temps"));
        timer_title.add_css_class("title-3");
        let white_timer = Label::new(Some("05:00"));
        white_timer.add_css_class("timer-label");
        let black_timer = Label::new(Some("05:00"));
        black_timer.add_css_class("timer-label");

        // Moves panel
        let moves_title = Label::new(Some("Coups"));
        moves_title.add_css_class("title-3");
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
        scrolled.set_vexpand(true);
        scrolled.add_css_class("panel");
        let move_list = ListBox::new();
        scrolled.set_child(Some(&move_list));

        // Nav buttons
        let btn_back = Button::with_label("⟵ Précédent");
        let btn_forward = Button::with_label("Suivant ⟶");
        btn_back.add_css_class("pill");
        btn_forward.add_css_class("pill");

        sidebar.append(&turn_label);
        sidebar.append(&info_label);
        sidebar.append(&timer_title);
        sidebar.append(&white_timer);
        sidebar.append(&black_timer);
        sidebar.append(&moves_title);
        sidebar.append(&scrolled);
        sidebar.append(&btn_back);
        sidebar.append(&btn_forward);

        // Wire UI to chessboard
        {
            if let Some(ref mut controller) = *controller_ref.borrow_mut() {
                controller.ui.set_side_panel_widgets(move_list.clone(), turn_label.clone(), info_label.clone());
                controller.ui.set_timer_labels(white_timer.clone(), black_timer.clone());
                controller.ui.set_turn_label(true);
                controller.ui.update_timer_labels(300, 300);
            }
        }

        // Hook up history
        {
            if let Some(ref mut ctrl) = *controller_ref.borrow_mut() {
                ctrl.set_nav_buttons(btn_back.clone(), btn_forward.clone());
            }
        }
        {
            let controller_ref_for_back = controller_ref.clone();
            btn_back.connect_clicked(move |_| {
                if let Some(ref mut ctrl) = *controller_ref_for_back.borrow_mut() {
                    ctrl.step_back();
                }
            });
        }
        {
            let controller_ref_for_fwd = controller_ref.clone();
            btn_forward.connect_clicked(move |_| {
                if let Some(ref mut ctrl) = *controller_ref_for_fwd.borrow_mut() {
                    ctrl.step_forward();
                }
            });
        }

        hbox.append(&aspect);
        hbox.append(&sidebar);

        Self { root: hbox }
    }

    pub fn root(&self) -> &GtkBox { &self.root }
}
