use crate::data::loader::Map;
use crate::navigation::astar::{astar_pathfinding, PathSegment};

pub fn find_optimal_path(map: &Map, start: (usize, usize), end: (usize, usize)) -> Option<Vec<PathSegment >> {
    astar_pathfinding(map, start, end)
}



// use crate::data::loader::Map;
// use crate::navigation::astar::{astar_pathfinding, PathSegment }; // Correction : PathStep → PathSegment 

// pub fn find_optimal_path(map: &Map, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
//     astar_pathfinding(map, start, end).map(|path| {
//         path.into_iter().map(|step| step.start).collect() // Correction : coordinates → start
//     })
// }
