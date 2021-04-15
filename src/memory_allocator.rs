use colored::*;
use prettytable::{Cell, Row, Table};
use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

pub enum MemoryAccess {
    Hit(i32),
    Miss(i32),
}

pub struct MemoryAccesses {
    pub accesses: Vec<MemoryAccess>,
}

impl MemoryAccesses {
    pub fn hits(&self) -> usize {
        self.accesses
            .iter()
            .filter(|access| match access {
                MemoryAccess::Hit(_) => true,
                _ => false,
            })
            .count()
    }

    pub fn misses(&self) -> usize {
        self.accesses
            .iter()
            .filter(|access| match access {
                MemoryAccess::Miss(_) => true,
                _ => false,
            })
            .count()
    }
}

impl fmt::Display for MemoryAccesses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (pos, access) in self.accesses.iter().enumerate() {
            let out = match access {
                MemoryAccess::Hit(value) => value.to_string().green(),
                MemoryAccess::Miss(value) => value.to_string().red(),
            };

            if pos == self.accesses.len() - 1 {
                write!(f, "{}", out)?;
            } else {
                write!(f, "{} ", out)?;
            }
        }

        Ok(())
    }
}

pub trait MemoryAllocator {
    fn new(frames: usize, accesses: usize) -> Self;
    fn run(&mut self, accesses: &Vec<i32>) -> MemoryAccesses;
    fn gen_table(&self) -> Table;
}

pub struct LRU {
    frame_size: usize,
    frames: Vec<Vec<i32>>,
    deque: VecDeque<i32>,
}

fn gen_table(frame_size: usize, frames: &Vec<Vec<i32>>) -> Table {
    let mut table = Table::new();
    for index in 0..frame_size {
        table.add_row(Row::new(
            frames
                .iter()
                .map(|data| {
                    Cell::new(
                        &data
                            .get(index)
                            .map(|num| num.to_string())
                            .unwrap_or(String::from("")),
                    )
                })
                .collect(),
        ));
    }

    table
}

impl MemoryAllocator for LRU {
    fn new(frames: usize, accesses: usize) -> Self {
        Self {
            frame_size: frames,
            frames: (0..accesses).map(|_| Vec::with_capacity(frames)).collect(),
            // frames: (0..frames).map(|_| Vec::with_capacity(accesses)).collect(),
            deque: VecDeque::new(),
        }
    }

    fn run(&mut self, accesses: &Vec<i32>) -> MemoryAccesses {
        let mut hits_or_misses = Vec::new();

        for (position, &access) in accesses.iter().enumerate() {
            // Already has it
            if position > 0 && self.frames[position - 1].contains(&access) {
                self.frames[position] = self.frames[position - 1].clone();
                self.deque.retain(|&value| value != access);

                hits_or_misses.push(MemoryAccess::Hit(access));
            } else if position > 0 && self.frames[position - 1].len() == self.frame_size {
                let least_recently_used = self.deque.pop_back().unwrap();
                let least_recently_used_position = self.frames[position - 1]
                    .iter()
                    .position(|&item| item == least_recently_used)
                    .unwrap();
                self.frames[position] = self.frames[position - 1].clone();
                self.frames[position][least_recently_used_position] = access;

                hits_or_misses.push(MemoryAccess::Miss(access));
            } else {
                // Can just add it
                if position > 0 {
                    self.frames[position] = self.frames[position - 1].clone();
                }
                self.frames[position].push(access);

                hits_or_misses.push(MemoryAccess::Miss(access));
            }

            self.deque.push_front(access);
        }

        MemoryAccesses {
            accesses: hits_or_misses,
        }
    }

    fn gen_table(&self) -> Table {
        gen_table(self.frame_size, &self.frames)
    }
}

pub struct FIFO {
    frame_size: usize,
    frames: Vec<Vec<i32>>,
    deque: VecDeque<i32>,
}

