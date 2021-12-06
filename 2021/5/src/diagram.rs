use std::default::Default;
use std::fmt;
use std::cmp::{min, max};

#[derive(Default, Debug)]
pub struct HydrothermalDiagram {
     matrix: Vec<Vec<usize>>,
}

impl HydrothermalDiagram {
     fn expand_to(&mut self, x_len: usize, y_len: usize) {
          while self.matrix.len() < y_len {
               self.matrix.push(Vec::new());
          }
          for row in &mut self.matrix {
               while row.len() < x_len {
                    row.push(0);
               }
          }
     }

     pub fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
          self.expand_to(max(x1, x2) + 1, max(y1, y2) + 1);
          if x1 == x2 {
               for y in min(y1, y2)..=max(y1, y2) {
                    self.matrix[y][x1] += 1;
               }
          } else if y1 == y2 {
               for x in min(x1, x2)..=max(x1, x2) {
                    self.matrix[y1][x] += 1;
               }
          }
     }

     pub fn total_danger_points(&self) -> usize {
          let mut total = 0;
          for row in &self.matrix {
               for col in row {
                    if *col > 1 {
                         total += 1;
                    }
               }
          }
          return total;
     }

     pub fn x_len(&self) -> usize {
          self.matrix[0].len()
     }
}

impl fmt::Display for HydrothermalDiagram {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          for row in &self.matrix {
               for col in row {
                    if *col == 0 {
                         write!(f, ".")?
                    } else {
                         write!(f, "{}", col)?
                    }
               }
               write!(f, "\n")?
          }
          return Ok(());
     }
}
