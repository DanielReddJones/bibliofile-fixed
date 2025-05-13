/*
 *@author Daniel Jones
 *@last modified May 12 2025
 *@ description: Terminal manager class
 * */

extern crate cursive;

pub mod screen {
use cursive::views::TextView;
use cursive::Cursive;

    pub fn main_screen() {
        let mut siv = cursive::default();
        siv.add_global_callback('q', Cursive::quit);

        siv.add_layer(TextView::new(
"Hello world! \n\
Press q to close the application!",
                ));
        siv.run();
    }
}
