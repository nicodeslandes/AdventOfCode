use crate::Direction;
use crate::Pos;

pub struct NextMoveIterator {
    next_direction: Option<Direction>,
    origin: Pos,
}

impl NextMoveIterator {
    pub fn new(pos: Pos) -> NextMoveIterator {
        NextMoveIterator {
            origin: pos,
            next_direction: Some(Direction::Up),
        }
    }
}

impl Iterator for NextMoveIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        loop {
            match &self.next_direction {
                None => break None,
                Some(d) => {
                    let Pos(x, y) = self.origin;
                    match d {
                        Direction::Up => {
                            self.next_direction = Some(Direction::Right);
                            if y > 0 {
                                break Some(Pos(x, y - 1));
                            }
                        }
                        Direction::Right => {
                            self.next_direction = Some(Direction::Bottom);
                            break Some(Pos(x + 1, y));
                        }
                        Direction::Bottom => {
                            self.next_direction = Some(Direction::Left);
                            break Some(Pos(x, y + 1));
                        }
                        Direction::Left => {
                            self.next_direction = None;
                            if x > 0 {
                                break Some(Pos(x - 1, y));
                            }
                        }
                    };
                }
            }
        }
    }
}
