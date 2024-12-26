use crate::geometry::d2::grid::Grid;

pub fn grid_of<T>(s: &str) -> Grid<T>
where
    T: From<char>,
{
    let lines = s.lines().collect::<Vec<_>>();
    let x_size = lines[0].len();
    let y_size = lines.len();

    Grid::new(
        x_size,
        y_size,
        s.lines()
            .collect::<String>()
            .chars()
            .map(|c| c.into())
            .collect(),
    )
}
