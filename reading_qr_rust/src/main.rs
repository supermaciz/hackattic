mod hackattic;
mod qr_reader;

use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::{adw, gtk};

struct AppModel {
    solution: String,
    hackattic_response: String,
    image: gtk::Picture,
    image_bytes: bytes::Bytes,
}

#[derive(Debug)]
enum AppMsg {
    Decode,
    Submit,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = String;
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Let's solve Reading QR"),
            set_default_width: 600,
            set_default_height: 700,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    set_show_end_title_buttons: true,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_halign: gtk::Align::Fill,
                    set_valign: gtk::Align::Fill,
                    set_vexpand: true,
                    set_hexpand: true,
                    set_margin_all: 4,

                    #[local_ref]
                    image_widget -> gtk::Box {}
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_margin_all: 12,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 12,
                        set_halign: gtk::Align::Center,

                        gtk::Button {
                            set_label: "Decode",
                            set_width_request: 120,
                            connect_clicked => AppMsg::Decode
                        },
                        gtk::Label {
                            #[watch]
                            set_label: &format!("{}", model.solution),
                            add_css_class: "title-1"
                        },
                        gtk::Button {
                            set_label: "Submit solution",
                            #[watch]
                            set_sensitive: !model.solution.is_empty(),
                            set_width_request: 120,
                            add_css_class: "pill",
                            add_css_class: "suggested-action",
                            connect_clicked => AppMsg::Submit
                        },
                        gtk::Label {
                            #[watch]
                            set_label: model.hackattic_response.as_str(),
                            add_css_class: "body-2"
                        }
                    }
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        solution: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let image_url = hackattic::get_qr_code().unwrap_or(String::from("pouet"));
        println!("URL: {}", image_url);

        // Create a Picture widget
        let image = gtk::Picture::new();
        let mut image_bytes = bytes::Bytes::new();

        // Fetch and load the image
        if let Ok(response) = reqwest::blocking::get(&image_url) {
            if let Ok(bytes) = response.bytes() {
                image_bytes = bytes.clone();
                let glib_bytes = gtk::glib::Bytes::from(&bytes.to_vec());
                if let Ok(texture) = gtk::gdk::Texture::from_bytes(&glib_bytes) {
                    image.set_paintable(Some(&texture));
                }
            }
        }
        let model = AppModel {
            solution,
            image,
            image_bytes,
            hackattic_response: String::new(),
        };

        let image_widget = gtk::Box::new(gtk::Orientation::Vertical, 0);
        image_widget.set_halign(gtk::Align::Center);
        image_widget.set_valign(gtk::Align::Center);
        image_widget.append(&model.image);

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Decode => {
                println!("Decode QR code");
                match qr_reader::read_qr(self.image_bytes.clone()) {
                    Ok(solution) => self.solution = solution,
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            AppMsg::Submit => {
                println!("Submit solution");
                match hackattic::post_solution(&self.solution) {
                    Ok(response) => self.hackattic_response = response,
                    Err(e) => println!("Error: {:?}", e),
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple_manual");
    app.run::<AppModel>(String::from(""));
}
