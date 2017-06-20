use std::collections::HashSet;
use geometry::*;
use dungeon::Level;
use grid;

const FOV_RADIUS: i32 = 4;

pub fn calculate(level: &Level, start: Point) -> HashSet<Point> {
    let mut fov: HashSet<Point> = HashSet::new();

    fov.insert(start);

    for r in 1 .. FOV_RADIUS + 1 {
        for point in Rectangle::point(start).grow(r).edges() {
            if grid::RECTANGLE.contains(point) {
                // check if `point` is visible by testing each possible preceeding
                // point that line of sight could come from. these predecessors are
                // determined by `point`'s position relative to the starting point
                // in such a way to prevent line of sight from bending around
                // obstacles; if we're above and to the right of the center, we
                // should only accept line of sight from below and to the left, etc.
                let Point(relative_x, relative_y) = start - point;
                let predecessor = point.step_towards(start);
                if los_comes_from(&level, &fov, predecessor)
                    || (relative_x.abs() < relative_y.abs()
                        && los_comes_from(&level, &fov, Point(point.0, predecessor.1)))
                    || (relative_y.abs() < relative_x.abs()
                        && los_comes_from(&level, &fov, Point(predecessor.0, point.1)))
                {
                    fov.insert(point);
                }
            }
        }
    }

    fov
}

// test to see if line of sight can pass on from a given point
fn los_comes_from(level: &Level, fov: &HashSet<Point>, point: Point) -> bool {
    fov.contains(&point) && level.tiles[point].permits_sight()
}
