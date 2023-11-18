/// Mod that handles the ui aspect of the app

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Builder};

/// Starts event loop
pub fn start(app_id: &str) -> glib::ExitCode {
    let app = Application::builder().application_id(app_id).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {

    let src = include_str!("ui.ui");
    let builder = Builder::from_string(src);
    let window = builder.object::<ApplicationWindow>("window").expect("couldn't get window");
    
    window.set_application(Some(app));


    /*
    let layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .valign(gtk::Align::Fill)
        .halign(gtk::Align::Fill)
        .margin_top(5)
        .margin_bottom(5)
        .build();


    let field = gtk::TextView::builder()
        .editable(true)
        .build();
    
    let grid = gtk::Grid::builder()
        .column_spacing(2)
        .row_spacing(2)
        .valign(gtk::Align::Fill)
        .halign(gtk::Align::Fill)
        .build();

    for rows in 0..3 {
        for cols in 0..3 {
            // Will produce the following output: 1 2 3 | 4 5 6 | 7 8 9
            let pos_x = cols + (3 * rows + 1);

            let button = gtk::Button::builder()
                .label(pos_x.to_string())
                .build();

            grid.attach(&button, cols, rows, 1, 1)
        }
    }
    layout.append(&field);
    layout.append(&grid); 

    window.set_child(Some(&layout));
*/
    // Present window
    window.present();
}
