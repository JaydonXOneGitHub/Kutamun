use std::sync::{Arc, Mutex, Weak};

use vector_x::Vector3;

use crate::{Direction, Grid, NavRes, multigrids::InternalMultiGrid};

pub struct AtomicMultiGrid<GridButton> {
    internal_grid: Arc<Mutex<InternalMultiGrid<GridButton>>>
}

impl<GridButton> AtomicMultiGrid<GridButton> {
    pub fn new() -> Self {
        return Self {
            internal_grid: Arc::new(Mutex::new(
                InternalMultiGrid::new()
            ))
        };
    }
}

impl<GridButton> AtomicMultiGrid<GridButton> {
    pub fn with_grid(self, grid: (usize, Grid<GridButton>)) -> Self {
        match self.internal_grid.try_lock() {
            Result::Ok(mut img) => {
                if !img.grids.contains_key(&grid.0) {
                    img.grids.insert(grid.0, grid.1);
                }
            },
            Result::Err(e) => {
                eprintln!("Error: {}", e.to_string());
            }
        }

        return self;
    }

    pub fn with_selected_grid(self, selection: usize) -> Self {
        match self.internal_grid.try_lock() {
            Result::Ok(mut img) => {
                if img.grids.contains_key(&selection) {
                    img.current_grid = Option::Some(selection);
                } else {
                    eprintln!("Selection with index {} is invalid at this time!", selection);
                }
            },
            Result::Err(e) => {
                eprintln!("Error: {}", e.to_string());
            }
        }
        
        return self;
    }
}

impl<GridButton> AtomicMultiGrid<GridButton> {
    pub fn move_focus<F>(
        &self, 
        direction: Direction,
        callback: F
    ) 
    where 
        F: Fn(
            &InternalMultiGrid<GridButton>,
            Direction,
            Vector3<usize>
        )-> Vector3<usize>
    {
        let res = self.internal_grid.try_lock();

        if res.is_err() {
            eprintln!("Error: {}", res.err().unwrap().to_string());
            return;
        }

        let mut img = res.unwrap();
        
        img.move_focus(direction, callback);
    }
}

impl<GridButton> AtomicMultiGrid<GridButton> {
    pub fn insert_grid(
        &self, 
        index: usize, 
        grid: Grid<GridButton>
    ) -> NavRes<Option<Grid<GridButton>>, String> {
        return match self.internal_grid.try_lock() {
            Result::Ok(mut img) => {
                NavRes::OK(img.insert_grid(index, grid))
            },
            Result::Err(e) => NavRes::Error(e.to_string())
        };
    }
    
    pub fn get_internal(&self) -> Weak<Mutex<InternalMultiGrid<GridButton>>> {
        return Arc::downgrade(&self.internal_grid);
    }

    pub fn get_internal_ref(&self) -> &Arc<Mutex<InternalMultiGrid<GridButton>>> {
        return &self.internal_grid;
    }
}

impl<GridButton> Clone for AtomicMultiGrid<GridButton> {
    fn clone(&self) -> Self {
        return Self {
            internal_grid: self.internal_grid.clone()
        };
    }
}