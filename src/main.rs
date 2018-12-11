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
            let body = notebook.get_note_body(String::from("myNotebook/hello.md"));
            create_menu(notebook);
            create_body(body);

        },
        Err(err) => println!("{}", err),
    }

    getch();
    endwin();
}

fn create_menu(notebook: models::notebook::Notebook) {
    let outer_win = newwin(70, 33,1, 2);
    let win = newwin(66, 30, 3, 4);

    box_(outer_win, 0, 0);
    wrefresh(outer_win);

    for title in notebook.get_note_titles() {
        wprintw(win, title);
        wprintw(win, "\n");
    }

    wrefresh(win);
}

fn create_body(body: String) {
    let outer_win = newwin(70, 110, 1, 35);
    let win = newwin(66, 100, 3, 40);

    box_(outer_win, 0, 0);
    wrefresh(outer_win);

    wprintw(win, &body);
    wrefresh(win);
}
