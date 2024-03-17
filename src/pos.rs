struct Pos
{
	x:	i32,
	y:	i32,
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
	fn mov(	&mut self, 
			direction: Direction,
			min_x: i32,
			max_x: i32,
			min_y: i32,
			max_y: i32) -> bool
	{
		let min_check = |a, min| -> bool
		{
			if (a - 1) < min
			{
				return false;
			}
			else
			{
				return true;
			}
		};
		
		let max_check = |a, max| -> bool
		{
			if (a + 1) > max
			{
				return false;
			}
			else
			{
				return true;
			}
		};
		
		match direction
		{
			Direction::Left => 
			{
				if min_check(self.x, min_x)
				{
					self.x -= 1;
					return true;
				}
			},
			Direction::Right => 
			{
				if max_check(self.x, max_x)
				{
					self.x += 1;
					return true;
				}
			},
			Direction::Up =>
			{
				if min_check(self.y, min_y)
				{
					self.y -= 1;
					return true;
				}
			},
			Direction::Down => 
			{
				if max_check(self.y, max_y)
				{
					self.y += 1;
					return true;
				}
			},
			Direction::UpLeft => 
			{
				if min_check(self.x, min_x) && min_check(self.y, min_y)
				{
					self.x -= 1;
					self.y -= 1;
					return true;
				}
			},
			Direction::UpRight => 
			{
				if max_check(self.x, max_x) && min_check(self.y, min_y)
				{
					self.x += 1;
					self.y -= 1;
					return true;
				}
			},
			Direction::DownLeft => 
			{
				if min_check(self.x, min_x) && max_check(self.y, max_y)
				{
					self.x -= 1;
					self.y += 1;
					return true;
				}
			},
			Direction::DownRight => 
			{
				if max_check(self.x, max_x) && max_check(self.y, max_y)
				{
					self.x += 1;
					self.y += 1;
					return true;
				}
			},
		}
		
		return false
	}
}
