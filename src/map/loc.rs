pub struct Loc<'a>
{
	filename: &'a str,
	min_x:	u32,
	min_y:	u32,
	max_x:	u32,
	max_y:	u32,
}
