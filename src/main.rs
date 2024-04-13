extern crate ncurses;

mod stats;
mod player;
mod pos;
mod map;

pub use player::Player as Player;
pub use stats::Stats as Stats;
pub use pos::Pos as Pos;


static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;

fn main()
{
    let mut p: Player = Player::new(Stats::new(10, 10), Pos::new());
    
    /*  test code   */
    ncurses::initscr();
    ncurses::noecho();
    ncurses::clear();
    
    'gameloop: loop
    {
        let input = ncurses::getch();
        ncurses::clear();
        
        match input
        {
            52  =>  ncurses::addstr("L"),
            54  =>  ncurses::addstr("R"),
            56  =>  ncurses::addstr("U"),
            50  =>  ncurses::addstr("D"),
            113 =>  {
                        ncurses::addstr("Q");
                        break 'gameloop;
                    },
            _   =>  ncurses::addstr("?"),
        };
        
        ncurses::refresh();
    }
    
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();
    /*****************/
}
