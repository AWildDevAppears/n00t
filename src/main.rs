extern crate ncurses;

use ncurses::*;

mod models;

fn main() {
    initscr();


    match models::notebook::get_notebook("./notebook.tar") {
        Ok(v) => {
            printw(&*v.name);
            refresh();
        },
        Err(err) => println!("{}", err),
    }


	getch();
	endwin();
}
