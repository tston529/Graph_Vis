#![allow(warnings)]

use rand::Rng;
use arrayvec;
use std::fmt;
use std::cmp::PartialEq;
use std::{thread, time};

const _GRAPH_HEIGHT: i32 = 10;
const GRAPH_WIDTH: usize = 10; // How many points are visible at one time
const LINE_MARK: char = '*';
const POINT_CHAR: char = 'X';
const HORIZ_SPACING: usize = 5;
const X_BUFFER: usize = 2;

struct Cursor {
	x: i16,
	y: i16,
    last_x: i16,
    last_y: i16,
    graph_size: usize,
	cursor_dir: Option<CursorDir>,
}

impl Cursor {
    fn save_cursor_pos(&mut self) {
        self.last_x = self.x;
        self.last_y = self.y;
        CursorDir::save_cursor_pos();
    }

    fn ret_cursor_pos(&mut self) {
        self.x = self.last_x;
        self.y = self.last_y;
        CursorDir::ret_cursor_pos();
    }

    fn before_origin(&self) -> bool {
        self.y > 0 && self.x > 0
    }

    fn past_graph(&self) -> bool {
        self.x > self.graph_size as i16 
    }

	fn move_cursor(&mut self) {
		let dir = self.cursor_dir.unwrap();
	    match dir {
	        CursorDir::Up(x) => self.y -= x,
	        CursorDir::Down(x) => self.y += x,
	        CursorDir::Right(x) => self.x += x,
	        CursorDir::Left(x) => self.x -= x,
	    }
	    CursorDir::move_cursor(dir);
	}

	fn move_cur(&mut self, dir: CursorDir) {
	    match dir {
	        CursorDir::Up(x) => self.y -= x,
	        CursorDir::Down(x) => self.y += x,
	        CursorDir::Right(x) => self.x += x,
	        CursorDir::Left(x) => self.x -= x,
	    }
	    CursorDir::move_cursor(dir);
	}

	fn print(&mut self, c: char) {
        if self.before_origin() {
		  print!("{}", c);
        }
		self.x += 1;
	}
}

#[derive(Copy, Clone, PartialEq)]
enum CursorDir {
    Up(i16),
    Down(i16),
    Right(i16),
    Left(i16),
}

impl CursorDir {
	fn move_cursor(self) {
	    print!("{}", self);
	}

	#[inline(always)]
	fn save_cursor_pos() {
	    print!("\x1B7");
	}

	#[inline(always)]
	fn ret_cursor_pos() {
	    print!("\x1B8");
	}
}


impl fmt::Display for CursorDir {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
	        CursorDir::Up(spaces) => {
                if spaces < &0 {
                    write!(f, "\x1B[0A")
                } else {
                    write!(f, "\x1B[{}A", spaces)
                }
            },
	        CursorDir::Down(spaces) => {
                if spaces < &0 {
                    write!(f, "\x1B[0B")
                } else {
                    write!(f, "\x1B[{}B", spaces)
                }
            },
	        CursorDir::Right(spaces) => {
                if spaces < &0 {
                    write!(f, "\x1B[0C")
                } else {
                    write!(f, "\x1B[{}C", spaces)
                }
            },
	        CursorDir::Left(spaces) => {
                if spaces < &0 {
                    write!(f, "\x1B[0D")
                } else {
                    write!(f, "\x1B[{}D", spaces)
                }
            }
	    }
	}
}

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16
}


fn main() {
    let mut array_vec: arrayvec::ArrayVec<[i32; 25]> = arrayvec::ArrayVec::new();
    for _ in 0..array_vec.capacity() {
        array_vec.push(rand::thread_rng().gen_range(1, 10));
    }
    let nums: [i32; 25] = array_vec.into_inner().unwrap();

    // let nums: [i32; 14] = [2, 3, 8, 6, 2, 5, 1, 8, 4, 6, 9, 1, 7, 3];

    let mut cursor = prep_graph(&nums);

    let graph_height = cursor.y as i32;
    let mut x = 0; // prevents too many calculations from happening by only taking a limited slice (based on what's visible in frame)
    let mut tt = 0; // the current time dilation (0 - HORIZ_SPACING)

    for t in 0..60 {
        clear_graph(&graph_height);

        // Calculate current time dilation
        tt = t%HORIZ_SPACING;

        // Calculate the starting index of the array which should be used to form the current slice
        x = (t as i16 / HORIZ_SPACING as i16);

        // time dilation should range from 0 to HORIZ_SPACING inclusive.  This is a hacky way around this.
        if t > HORIZ_SPACING && t%HORIZ_SPACING == 0 {
            x -= 1;
            tt = HORIZ_SPACING; 
        }

        draw_graph(&mut cursor, &nums[x as usize..], &graph_height, tt);
        println!();
        cursor.ret_cursor_pos();

        thread::sleep(time::Duration::from_millis(200));
    }

    println!();
}

