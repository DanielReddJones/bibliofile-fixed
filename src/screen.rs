/*
 *@author Daniel Jones
 *@last modified May 12 2025
 *@ description: Terminal manager class
 * */

extern crate cursive;

pub mod screen {
    
    //Module declarations
    use cursive::views::TextView;
    use cursive::Cursive;

    pub fn main_screen() {
        let mut siv = cursive::default();
        siv.add_global_callback('q', Cursive::quit);
        

        //this is the layer in which the ebook text will appear
        siv.add_layer(TextView::new(
            "Hello world! \n\
            Press q to close the application!",
        ));
        siv.run();
    }
}
