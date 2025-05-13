/*
Program: Bibliofile
Language: Rustc 1.74.0
ide: kate
Operating system: Arch Linux
Purpose: TUI-based ereader and library manager for Linux terminal environments.
Last edited: 11/18/23
*/

mod html_module;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use cursive::view::Scrollable;
use epub::doc::EpubDoc; //library for navigating epubs
use std::*;
use std::path::Path;
use std::str;

/*
This function will determine if Bibliofile has been opened before. If it has not, it will create a library folder under /opt/bibliofile.

Function: library_Exists
param: none
Return Type: void
 */

//temporarily commented out until I implement the library functionality
/*
fn library_exists(){


    //if /var/lib/bibliofile/library does not exist, create it.
    let library_exist_var = Path::exists("/var/lib/bibliofile/library".as_ref());

    if library_exist_var == false{
        fs::create_dir_all("/var/lib/bibliofile/library").expect("Error...could not create library. If this is your first time running Bibliofile, try running with sudo.\n\
        Keep in mind, this program was designed with *nix systems in mind. Running this on Windows will result in unexpected behavior, as it uses a different file system.");
    }

}
*/




/*
initial function. Reads the ebook passed by argument, and checks if library directory exists in Linux.
name: main
params: none
Return type: void
 */

fn main() {
    //library_exists();
    if env::args().len() == 1 {
        println!("You need to enter a book! \n\
        context: bibliofile book.epub\n\
        Closing program.");
    }
    else {

        screen_func();
    }
}


/*
Initial screen setup. Sets up the initial TUI screen that will be used by the rest of the application
function: screen_func
Params: string
Return type: none
 */
fn screen_func(){
    

    let mut siv = cursive::default();


    siv.add_global_callback('q', |s| s.quit());
    let text = "Welcome to Bibliofile! Press begin to read your book.";

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new(text))
        .title("Bibliofile")
        .button("begin", |s| {get_init_text(s);}),
    );

    // Starts the event loop.
    siv.run();


}



/*
This function is called whenever the screen gets refreshed. This is seperate from get_init_text because
it requires the TUI to already be active to be called.
function: get_text
params: Cursive session
Returns type: void
 */
fn get_text(s: &mut Cursive) {
    
    //removes old layer, builds new one
    s.pop_layer();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let doc = EpubDoc::new(filename);
    let mut doc = doc.unwrap();
    
    //gets page number/bookmark
    let title = doc.mdata("title");
    let bookmark_file_name = title.clone().unwrap() + "_bookmark";
    let next_page = title.clone().unwrap() + "_next";

    //if bookmark for file exists, continue to open. If not, create the file and try again.
    if Path::exists(bookmark_file_name.as_ref()){

        let next_page_var = fs::read_to_string(next_page.clone()).expect("no file found!");

        let mut page_num: usize = next_page_var
            .trim()
            .parse()
            .expect("not a number");// converts from string to usize

    

        page_num = page_num + 1;
        doc.set_current_page(page_num);
        let content = doc.get_current_str();
        let str_content = content.unwrap();
        let text = html_module::main(str_content.0);
        let number_title = page_num;



        let page_string = page_num.to_string();

        //writes where the next page is so that Rust knows where to go
        fs::write(next_page, page_string.as_bytes()).expect("Unable to write file");

        //refreshed screen layout
        s.add_layer(Dialog::around(TextView::new(text))
            .title(title.unwrap() + " page: " + &number_title.to_string())
            .button("last", |s| {get_last_text(s);})
            .button("quit", |s| s.quit())
            .button("bookmark", move |s| {bookmark_func(s, page_num, &bookmark_file_name);})
            .button("next", |s| {get_text(s);})
            .scrollable()
            .scroll_x(true),
    );
    }

    else{
        fs::write(next_page, "1").expect("Unable to write file");
        get_text(s);
    }

}

/*
This function is called to turn the page back 1.
function: get_text
params: Cursive session
Returns type: void
 */
