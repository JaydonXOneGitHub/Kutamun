# About
This library came about when I was utilizing my [Iced Spatial Navigation](https://github.com/JaydonXOneGitHub/iced_spatial_navigation) crate<br>
in my original draft of my smart TV Linux distro interface.<br><br>

I had come to notice, however, that it was going to be complicated to<br>
handle images within Iced, thanks to its declarative nature.<br><br>

I then decided to utilize an imperative GUI library, and needed<br>
a version of ISN that would work anywhere.<br><br>

# How to Use
First, in your terminal, type:<br>
```bash
cargo add kutamun
```
Second, for a basic example, do something like this:
```rust
use kutamun::{MultiGrid, Grid};

pub struct Test {
  msg: String
}

// This is where your navigation logic goes.
// Think about your implementation carefully
// to prevent it from going all over the place.
fn callback(
    dir: Direction,
    old_pos: Vector3<usize>
) -> Vector3<usize> {
    Vector3::default()
}

fn main() {
  let multi: MultiGrid<Test> = MultiGrid::new()
  .with_grid((0, Grid::new())) // the (usize, Grid<T>) pair is for indexing
  .with_selected_grid(0) // This automatically selects the corresponding Grid if it's there
  .with_navigation_callback(Some(Box::new(callback))); // This assigns navigation callback (must be set for navigation)

  // Fill out the rest of your logic here
}
```

# Features
Thanks to its architecture, it allows for lightweight and O(1) layout search.<br>
Rather than AABB, it utilizes index searching via a `HashSet<usize, Vec<Vec<T>>>` under the hood.<br>
