# Bibliofile
### A TUI-based ebook reader inspired by NCurses-era programs!

Bibliofile is a simplified epub reader with automatic bookmarking!

Instructions:

 - installation
 - use
 - feedback



 ## installation

 At the moment, Bibliofile is a work-in-progress. 

I have uploaed the crate to crates.io since it is in a minimally-workable state at the moment, but if you wish to compile it yourself simply follow these directions:


 ### 1

 Install Rust. The full Instructions on how to do this are here: https://www.rust-lang.org/tools/install

 The easiest way of installing Rust is with the following command:

 ```bash
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 ```


 ### 2

 Clone the repository to the directory of your choice

 ```git
 git clone https://git.whoisthisjoker.com/daniel/bibliofile
 ```

 ### 3

cd into the directory *bibliofile*. Compile the program. This is best done with cargo.

 ```rust
 cargo build
 ```

 The executable will be in target/debug.

 Please keep in mind, at this point in development compilation success is not guaranteed.

 ## running

 Running the program is simple, as at this point there is not many features. Simply run

 ```bibliofile [booknamehere.epub]```

There will be buttons at the bottom of each page that allow you to turn pages. Simply press "last" or "next" to turn the page. There are not yet any keyboard commands to turn pages, but that is the next feature I mean to implement.


## disclamers

This program is incomplete. It will be lacking features such s searching text, skipping pages, and using the table of contents. I update this program regularly, so stay tuned!

## license

    Bibliofile: A TUI epub reader inspired by DOS-era programs

    Copyright (C) 2023  Daniel Redd joes
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
