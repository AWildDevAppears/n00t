extern crate ncurses;

use ncurses::*;

mod models;

fn main() {
    initscr();

    match models::notebook::get_notebook("./notebook.tar") {
        Ok(notebook) => {
            for title in notebook.get_note_titles() {
                printw(title);
            }
            refresh();
        },
        Err(err) => println!("{}", err),
    }

    getch();
    endwin();
}
