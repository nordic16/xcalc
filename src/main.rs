mod ui;

use gtk::glib;

const APP_ID: &str = "org.gtk_rs.xcalc";

fn main() -> glib::ExitCode {
    ui::start(APP_ID)
}                       
