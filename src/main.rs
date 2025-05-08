/*@author: Daniel Jones
 *@ Last modified: May 8 2925
 *@ description: TUI ebook reader for Linux
 */

use std::env;
mod screen;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        screen::screen::main_screen();
    }
    else{
        println!("{}", args.len());
    }

}
