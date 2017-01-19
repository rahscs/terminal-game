extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::str::from_utf8;

const PLAYER : char = '@';
const BEGIN : char = '\\';
const FLOOR : char = ' ';
const WALL : char = '#';
const CAT : char = '%';
const EEL : char = 'e';
const TREX : char = 't';
const ANTEATER : char = 'a';
const DOOR : char = '!';

enum Direction {
	Left, Right, Up, Down,
}

fn backup(pos: &mut (u16, u16), dir: Direction) {
	match dir {
		Direction::Left => pos.0 += 1,
		Direction::Right => pos.0 -= 1,
		Direction::Up => pos.1 += 1,
		Direction::Down => pos.1 -= 1,
	}
}

fn redraw(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
	world: &mut [u8], pos: &mut (u16, u16), dir: Direction, fight: u8)
	-> bool
{
	let mut found_it = false;
	let mut floor_msg = "Step ...  Step ...        ";
	// Clear first line.
	if fight == 2 {
		floor_msg = "";
//		write!(stdout, "{}{}", termion::cursor::Goto(1, 2),
//			termion::clear::CurrentLine).unwrap();
	}else{
		write!(stdout, "{}{}", termion::cursor::Goto(1, 1),
			termion::clear::CurrentLine).unwrap();
	}
	// Go to 3rd line to draw world
	write!(stdout, "{}", termion::cursor::Goto(1, 3)).unwrap();
	let mut linedown = 2; // Initial y
	let mut lineright = 1; // Initial x
	let mut selected : char = ' ';
	// Draw world
	for i in 0..world.len() {
		if world[i] == ('\n' as u8) {
			linedown += 1;
			lineright = 1;
			write!(stdout, "{}",
				termion::cursor::Goto(1, linedown + 1)).unwrap();
		} else {
			if pos.0 == lineright && pos.1 == linedown {
				if fight == 1 {
					match world[i] as char {
						CAT => {
							found_it = true;
							world[i]=' ' as u8;
							floor_msg = "kill cat";
						},
						EEL => {
							found_it = true;
							world[i]=' ' as u8;
							floor_msg = "kill eel";
						}
						TREX => {
							found_it = true;
							world[i]=' ' as u8;
							floor_msg = "kill trex";
						}
						ANTEATER => {
							found_it = true;
							world[i]=' ' as u8;
							floor_msg =
								"kill anteater";
						}
						_ => {},
					}
				}
				selected = world[i] as char;
			}
			write!(stdout, "{}", world[i] as char);
			lineright += 1;
		}
	}

	// Write details for block.
	let stringy = format!("selected : {}", selected);
	write!(stdout, "{}{}", termion::cursor::Goto(1, 1), match selected {
		BEGIN => "Hello, Welcome to RAHSCS Terminal Game!",
		FLOOR => floor_msg,
		DOOR => {
			if fight == 0 {
				backup(pos, dir);
			}
			"The door is locked!"
		},
		WALL => {
			if fight == 0 {
				backup(pos, dir);
			}
			"There's a wall there!"
		},
		CAT => {
			backup(pos, dir);
			"The cat attacks you!"
		},
		EEL => {
			backup(pos, dir);
			"The eel strangles you!"
		},
		TREX => {
			backup(pos, dir);
			"The trex kicks you!"
		},
		ANTEATER => {
			backup(pos, dir);
			"The anteater snorts you!"
		},
		_ => &stringy,
	}).unwrap();
	write!(stdout, "{}{}", termion::cursor::Goto(pos.0, pos.1 + 1), PLAYER)
		.unwrap();
	found_it
}

fn main() {
	let mut world = from_utf8(include_bytes!("levela.text")).unwrap().as_bytes().to_owned();
	// Get the standard input stream.
	let stdin = stdin();
	// Get the standard output stream and go to raw mode.
	let mut stdout = stdout().into_raw_mode().unwrap();
	//
	let mut pos : (u16, u16) = (1, 2);

	write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide)
		.unwrap();
	redraw(&mut stdout, &mut world, &mut pos, Direction::Left, 0);

	// Update terminal screen.
	stdout.flush().unwrap();

	// Main loop
	for c in stdin.keys() {
		// Print any ke that's pressed.
		match c.unwrap() {
			Key::Char('q') => break, // Quit
			Key::Char('f') => {
				let mut tmppos : (u16, u16);
				tmppos = (pos.0 - 1, pos.1);
				let a = redraw(&mut stdout, &mut world,
					&mut tmppos, Direction::Left, 1);
				if a == false {
				tmppos = (pos.0 + 1, pos.1);
				let a = redraw(&mut stdout, &mut world,
					&mut tmppos, Direction::Right, 1);
				if a == false {
				tmppos = (pos.0, pos.1 - 1);
				let a = redraw(&mut stdout, &mut world,
					&mut tmppos, Direction::Up, 1);
				if a == false {
				tmppos = (pos.0, pos.1 + 1);
				redraw(&mut stdout, &mut world,
					&mut tmppos, Direction::Down, 1);
				}
				}
				}
				redraw(&mut stdout, &mut world, &mut pos,
					Direction::Left, 2);
			},
//			Key::Alt(c)    => println!("Alt-{}", c),
//			Key::Ctrl(c)   => println!("Ctrl-{}", c),
			Key::Left      => {
				if pos.0 > 1 {
					pos.0 -= 1;
					redraw(&mut stdout, &mut world, &mut pos,
						Direction::Left, 0);
				}
			},
			Key::Right     => {
				if pos.0 < 32 {
					pos.0 += 1;
					redraw(&mut stdout, &mut world, &mut pos,
						Direction::Right, 0);
				}
			},
			Key::Up        => {
				if pos.1 > 2 {
					pos.1 -= 1;
					redraw(&mut stdout, &mut world, &mut pos,
						Direction::Up, 0);
				}
			},
			Key::Down      => {
				if pos.1 < 9 {
					pos.1 += 1;
					redraw(&mut stdout, &mut world, &mut pos,
						Direction::Down, 0);
				}
			},
			_              => {},
		}

		// Update terminal screen.
		stdout.flush().unwrap();
	}

	// Unhide Cursor
	write!(stdout, "{}{}{}", termion::clear::All,
		termion::cursor::Goto(1, 1), termion::cursor::Show).unwrap();
}
