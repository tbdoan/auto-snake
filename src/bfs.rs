use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::arena::Coord;
use crate::arena::Dimensions;
use crate::arena::Direction;

/// Return path, inclusive start and end. If no path found, return empty
pub fn bfs(start: Coord, end: Coord, dim: Dimensions, obstacles: &[Coord]) -> Vec<Coord> {
    assert!(start != end, "start must not equal end");
    assert!(!obstacles.contains(&end), "end is guaranteed open");

    // start at the snake's head
    let mut q = VecDeque::new();
    let mut v = HashSet::new();
    q.push_back(start);
    v.insert(start);

    // add "obstacles" to the visited set, so we dont traverse
    v.extend(obstacles);

    // for path reconstruction
    let mut child_parent = HashMap::new();

    'bfs: while let Some(cur) = q.pop_front() {
        for nbor in cur.neighbors() {
            if dim.check_oob(&nbor) {
                continue;
            }
            if v.contains(&nbor) {
                continue;
            }
            child_parent.insert(nbor, cur);
            v.insert(nbor);
            q.push_back(nbor);

            // we want to make sure the target ends up in the mapping
            if nbor == end {
                break 'bfs;
            }
        }
    }

    if !child_parent.contains_key(&end) {
        return Vec::new();
    }

    // reconstruct the path
    let mut path = Vec::new();
    let mut cur = &end;
    while let Some(parent) = child_parent.get(cur) {
        path.push(*cur);
        cur = parent;
    }
    path.push(*cur);
    path.reverse();
    return path;
}
