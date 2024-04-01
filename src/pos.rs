pub struct Pos
{
	x:	u32,
	y:	u32,
}

pub enum Direction
{
	Left,
	Right,
	Up,
	Down,
	UpLeft,
	UpRight,
	DownLeft,
	DownRight,
}

impl Pos
{
	pub fn set_pos(	&mut self,
					new_x: u32,
					new_y: u32) -> bool
	{
		//check valid
		
		self.x = new_x;
		self.y = new_y;
		return true;
	}
	
	pub fn mov(	&mut self, 
				direction: Direction)
	{
		//check valid
		
		match direction
		{
			Direction::Left => 
			{
				self.x -= 1;
			},
			Direction::Right => 
			{
				self.x += 1;
			},
			Direction::Up =>
			{
				self.y -= 1;
			},
			Direction::Down => 
			{
				self.y += 1;
			},
			Direction::UpLeft => 
			{
				self.x -= 1;
				self.y -= 1;
			},
			Direction::UpRight => 
			{
				self.x += 1;
				self.y -= 1;
			},
			Direction::DownLeft => 
			{
				self.x -= 1;
				self.y += 1;
			},
			Direction::DownRight => 
			{
				self.x += 1;
				self.y += 1;
			},
		}
	}
}
