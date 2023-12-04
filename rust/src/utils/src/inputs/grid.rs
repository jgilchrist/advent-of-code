use crate::geometry::d2::grid::Grid;
use itertools::Itertools;

/// Create a `Grid<T>` from a string, where `T` can be created from
/// a single character.
///
/// ```
/// use utils::geometry::d2::vecs::Vec2;
/// use utils::inputs::grid_of;
///
/// struct Digit(pub u32);
///
/// impl From<char> for Digit {
///     fn from(value: char) -> Self {
///         Self(value.to_digit(10).unwrap())
///     }
/// }
///
/// let grid = grid_of::<Digit>("123\n345\n789");
/// assert_eq!(grid.at(Vec2 { x: 1, y: 1 }).unwrap().0, 4);
/// ```
pub fn grid_of<T>(s: &str) -> Grid<T>
where
    T: From<char>,
{
    let lines = s.lines().collect_vec();
    let x_size = lines[0].len();
    let y_size = lines.len();

    Grid::new(
        x_size,
        y_size,
        s.lines().join("").chars().map(|c| c.into()).collect(),
    )
}
