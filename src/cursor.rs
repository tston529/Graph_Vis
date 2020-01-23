use std::fmt;

pub struct Cursor {
	pub x: i16,
	pub y: i16,
    pub last_x: i16,
    pub last_y: i16,
    pub x_left_boundary: usize,
    pub x_right_boundary: usize,
	pub cursor_dir: Option<CursorDir>,
}

impl Cursor {
    pub fn save_cursor_pos(&mut self) {
        self.last_x = self.x;
        self.last_y = self.y;
        CursorDir::save_cursor_pos();
    }

    pub fn ret_cursor_pos(&mut self) {
        self.x = self.last_x;
        self.y = self.last_y;
        CursorDir::ret_cursor_pos();
    }

	pub fn move_cursor(&mut self) {
		let dir = self.cursor_dir.unwrap();
	    match dir {
	        CursorDir::Up(x) => self.y -= x,
	        CursorDir::Down(x) => self.y += x,
	        CursorDir::Right(x) => self.x += x,
	        CursorDir::Left(x) => self.x -= x,
	    }
	    CursorDir::move_cursor(dir);
	}

	pub fn move_cur(&mut self, dir: CursorDir) {
	    match dir {
	        CursorDir::Up(x) => self.y -= x,
	        CursorDir::Down(x) => self.y += x,
	        CursorDir::Right(x) => self.x += x,
	        CursorDir::Left(x) => self.x -= x,
	    }
	    CursorDir::move_cursor(dir);
	}

	pub fn print(&mut self, c: char) {
        if self.y > 0 && self.x > 0 {
		  print!("{}", c);
        }
		self.x += 1;
	}
}

#[derive(Copy, Clone, PartialEq)]
pub enum CursorDir {
    Up(i16),
    Down(i16),
    Right(i16),
    Left(i16),
}

impl CursorDir {
	pub fn move_cursor(self) {
	    print!("{}", self);
	}

	#[inline(always)]
	pub fn save_cursor_pos() {
	    print!("\x1B7");
	}

	#[inline(always)]
	pub fn ret_cursor_pos() {
	    print!("\x1B8");
	}
}


impl fmt::Display for CursorDir {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
	        CursorDir::Up(spaces) => {
                if *spaces < 0 {
                    write!(f, "\x1B[0A")
                } else {
                    write!(f, "\x1B[{}A", spaces)
                }
            },
	        CursorDir::Down(spaces) => {
                if *spaces < 0 {
                    write!(f, "\x1B[0B")
                } else {
                    write!(f, "\x1B[{}B", spaces)
                }
            },
	        CursorDir::Right(spaces) => {
                if *spaces < 0 {
                    write!(f, "\x1B[0C")
                } else {
                    write!(f, "\x1B[{}C", spaces)
                }
            },
	        CursorDir::Left(spaces) => {
                if *spaces < 0 {
                    write!(f, "\x1B[0D")
                } else {
                    write!(f, "\x1B[{}D", spaces)
                }
            }
	    }
	}
}
