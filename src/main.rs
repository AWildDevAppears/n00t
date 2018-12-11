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
    let win = newwin(70, 30, 1, 2);

    for title in notebook.get_note_titles() {
        wprintw(win, title);
        wprintw(win, "\n");
    }

    wrefresh(win);
}

fn create_body(body: String) {
    let win = newwin(70, 100, 1, 30);

    wprintw(win, &body);
    wrefresh(win);
}
