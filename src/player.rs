use super::stats;
use super::pos;

pub struct Player
{
	stats:	stats::Stats,
	pos: 	pos::Pos,
}

impl Player
{
	pub fn new(	stats: 	stats::Stats,
				pos: 	pos::Pos) -> Player
	{
		return Player{	stats: stats,
						pos: pos
		};
	}
}
