extern crate ncurses;

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
