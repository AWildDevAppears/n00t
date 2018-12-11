extern crate ncurses;

use ncurses::*;

mod models;

fn main() {
    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    refresh();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

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
    let win = newwin(70, 30, 0, 0);
    wprintw(win, "\n");

    for title in notebook.get_note_titles() {
        wprintw(win, "  ");
        wprintw(win, title);
        wprintw(win, "\n");
    }

    wrefresh(win);
}
