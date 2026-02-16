/// Enum for gauging a Grid's state
#[derive(Clone, Copy)]
pub enum GridState {
    /// Highest tier - grid has selection
    Selected,
    /// Middle tier - grid is meant to be "visible"
    Visible,
    /// Lowest tier - grid is meant to "disappear"
    Inactive
}

impl Default for GridState {
    fn default() -> Self {
        return Self::Visible;
    }
}