impl MemoryAllocator for FIFO {
    fn new(frames: usize, accesses: usize) -> Self {
        Self {
            frame_size: frames,
            frames: (0..accesses).map(|_| Vec::with_capacity(frames)).collect(),
            deque: VecDeque::new(),
        }
    }

    fn run(&mut self, accesses: &Vec<i32>) -> MemoryAccesses {
        let mut hits_or_misses = Vec::new();

        for (position, &access) in accesses.iter().enumerate() {
            // Already has it
            if position > 0 && self.frames[position - 1].contains(&access) {
                self.frames[position] = self.frames[position - 1].clone();
                hits_or_misses.push(MemoryAccess::Hit(access));
            } else if position > 0 && self.frames[position - 1].len() == self.frame_size {
                let least_recently_used = self.deque.pop_back().unwrap();

                let least_recently_used_position = self.frames[position - 1]
                    .iter()
                    .position(|&item| item == least_recently_used)
                    .unwrap();

                self.frames[position] = self.frames[position - 1].clone();
                self.frames[position][least_recently_used_position] = access;

                hits_or_misses.push(MemoryAccess::Miss(access));
                self.deque.push_front(access);
            } else {
                // Can just add it
                if position > 0 {
                    self.frames[position] = self.frames[position - 1].clone();
                }
                self.frames[position].push(access);

                hits_or_misses.push(MemoryAccess::Miss(access));
                self.deque.push_front(access);
            }
        }

        MemoryAccesses {
            accesses: hits_or_misses,
        }
    }

    fn gen_table(&self) -> Table {
        gen_table(self.frame_size, &self.frames)
    }
}

pub struct OPT {
    frame_size: usize,
    frames: Vec<Vec<i32>>,
}

impl MemoryAllocator for OPT {
    fn new(frames: usize, accesses: usize) -> Self {
        Self {
            frame_size: frames,
            frames: (0..accesses).map(|_| Vec::with_capacity(frames)).collect(),
        }
    }

    // O(n^2) runtime (excluding clones and stuff) (could be better)
    fn run(&mut self, accesses: &Vec<i32>) -> MemoryAccesses {
        let mut hits_or_misses = Vec::new();

        for (position, &access) in accesses.iter().enumerate() {
            // Already has it
            if position > 0 && self.frames[position - 1].contains(&access) {
                self.frames[position] = self.frames[position - 1].clone();

                hits_or_misses.push(MemoryAccess::Hit(access));
            } else if position > 0 && self.frames[position - 1].len() == self.frame_size {
                let furthest_access_position = {
                    // access -> index
                    let mut not_used: HashMap<i32, usize> = self.frames[position - 1]
                        .clone()
                        .into_iter()
                        .enumerate()
                        .map(|(index, access)| (access, index))
                        .collect();

                    let mut to_remove = 0;

                    for future in (position + 1)..accesses.len() {
                        let future_item = accesses[future];
                        match (not_used.get(&future_item), not_used.len()) {
                            (Some(&index), size) if size == 1 => {
                                to_remove = index;
                                break;
                            }
                            (Some(_), _) => {
                                not_used.remove(&future_item);
                            }
                            _ => {}
                        }
                    }

                    if not_used.len() > 1 {
                        // Replace on the first position if there are multiple frames that are not used.
                        let mut not_used_list = not_used.values().collect::<Vec<_>>();
                        not_used_list.sort();
                        *not_used_list[0]
                    } else {
                        to_remove
                    }
                };

                self.frames[position] = self.frames[position - 1].clone();
                self.frames[position][furthest_access_position] = access;

                hits_or_misses.push(MemoryAccess::Miss(access));
            } else {
                // Can just add it
                if position > 0 {
                    self.frames[position] = self.frames[position - 1].clone();
                }
                self.frames[position].push(access);

                hits_or_misses.push(MemoryAccess::Miss(access));
            }
        }

        MemoryAccesses {
            accesses: hits_or_misses,
        }
    }

    fn gen_table(&self) -> Table {
        gen_table(self.frame_size, &self.frames)
    }
}
