extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::{Inhibit, WidgetExt, ContainerExt, WindowExt};
use relm::{Widget, RemoteRelm};

use self::Msg::*;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[derive(Clone)]
struct Win {
    label: gtk::Label,
    header_bar: gtk::HeaderBar,
    window: gtk::Window,
}

impl Widget for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;
    type Root = gtk::Window;

    fn model(_: Self::ModelParam) -> Self::Model {
        ()
    }

    fn root(&self) -> &Self::Root {
        &self.window
    }

    fn update(&mut self, event: Self::Msg, _model: &mut Self::Model) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    fn view(relm: &RemoteRelm<Self>, _model: &Self::Model) -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        let header_bar = gtk::HeaderBar::new();
        header_bar.set_show_close_button(true);
        header_bar.set_title("Title");
        header_bar.set_subtitle("subtitle");
        window.set_titlebar(&header_bar);

        let label = gtk::Label::new("label");
        window.add(&label);

        connect!(relm, window, connect_delete_event(_, _) (Some(Quit), Inhibit(false)));

        window.show_all();

        Win {
            label,
            header_bar,
            window,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
