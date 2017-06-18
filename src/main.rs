#![feature(proc_macro)]

extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

use gtk::{Inhibit, WidgetExt, WindowExt};
use relm::Widget;
use relm_attributes::widget;

use self::Msg::*;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn init_view(&self, _model: &mut ()) {
        self.window.set_titlebar(&self.bar);
        self.label.set_text(&self.window.get_id().to_string());
    }

    fn model() -> () {
    }

    fn update(&mut self, event: Msg, _model: &mut ()) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        #[name="window"]
        gtk::ApplicationWindow {
            #[name="label"]
            gtk::Label {
                text: "",
            },
            #[name="bar"]
            gtk::HeaderBar {
                title: "Title",
                subtitle: "subtitle",
                show_close_button: true,
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
