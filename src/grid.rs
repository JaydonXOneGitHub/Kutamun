use vector_x::Vector2;

use crate::{GridState, NavRes};

pub struct Grid<GridButton> {
    pub(crate) buttons: Vec<Vec<GridButton>>,
    pub(crate) position: Vector2<usize>,
    pub(crate) enabled: GridState
}

impl<GridButton> Grid<GridButton> {
    pub fn new() -> Self {
        return Self {
            buttons: Vec::new(),
            position: Vector2::default(),
            enabled: GridState::default()
        };
    }

    pub fn from_callback(callback: impl FnOnce() -> Vec<Vec<GridButton>>) -> Self {
        return Self {
            buttons: callback(),
            position: Vector2::default(),
            enabled: GridState::default()
        };
    }

    pub fn from_vec(buttons: Vec<Vec<GridButton>>) -> Self {
        return Self {
            buttons: buttons,
            position: Vector2::default(),
            enabled: GridState::default()
        };
    }
}

impl<GridButton> Grid<GridButton> {
    pub fn with_enabled(mut self, enabled: GridState) -> Self {
        self.set_enabled(enabled);
        return self;
    }

    pub fn with_position(mut self, position: Vector2<usize>) -> Self {
        self.set_position(position);
        return self;
    }
}

impl<GridButton> Grid<GridButton> {
    pub fn set_enabled(&mut self, enabled: GridState) {
        self.enabled = enabled;
    }

    pub fn set_position(&mut self, position: Vector2<usize>) {
        self.position = position;
    }
}

impl<GridButton> Grid<GridButton> {
    pub fn get_button_mut(&mut self) -> NavRes<&mut GridButton, String> {
        let v1: Option<&mut Vec<GridButton>> = self.buttons.get_mut(
            self.position.two
        );

        if v1.is_none() {
            return NavRes::Error("Row could not be found!".into());
        }

        let row: &mut Vec<GridButton> = v1.unwrap();

        let v2: Option<&mut GridButton> = row.get_mut(self.position.one);

        if v2.is_none() {
            return NavRes::Error("Button could not be found!".into());
        }

        return NavRes::OK(v2.unwrap());
    }
}

impl<GridButton> Grid<GridButton> {
    pub fn get_position(&self) -> Vector2<usize> {
        return self.position;
    }

    pub fn get_button(&self) -> NavRes<&GridButton, String> {
        let v1: Option<&Vec<GridButton>> = self.buttons.get(
            self.position.two
        );

        if v1.is_none() {
            return NavRes::Error("Row could not be found!".into());
        }

        let row: &Vec<GridButton> = v1.unwrap();

        let v2: Option<&GridButton> = row.get(self.position.one);

        if v2.is_none() {
            return NavRes::Error("Button could not be found!".into());
        }

        return NavRes::OK(v2.unwrap());
    }
}