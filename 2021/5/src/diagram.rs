use std::default::Default;
use std::fmt;
use std::cmp::{min, max};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct HydrothermalDiagram {
     light_matrix: HashMap<(usize, usize), usize>,
}

impl HydrothermalDiagram {
     fn update_coords(&mut self, x: usize, y: usize) {
          if let Some(coord_total) = self.light_matrix.get_mut(&(x, y)) {
               *coord_total += 1;
          } else {
               self.light_matrix.insert((x, y), 1);
          }
     }

     fn matrix_size(&self) -> (usize, usize) {
          let x_len = self.light_matrix.keys().map(|(x, _y)| x).max().unwrap_or(&0);
          let y_len = self.light_matrix.keys().map(|(_x, y)| y).max().unwrap_or(&0);
          return (*x_len + 1, *y_len + 1);
     }

     fn get_coords_total(&self, x: usize, y: usize) -> usize {
          if let Some(coord_total) = self.light_matrix.get(&(x, y)) {
               return *coord_total;
          } else {
               return 0;
          }
     }

     pub fn add_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
          if x1 == x2 {
               for y in min(y1, y2)..=max(y1, y2) {
                    self.update_coords(x1, y);
               }
          } else if y1 == y2 {
               for x in min(x1, x2)..=max(x1, x2) {
                    self.update_coords(x, y1);
               }
          } else {
               let steps_x = max(x1, x2)-min(x1, x2);
               let steps_y = max(y1, y2)-min(y1, y2);
               if steps_x == steps_y && steps_x > 1 {
                    for step in 0..=steps_x {
                         let x = if x1 < x2 { x1 + step } else { x1 - step };
                         let y = if y1 < y2 { y1 + step } else { y1 - step };
                         self.update_coords(x, y);
                    }
               }
          }
     }

     pub fn total_danger_points(&self) -> usize {
          let mut total = 0;
          for (_coords, coords_total) in &self.light_matrix {
               if *coords_total > 1 {
                    total += 1;
               }
          }
          return total;
     }

     pub fn x_len(&self) -> usize {
          let (x_len, _y_len) = self.matrix_size();
          return x_len;
     }
}

impl fmt::Display for HydrothermalDiagram {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          let (x_len, y_len) = self.matrix_size();
          for y in 0..y_len {
               for x in 0..x_len {
                    let coords_total = self.get_coords_total(x, y);
                    if coords_total == 0 {
                         write!(f, ".")?
                    } else {
                         write!(f, "{}", coords_total)?
                    }
               }
               write!(f, "\n")?
          }
          return Ok(());
     }
}
