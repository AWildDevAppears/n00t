#![feature(box_syntax)]

extern crate ncurses;

use ncurses::*;

mod models;

fn main() {
    initscr();
    cbreak();
    noecho();

    match models::notebook::get_notebook("./notebook.tar") {
        Ok(notebook) => {
            create_menu(notebook);
        },
        Err(err) => println!("{}", err),
    }

    getch();
    endwin();
}

fn create_menu(notebook: models::notebook::Notebook) {
    let menu_window = newwin(10, 70, 5, 5);
    box(menu_window, 0 , 0);

    for title in notebook.get_note_titles() {
        printw(title);
    }

    wrefresh(menu_window);
}
