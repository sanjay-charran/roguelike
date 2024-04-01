mod loc;
mod tile;

use std::collections::HashMap;

pub struct Map<'a>
{
	main:		loc::Loc<'a>,
	filepath:	&'a str,
	locs:		HashMap<&'a str, loc::Loc<'a>>,
	current:	String,
	tiles:		Vec<Vec<tile::Tile>>
}
