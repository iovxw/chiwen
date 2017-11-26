extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate futures;
extern crate futures_glib;
extern crate libpsensor;

use std::rc::Rc;
use std::time::Duration;

use gtk::prelude::*;
use relm::{Widget, Relm, Update};
use futures_glib::Interval;
use futures::prelude::*;
use libpsensor::SensorList;

use self::Msg::*;

#[derive(Clone)]
pub struct Model {
    sensors: Rc<SensorList>,
}

#[derive(Msg)]
pub enum Msg {
    SensorsUpdate,
    Quit,
}

#[derive(Clone)]
struct Win {
    model: Model,
    temperatures: Vec<gtk::Label>,
    header_bar: gtk::HeaderBar,
    window: gtk::Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        Model { sensors: Rc::new(SensorList::new()) }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            SensorsUpdate => {
                for (id, sensor) in self.model.sensors.iter().enumerate() {
                    self.temperatures[id].set_text(&sensor.value.get().round().to_string());
                }
            }
            Quit => gtk::main_quit(),
        }
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let s = self.clone();
        let stream =
            Interval::new(Duration::from_secs(1)).map(move |_| { s.model.sensors.update(); });
        relm.connect_exec_ignore_err(stream, |_| Msg::SensorsUpdate);
    }
}

impl Widget for Win {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        let header_bar = gtk::HeaderBar::new();
        header_bar.set_show_close_button(true);
        header_bar.set_title("Title");
        header_bar.set_subtitle("subtitle");
        window.set_titlebar(&header_bar);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let mut temperatures = Vec::with_capacity(model.sensors.len());
        for sensor in model.sensors.iter() {
            let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
            let name = gtk::Label::new(sensor.name.as_str());
            let temperature = gtk::Label::new("0");
            hbox.add(&name);
            hbox.add(&temperature);
            temperatures.push(temperature);
            vbox.add(&hbox);
        }
        window.add(&vbox);

        connect!(relm, window, connect_delete_event(_, _), return (Some(Quit), Inhibit(false)));

        window.show_all();

        Win {
            temperatures,
            header_bar,
            window,
            model,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
