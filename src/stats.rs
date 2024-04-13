pub struct Stats
{
	health:		i32,
	mana:		i32,
	strength:	i32,
	dexterity:	i32,
	toughness:	i32,
	speed:		i32,
}


impl Stats
{
	pub fn new(	health:	i32,
				mana: 	i32) -> Stats
	{
		return Stats{	health: 	health,
						mana: 		mana,
						strength: 	1,
						dexterity: 	1,
						toughness:	1,
						speed:		1
		};
	}
}
