mod hackattic;

use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt,
    SimpleComponent,
};
use relm4::{adw, gtk};

struct AppModel {
    counter: u8,
    image: gtk::Picture,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Mon app à la con"),
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

                    gtk::Label {
                        #[watch]
                        set_label: &format!("{}", model.counter),
                        add_css_class: "title-1",
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 12,
                        set_halign: gtk::Align::Center,

                        gtk::Button {
                            set_label: "Decrement",
                            set_width_request: 120,
                            add_css_class: "pill",
                            connect_clicked => AppMsg::Decrement
                        },
                        gtk::Button {
                            set_label: "Increment",
                            set_width_request: 120,
                            add_css_class: "pill",
                            add_css_class: "suggested-action",
                            connect_clicked => AppMsg::Increment
                        },
                    }
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let image_url = hackattic::get_qr_code().unwrap_or(String::from("pouet"));
        println!("URL: {}", image_url);

        // Create a Picture widget
        let image = gtk::Picture::new();
        image.set_can_shrink(false);

        // Fetch and load the image
        if let Ok(response) = reqwest::blocking::get(&image_url) {
            if let Ok(bytes) = response.bytes() {
                let glib_bytes = gtk::glib::Bytes::from(&bytes.to_vec());
                if let Ok(texture) = gtk::gdk::Texture::from_bytes(&glib_bytes) {
                    image.set_paintable(Some(&texture));
                    // Set a large size to make the QR code readable
                    image.set_size_request(425, 425);
                }
            }
        }

        let model = AppModel { counter, image };

        let image_widget = gtk::Box::new(gtk::Orientation::Vertical, 0);
        image_widget.set_vexpand(true);
        image_widget.set_hexpand(true);
        image_widget.set_halign(gtk::Align::Center);
        image_widget.set_valign(gtk::Align::Center);
        image_widget.append(&model.image);

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }

    // // Update the view to represent the updated model.
    // fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
    //     widgets
    //         .label
    //         .set_label(&format!("Counter: {}", self.counter));
    // }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple_manual");
    app.run::<AppModel>(0);
}
