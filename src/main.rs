extern crate ncurses;

mod stats;
mod player;
mod pos;

fn main()
{
    /*  test code   */
    ncurses::initscr();
    ncurses::addstr("Hello, world!");
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();
    /*****************/
}
