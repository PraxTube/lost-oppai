use bevy::{prelude::*, utils::HashSet};

fn number_of_edges(vertex_index: usize, edges: &HashSet<(usize, usize)>) -> usize {
    edges
        .iter()
        .filter(|&&(a, b)| a == vertex_index || b == vertex_index)
        .count()
}

fn generate_edges(vertices: &[Vec2]) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 0..vertices.len() {
        for j in 0..vertices.len() {
            if i >= j {
                continue;
            }
            edges.push((i, j, vertices[i].distance_squared(vertices[j])));
        }
    }

    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    edges.into_iter().map(|(a, b, _)| (a, b)).collect()
}

pub fn kruskals_edges(vertices: &[Vec2]) -> HashSet<(usize, usize)> {
    let sorted_edges = generate_edges(vertices);

    let mut result = HashSet::new();
    let mut sets = Vec::new();
    for i in 0..vertices.len() {
        let mut s = HashSet::new();
        s.insert(i);
        sets.push(s);
    }

    for (u, v) in sorted_edges {
        let mut u_set_index = None;
        let mut v_set_index = None;

        for (index, set) in sets.iter().enumerate() {
            if set.contains(&u) {
                u_set_index = Some(index);
            }
            if set.contains(&v) {
                v_set_index = Some(index);
            }
            if u_set_index.is_some() && v_set_index.is_some() {
                break;
            }
        }

        if let (Some(u_index), Some(v_index)) = (u_set_index, v_set_index) {
            if u_index != v_index {
                result.insert((u, v));
                let other = sets[v_index].clone();
                sets[u_index].extend(other);
                sets.remove(v_index);
            }
        }
    }
    result
}

pub fn connect_outer_vertices(vertices: &[Vec2], edges: &mut HashSet<(usize, usize)>) {
    if vertices.len() < 2 {
        error!(
            "Should not call function to connect outer vertices on only two vertices, {:?}",
            vertices
        );
        return;
    }

    for i in 0..vertices.len() {
        // If this vertex has more than two edges we don't need
        // to connect it any further because it either is
        // a vertex in the center somewhere or already forms a loop.
        if number_of_edges(i, edges) > 1 {
            continue;
        };

        let mut best_vertex_index = if i == 0 { 1 } else { 0 };
        let mut best_loss = f32::MAX;

        for j in 0..vertices.len() {
            if j == i {
                continue;
            }
            // Don't add the same edge twice
            if edges.contains(&(i, j)) || edges.contains(&(j, i)) {
                continue;
            };

            let loss = vertices[i].distance_squared(vertices[j]) / vertices[j].length_squared();
            if loss < best_loss {
                best_vertex_index = j;
                best_loss = loss;
            }
        }

        // TODO: Currently, we will no matter what add a second edge to all vertices.
        // This might not be too great if the vertex is really alone and has no great connectivity
        // partners. I didn't see any really bad cases yet so it's okay for now, but it could be
        // improved in this regard.
        edges.insert((i, best_vertex_index));
    }
}
