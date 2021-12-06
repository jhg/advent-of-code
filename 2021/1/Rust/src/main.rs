use helpers::Opt;

use std::io;

mod depth;
use depth::SlidingWindowDepthMeasures;

fn main() -> io::Result<()> {
    let mut depth_measures_window = SlidingWindowDepthMeasures::default();
    let mut prev_depth_measure = None;
    let mut total_times_increase = 0;
    for line in Opt::from_args().lines()? {
        let depth_measure: usize = line.parse().expect("Can not parse the depth measurement");
        depth_measures_window.add(depth_measure);
        if !depth_measures_window.enough_len() {
            continue;
        }
        let measures_sum = depth_measures_window.measures_sum().expect("Without measures in the sliding window");
        if let Some(prev_depth_measure) = prev_depth_measure {
            if prev_depth_measure < measures_sum {
                println!("{} (increased)", measures_sum);
                total_times_increase += 1;
            } else if prev_depth_measure > measures_sum {
                println!("{} (decreased)", measures_sum);
            } else {
                println!("{} (keep)", measures_sum);
            }
        } else {
            println!("{} (N/A - no previous measurement)", measures_sum);
        }
        prev_depth_measure = Some(measures_sum);
    }
    println!("{}", total_times_increase);
    return Ok(());
}
