use std::collections::HashMap;

use vector_x::{Vector2, Vector3};

use crate::{Direction, Grid, NavRes};

pub struct InternalMultiGrid<GridButton> {
    pub(crate) grids: HashMap<usize, Grid<GridButton>>,
    pub(crate) current_grid: Option<usize>
}

impl<GridButton> InternalMultiGrid<GridButton> {
    pub fn with_grid(mut self, grid: (usize, Grid<GridButton>)) -> Self {
        if self.grids.contains_key(&grid.0) {
            return self;
        }

        self.grids.insert(grid.0, grid.1);

        return self;
    }

    pub fn with_selected_grid(mut self, selection: usize) -> Self {
        if !self.grids.contains_key(&selection) {
            println!("Selection with index {} is invalid at this time!", selection);
            return self;
        }

        self.current_grid = Option::Some(selection);
        return self;
    }
}

impl<GridButton> InternalMultiGrid<GridButton> {
    pub fn new() -> Self {
        return Self {
            grids: HashMap::new(),
            current_grid: Option::None
        };
    }
}

impl<GridButton> InternalMultiGrid<GridButton> {
    pub fn get_grid(&self) -> NavRes<&Grid<GridButton>, String> {
        if self.current_grid.is_none() {
            return NavRes::Error("No grid has been selected!".into());
        }

        let cg: usize = self.current_grid.clone().unwrap();

        return match self.grids.get(&cg) {
            Option::Some(g) => NavRes::OK(g),
            Option::None => NavRes::Error("Grid not found!".into())
        };
    }
}

impl<GridButton> InternalMultiGrid<GridButton> {
    pub fn move_focus<F>(
        &mut self, 
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
        if self.current_grid.is_none() {
            return;
        }

        let cg1: usize = self.current_grid.clone().unwrap();
        let o1 = self.grids.get(&cg1);

        let grid1: &Grid<GridButton> = o1.unwrap();

        let pos = callback(
            &self,
            direction, 
            Vector3::new(
                cg1, 
                grid1.get_position().one, 
                grid1.get_position().two
            )
        );

        self.current_grid = Option::Some(pos.one);

        let cg2: usize = self.current_grid.clone().unwrap();
        let o2 = self.grids.get_mut(&cg2);

        let grid2: &mut Grid<GridButton> = o2.unwrap();

        grid2.set_position(Vector2::new(pos.two, pos.three));
    }

    pub fn get_grid_mut(&mut self) -> NavRes<&mut Grid<GridButton>, String> {
        if self.current_grid.is_none() {
            return NavRes::Error("No grid has been selected!".into());
        }

        let cg: usize = self.current_grid.unwrap();

        return match self.grids.get_mut(&cg) {
            Option::Some(g) => NavRes::OK(g),
            Option::None => NavRes::Error("Grid not found!".into())
        };
    }

    pub fn insert_grid(
        &mut self, 
        index: usize, 
        grid: Grid<GridButton>
    ) -> Option<Grid<GridButton>> {
        return self.grids.insert(index, grid);
    }

    pub fn get_grids(&self) -> &HashMap<usize, Grid<GridButton>> {
        return &self.grids;
    }

    pub fn get_grids_mut(&mut self) -> &mut HashMap<usize, Grid<GridButton>> {
        return &mut self.grids;
    }

    pub fn get_current_grid(&self) -> Option<usize> {
        return self.current_grid;
    }
}