use anyhow::{anyhow, bail, Result};

trait MemoryReplacement {
    fn get_page_to_remove(&self) -> Option<i32>;
    fn add(&mut self, access: i32) -> Result<()>;
    fn replace(&mut self, old_value: i32, new_value: i32) -> Result<()>;
    fn upsert(&mut self, value: i32) -> Result<()>;
    // fn run(self) -> Result<>;
}

// pub struct MemoryReplacement {
//     frame_size: usize,
//     frames: Vec<Vec<i32>>,
//     position: usize, // deque: VecDeque<i32>,

//                      // fn new(frames: usize, accesses: usize) -> Self;
//                      // fn run(&mut self, accesses: &Vec<i32>) -> MemoryAccesses;
//                      // fn gen_table(&self) -> Table;
// }

// impl MemoryReplacement {
//     // fn get_page_to_remove(&self) -> Option<i32> {}

//     fn get_page_to_remove(&self) -> Option<usize> {

//     }

//     fn add(&mut self, access: i32) -> Result<()> {
//         self.frames[self.position].push(access);

//         if self.frames[self.position].len() > self.frame_size {
//             bail!("Invalid frame add");
//         }

//         Ok(())
//     }

//     // fn replace(&mut self, old_value: i32, new_value: i32) -> Result<()> {
//     //     let old_pos = self.frames[self.position]
//     //         .iter()
//     //         .position(|&value| value == old_value)
//     //         .ok_or(anyhow!(
//     //             "Can not replace \"{}\" since it's value was not found in frame.",
//     //             old_value
//     //         ))?;

//     //     self.frames[old_pos][old_pos] = new_value;

//     //     Ok(())
//     // }

//     fn replace(&mut self, old_index: usize, new_value: i32) -> Result<()> {
//         self.frames[self.position][old_index] = new_value;

//         Ok(())
//     }

//     fn upsert(&mut self, value: i32) -> Result<()> {
//         if !self.frames[self.position].contains(&value) {
//             if self.frames[self.position].len() == self.frame_size {
//                 // Replace something
//                 let to_remove = self.get_page_to_remove().unwrap();
//                 self.replace(to_remove, value);
//             } else {
//                 // We can just add it
//                 self.frames[self.position].push(value);
//             }
//         }

//         Ok(())
//     }

//     // fn run(self) -> MemoryAccesses {}
// }