fn prep_graph(arr: &[i32]) -> Cursor {
    // let min = find_min(arr);
    let max = find_max(arr);

    let y_height = max+2;

    // TODO: use this to handle floats and larger ranges
    // let line_height = (y_height as f32/GRAPH_HEIGHT as f32) as i32;

    // PRINT THE BOUNDARIES OF THE GRAPH
    println!();

    for _ in 0..y_height {
        println!("|");
    }


    let sz = if GRAPH_WIDTH == 1 {
        arr.len() * HORIZ_SPACING
    } else {
        GRAPH_WIDTH * HORIZ_SPACING
    };

    print!("+");
    for _ in 0..sz {
        print!("-");
    }
    println!();

    // Move the cursor to the beginning of the previous line,
    //   drawing occurs relative to the bottom of the graph.
    CursorDir::move_cursor(CursorDir::Up(1));

    let cursor = Cursor{x: 0, y: y_height as i16, last_x: 0, last_y: 0, graph_size: sz, cursor_dir: None};

    return cursor
}

fn clear_graph(graph_height: &i32) {
    CursorDir::move_cursor(CursorDir::Up(((*graph_height) as i16)-2));
    for _ in 0..(*graph_height)-2 {
        print!("\x1B[0K|");
        CursorDir::move_cursor(CursorDir::Down(1));
        CursorDir::move_cursor(CursorDir::Left(1));
    }
}

fn draw_graph(cursor: &mut Cursor, arr: &[i32], graph_height: &i32, time_delta: usize) {
    for (i, &y) in arr.iter().enumerate() {
        cursor.save_cursor_pos();

        // Move up y lines, move right i spaces, print num
        cursor.move_cur(CursorDir::Up(y as i16));
        cursor.move_cur(CursorDir::Right((HORIZ_SPACING*(i)+X_BUFFER) as i16 - time_delta as i16));

        if cursor.past_graph() {
            cursor.ret_cursor_pos();
            break;
        }

        cursor.print(POINT_CHAR); 
        
        if i < arr.len() - 1 {
            let p2 = Point{x: ((HORIZ_SPACING*(i+1)+X_BUFFER) as i16 - time_delta as i16), y: (graph_height-arr[i+1]) as i16};
            draw_lines(cursor, p2);
        }

        cursor.ret_cursor_pos();
    }
    print!("\x1B[2B");
}


fn draw_lines(cursor: &mut Cursor, next_point: Point) {
    let dx = HORIZ_SPACING as i16;
    let dy = (next_point.y as i8 - cursor.y as i8) as i8;
    let distance = ((((dx as i32).pow(2) + (dy as i32).pow(2)) as f64).sqrt()) as i16;
    
    cursor.cursor_dir = if dy == 0 {
        None
    } else if dy > 0 {
        Some(CursorDir::Down(1)) // Move cursor down 1 line
    } else {
        Some(CursorDir::Up(1)) // Move cursor up 1 line
    };

    for i in 0..distance-1 {
        // Curve the line at the halfway point and at each third
        //   by moving the cursor left one space. Will only do this
        //   if y difference is larger than x difference
        if i != 0  && 
           (dy as i8).abs() > dx as i8 && 
           (i % (distance as f32/3.0) as i16 == 0 ||
           i % (distance as f32/2.0) as i16 == 0) {
           	cursor.move_cur(CursorDir::Left(1));
        }

        // Curve the line at the halfway point if it's longer 
        //   horizontally than vertically and if its
        //   direction is down.
        if i != 0  && dy > 0 &&
           (dx as i8) > (1.25*dy as f32) as i8 && 
           i-1 % (distance as f32/2.0) as i16 == 0 {
           	cursor.move_cur(CursorDir::Up(1));
        }

        // Curve the line at the halfway point if it's longer 
        //   horizontally than vertically and if its
        //   direction is up.
        if i != 0  && dy < 0 &&
           (-1*dx as i8) < (1.25*dy as f32) as i8 && 
           i-1 % (distance as f32/2.0) as i16 == 0 {
           	cursor.move_cur(CursorDir::Down(1));
        }

        cursor.print(LINE_MARK);

        match cursor.cursor_dir {
        	Some(_) => cursor.move_cursor(),
        	_ => {},
        }

        if cursor.past_graph() {
            break;
        }

        // If we preemptively reached the next point, no point
        //  in continuing.
        if (dy < 0 && cursor.y <= next_point.y && cursor.x >= next_point.x) || 
           (dy > 0 && cursor.y >= next_point.y && cursor.x >= next_point.x) || 
        	cursor.x > next_point.x {
            break;
        }

        // ensure the cursor doesn't print past its target
        //  by moving the cursor 
        //  and cursor tracker (cursor.y)
        match cursor.cursor_dir {
        	None      => {},
        	Some(dir) => {
        		if dir == CursorDir::Up(1) && 
		           cursor.x != next_point.x && 
		           cursor.y < next_point.y {
		            cursor.move_cur(CursorDir::Down(1));
		        } else if dir == CursorDir::Down(1) && 
		          cursor.x != next_point.x && 
		          cursor.y > next_point.y {
		            cursor.move_cur(CursorDir::Up(1));
		        }
        	},
        }
    }
}

fn find_min(arr: &[i32]) -> i32 {
    let mut min = arr[0];
    for &x in arr.iter().skip(1) {
        if x < min {
            min = x;
        }
    }
    min
}


fn find_max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &x in arr.iter().skip(1) {
        if x > max {
            max = x;
        }
    }
    max
}