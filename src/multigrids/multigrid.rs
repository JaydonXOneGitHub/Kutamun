use std::{cell::RefCell, rc::{Rc, Weak}};

use vector_x::Vector3;

use crate::{Direction, Grid, NavRes, multigrids::internalmultigrid::InternalMultiGrid};

pub struct MultiGrid<GridButton> {
    internal_grid: Rc<RefCell<InternalMultiGrid<GridButton>>>
}

impl<GridButton> MultiGrid<GridButton> {
    pub fn new() -> Self {
        return Self {
            internal_grid: Rc::new(RefCell::new(InternalMultiGrid::new()))
        };
    }
}

impl<GridButton> MultiGrid<GridButton> {
    pub fn with_grid(self, grid: (usize, Grid<GridButton>)) -> Self {
        if let Result::Ok(mut img) = self.internal_grid.try_borrow_mut() {
            if !img.grids.contains_key(&grid.0) {
                img.grids.insert(grid.0, grid.1);
            }
        }

        return self;
    }

    pub fn with_selected_grid(self, selection: usize) -> Self {
        if let Result::Ok(mut img) = self.internal_grid.try_borrow_mut() {
            if img.grids.contains_key(&selection) {
                img.current_grid = Option::Some(selection);
            } else {
                eprintln!("Selection with index {} is invalid at this time!", selection);
            }
        }
        
        return self;
    }
}

impl<GridButton> MultiGrid<GridButton> {
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
        let res = self.internal_grid.try_borrow_mut();

        if res.is_err() {
            eprintln!("Error: {}", res.err().unwrap().to_string());
            return;
        }

        let mut img = res.unwrap();
        
        img.move_focus(direction, callback);
    }
}

impl<GridButton> MultiGrid<GridButton> {
    pub fn insert_grid(
        &self, 
        index: usize, 
        grid: Grid<GridButton>
    ) -> NavRes<Option<Grid<GridButton>>, String> {
        return match self.internal_grid.try_borrow_mut() {
            Result::Ok(mut img) => {
                NavRes::OK(img.insert_grid(index, grid))
            },
            Result::Err(e) => NavRes::Error(e.to_string())
        };
    }
    
    pub fn get_internal(&self) -> Weak<RefCell<InternalMultiGrid<GridButton>>> {
        return Rc::downgrade(&self.internal_grid);
    }

    pub fn get_internal_ref(&self) -> &Rc<RefCell<InternalMultiGrid<GridButton>>> {
        return &self.internal_grid;
    }
}

impl<GridButton> Clone for MultiGrid<GridButton> {
    fn clone(&self) -> Self {
        return Self {
            internal_grid: self.internal_grid.clone()
        };
    }
}