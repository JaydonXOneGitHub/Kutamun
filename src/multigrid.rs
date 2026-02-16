use std::collections::HashMap;

use vector_x::{Vector2, Vector3};

use crate::{Direction, Grid, NavRes};

pub type NavigationCallback = Box<dyn Fn(
    Direction,
    Vector3<usize>
)-> Vector3<usize>>;

pub struct MultiGrid<GridButton> {
    grids: HashMap<usize, Grid<GridButton>>,
    navigation_callback: Option<NavigationCallback>,
    pub(crate) current_grid: Option<usize>
}

impl<GridButton> MultiGrid<GridButton> {
    pub fn new() -> Self {
        return Self {
            grids: HashMap::new(),
            navigation_callback: Option::None,
            current_grid: Option::None
        };
    }
}

impl<GridButton> MultiGrid<GridButton> {
    pub fn with_grid(mut self, grid: (usize, Grid<GridButton>)) -> Self {
        if self.grids.contains_key(&grid.0) {
            return self;
        }

        self.grids.insert(grid.0, grid.1);

        return self;
    }

    pub fn with_navigation_callback(
        mut self, 
        callback: Option<NavigationCallback>
    ) -> Self {
        self.navigation_callback = callback.into();
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

impl<GridButton> MultiGrid<GridButton> {
    pub fn get_or_create_grid(
        &mut self, 
        index: usize
    ) -> NavRes<&mut Grid<GridButton>, String> {
        return if self.grids.contains_key(&index) {
            NavRes::OK(self.grids.get_mut(&index).unwrap())
        } else {
            self.create_grid(index)
        };
    }

    pub fn create_grid(
        &mut self,
        index: usize
    ) -> NavRes<&mut Grid<GridButton>, String> {
        self.grids.insert(index, Grid::new());

        return NavRes::OK(self.grids.get_mut(&index).unwrap());
    }

    pub fn move_focus(&mut self, direction: Direction) {
        if self.current_grid.is_none() {
            return;
        }

        let cg: usize = self.current_grid.clone().unwrap();
        let o = self.grids.get_mut(&cg);

        let grid: &mut Grid<GridButton> = o.unwrap();

        if let Option::Some(callback) = &self.navigation_callback {
            let pos = callback(direction, Vector3::new(
                cg, 
                grid.get_position().one, 
                grid.get_position().two
            ));

            self.current_grid = Option::Some(pos.one);

            grid.set_position(Vector2::new(pos.two, pos.three));
        }
    }
}

impl<GridButton> MultiGrid<GridButton> {
    pub fn insert_grid(
        &mut self, 
        index: usize, 
        grid: Grid<GridButton>
    ) -> Option<Grid<GridButton>> {
        return self.grids.insert(index, grid);
    }
    
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
}