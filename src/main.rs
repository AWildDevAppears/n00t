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

    let gutter = 2;

    match models::notebook::get_notebook("./notebook.tar") {
        Ok(notebook) => {
            let body = notebook.get_note_body(String::from("myNotebook/hello.md"));

            create_menu(notebook, max_y - 2, max_x / 5);
            create_body(body, max_y - 2 , max_x - gutter - (max_x / 5), (max_x / 5) + gutter);

        },
        Err(err) => println!("{}", err),
    }

    getch();
    endwin();
}

fn create_menu(notebook: models::notebook::Notebook, height: i32, width: i32) {
    let outer_win = newwin(height, width, 1, 2);
    let win = newwin(height - 4, width - 4, 3, 4);

    box_(outer_win, 0, 0);
    wrefresh(outer_win);

    for title in notebook.get_note_titles() {
        wprintw(win, title);
        wprintw(win, "\n");
    }

    wrefresh(win);
}

fn create_body(body: String, height: i32, width: i32, offset: i32) {
    let outer_win = newwin(height, width, 1, offset);
    let win = newwin(height - 4 , width - 5, 3, offset + 4);

    box_(outer_win, 0, 0);
    wrefresh(outer_win);

    wprintw(win, &body);
    wrefresh(win);
}
