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

struct Context {
	world: std::vec::Vec<u8>,
	stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

fn backup(pos: &mut (u16, u16), dir: Direction) {
	match dir {
		Direction::Left => pos.0 += 1,
		Direction::Right => pos.0 -= 1,
		Direction::Up => pos.1 += 1,
		Direction::Down => pos.1 -= 1,
	}
}

fn redraw(context: &mut Context, pos: &mut (u16, u16), dir: Direction,
	fight: u8) -> bool
{
	let mut found_it = false;
	let mut floor_msg = "Step ...  Step ...        ";
	// Clear first line.
	if fight == 2 {
		floor_msg = "";
		write!(context.stdout, "{}{}", termion::cursor::Goto(1, 2),
			termion::clear::CurrentLine).unwrap();
	}else if fight == 3 {
		write!(context.stdout, "{}{}{}{}", termion::cursor::Goto(1, 1),
			termion::clear::CurrentLine,termion::cursor::Goto(1, 2),
			termion::clear::CurrentLine).unwrap();
		floor_msg = "There is nothing to fight!";
	}else{
		write!(context.stdout, "{}{}", termion::cursor::Goto(1, 1),
			termion::clear::CurrentLine).unwrap();
	}
	// Go to 3rd line to draw world
	write!(context.stdout, "{}", termion::cursor::Goto(1, 3)).unwrap();
	let mut linedown = 2; // Initial y
	let mut lineright = 1; // Initial x
	let mut selected : char = ' ';
	// Draw world
	for i in 0..context.world.len() {
		if context.world[i] == ('\n' as u8) {
			linedown += 1;
			lineright = 1;
			write!(context.stdout, "{}",
				termion::cursor::Goto(1, linedown + 1)).unwrap();
		} else {
			if pos.0 == lineright && pos.1 == linedown {
				if fight == 1 {
					match context.world[i] as char {
						CAT => {
							found_it = true;
							context.world[i]=' ' as u8;
							floor_msg = "kill cat";
						},
						EEL => {
							found_it = true;
							context.world[i]=' ' as u8;
							floor_msg = "kill eel";
						}
						TREX => {
							found_it = true;
							context.world[i]=' ' as u8;
							floor_msg = "kill trex";
						}
						ANTEATER => {
							found_it = true;
							context.world[i]=' ' as u8;
							floor_msg =
								"kill anteater";
						}
						_ => {},
					}
				}
				selected = context.world[i] as char;
			}
			write!(context.stdout, "{}", context.world[i] as char)
				.unwrap();
			lineright += 1;
		}
	}

	// Write details for block.
	let stringy = format!("selected : {}", selected);
	write!(context.stdout, "{}{}", termion::cursor::Goto(1, 1), match selected {
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
	write!(context.stdout, "{}{}", termion::cursor::Goto(pos.0, pos.1 + 1),
		PLAYER).unwrap();
	found_it
}

fn main() {
	let mut context = Context {
		world: from_utf8(include_bytes!("levela.text")).unwrap()
			.as_bytes().to_owned(),
		stdout: stdout().into_raw_mode().unwrap(),
	};
	let mut pos = (1u16, 2u16);
	// Get the standard input stream.
	let stdin = stdin();

	write!(context.stdout, "{}{}", termion::clear::All,
		termion::cursor::Hide).unwrap();
	redraw(&mut context, &mut pos, Direction::Left, 0);

	// Update terminal screen.
	context.stdout.flush().unwrap();

	// Main loop
	for c in stdin.keys() {
		// Print any ke that's pressed.
		match c.unwrap() {
			Key::Char('q') => break, // Quit
			Key::Char('f') => {
				let mut tmppos : (u16, u16);
				tmppos = (pos.0 - 1, pos.1);
				let mut a = redraw(&mut context,
					&mut tmppos, Direction::Left, 1);
				if a == false {
				tmppos = (pos.0 + 1, pos.1);
				a = redraw(&mut context,
					&mut tmppos, Direction::Right, 1);
				if a == false {
				tmppos = (pos.0, pos.1 - 1);
				a = redraw(&mut context,
					&mut tmppos, Direction::Up, 1);
				if a == false {
				tmppos = (pos.0, pos.1 + 1);
				a = redraw(&mut context,
					&mut tmppos, Direction::Down, 1);
				}
				}
				}
				if a {
					redraw(&mut context, &mut pos,
						Direction::Left, 2);
				}else {
					redraw(&mut context, &mut pos,
						Direction::Left, 3);
				}
			},
			Key::Left      => {
				if pos.0 > 1 {
					pos.0 -= 1;
					redraw(&mut context, &mut pos,
						Direction::Left, 0);
				}
			},
			Key::Right     => {
				if pos.0 < 32 {
					pos.0 += 1;
					redraw(&mut context, &mut pos,
						Direction::Right, 0);
				}
			},
			Key::Up        => {
				if pos.1 > 2 {
					pos.1 -= 1;
					redraw(&mut context, &mut pos,
						Direction::Up, 0);
				}
			},
			Key::Down      => {
				if pos.1 < 9 {
					pos.1 += 1;
					redraw(&mut context, &mut pos,
						Direction::Down, 0);
				}
			},
			_              => {},
		}

		// Update terminal screen.
		context.stdout.flush().unwrap();
	}

	// Unhide Cursor
	write!(context.stdout, "{}{}{}", termion::clear::All,
		termion::cursor::Goto(1, 1), termion::cursor::Show).unwrap();
}