fn get_last_text(s: &mut Cursive) {


    //removes old layer, builds new one
    s.pop_layer();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let doc = EpubDoc::new(filename);
    let mut doc = doc.unwrap();

    //gets page number/bookmark
    let title = doc.mdata("title");
    let bookmark_file_name = title.clone().unwrap() + "_bookmark";
    let next_page = title.clone().unwrap() + "_next";

    //if bookmark for file exists, continue to open. If not, create the file and try again.
    if Path::exists(bookmark_file_name.as_ref()){

        let next_page_var = fs::read_to_string(next_page.clone()).expect("no file found!");

        let mut page_num: usize = next_page_var
            .trim()
            .parse()
            .expect("not a number");// converts from string to usize





        if page_num == 1{
            s.add_layer(Dialog::around(TextView::new("Already on first page!"))
            .button("ok", |s| {s.pop_layer();})
            );
            page_num = 0;
            let page_string = page_num.to_string();

            //writes where the next page is so that Rust knows where to go
            fs::write(next_page, page_string.as_bytes()).expect("Unable to write file");
            get_text(s);
        }
        else{
            page_num = page_num - 1;
            doc.set_current_page(page_num);
            let content = doc.get_current_str();
            let str_content = content.unwrap();
            let text = html_module::main(str_content.0);
            let number_title = page_num;



        let page_string = page_num.to_string();

        //writes where the next page is so that Rust knows where to go
        fs::write(next_page, page_string.as_bytes()).expect("Unable to write file");

        //refreshed screen layout
        s.add_layer(Dialog::around(TextView::new(text))
            .title(title.unwrap() + " page: " + &number_title.to_string())
            .button("last", |s| {get_last_text(s);})
            .button("quit", |s| s.quit())
            .button("bookmark", move |s| {bookmark_func(s, page_num, &bookmark_file_name);})
            .button("next", |s| {get_text(s);})
            .scrollable()
            .scroll_x(true),
        );
        }
    }

    else{
        fs::write(next_page, "1").expect("Unable to write file");
        get_text(s);
    }

}

/*
    Loads basic config files for ebook. Checks if there is a bookmark, and if not, displays what it sees and passes along
    to the appropriate function.
 */
fn get_init_text(s: &mut Cursive) {

    //removes old layer, builds new one
    s.pop_layer();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let doc = EpubDoc::new(filename);
    let mut doc = doc.unwrap();

    //gets page number/bookmark
    let title = doc.mdata("title");
    let bookmark_file_name = title.clone().unwrap() + "_bookmark";
    let next_page = title.clone().unwrap() + "_next";

    //if bookmark for file exists, continue to open. If not, create the file and try again.
    if Path::exists(bookmark_file_name.as_ref()){

        let starting_page = fs::read_to_string(bookmark_file_name.clone()).expect("no file found!");


        let page_num: usize = starting_page
            .trim()
            .parse()
            .expect("not a number");// converts from string to usize


        let usize_num = page_num as usize; //set_current_page must be usize, not int. This line converts.

        doc.set_current_page(usize_num);
        let content = doc.get_current_str();
        let str_content = content.unwrap();
        let text = html_module::main(str_content.0);



        let page_string = page_num.to_string();


        fs::write(next_page, page_string.as_bytes()).expect("Unable to write file");

        //refreshed screen layout
        s.add_layer(Dialog::around(TextView::new(text))
            .title(title.unwrap() + " page: " + &usize_num.to_string())
            .button("last", |s| {get_last_text(s);})
            .button("quit", |s| s.quit())
            .button("bookmark", move |s| {bookmark_func(s, page_num, &bookmark_file_name);})
            .button("next", |s| {get_text(s);})
            .scrollable()
            .scroll_x(true),
    );
    }

    else{
        fs::write(bookmark_file_name, "1").expect("Unable to write file");
        get_init_text(s);
    }

}




fn bookmark_func(s: &mut Cursive, page_num: usize, bookmark: &str){

    if page_num != 1{
    let new_page_num = page_num;
    //new_page_num = new_page_num - 1;
    let page_string = new_page_num.to_string();
    fs::write(bookmark, page_string.as_bytes()).expect("Unable to write file");


    }
    else{
        let page_num = "1";
        fs::write(bookmark, page_num.as_bytes()).expect("Unable to write file");
    }

    s.add_layer(Dialog::around(TextView::new("Bookmark placed!"))
        .button("ok", |s| {s.pop_layer();})
    )
    }




