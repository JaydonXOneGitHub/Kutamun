use vector_x::Vector3;

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down
}

#[derive(Clone)]
pub enum GridAction {
    Select(Vector3<usize>),
    Move(Direction),
    Back,
}