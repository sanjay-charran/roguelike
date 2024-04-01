pub enum Tile
{
	Grass,
	Wall,
}

impl Tile
{
	pub fn get_sym(&self) -> char
	{
		match (self)
		{
			Tile::Grass 	=> return ',',
			Tile::Wall		=> return '/',
			_				=> return '?',
		}
	}
}
