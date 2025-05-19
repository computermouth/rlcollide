use std::f32;

use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct Surface {
    normal: Vector3,
    vertices: [Vector3;3],
    lower_y: f32,
    upper_y: f32,
}

// should be determined at map compile time by
// farthest distance from 0, 0, 0
const GRID_SIZE: f32 = 10.0;

// should return Option<(hit_surface, hit_pos) instead
// mask is probably unnecessary
pub fn find_surface_on_ray2(origin: Vector3, dir: Vector3, hit_surface: &mut Option<Surface>, hit_pos: &mut Vector3, mask: usize) {

    // Set that no surface has been hit
    *hit_surface = None;
    *hit_pos = origin + dir;

    // Get normalized direction
    let max_len = dir.length();
    if max_len  == 0. {
        return;
    }
    let normalized_dir = dir.normalized();

    // Get the start and end coords converted to cell-space
    let cellf = origin / GRID_SIZE;

    let rdinv = Vector3 {
        x: if dir.x.abs() > f32::EPSILON {
            GRID_SIZE / dir.x
        } else {
            f32::INFINITY
        },
        y: if dir.y.abs() > f32::EPSILON {
            GRID_SIZE / dir.y
        } else {
            f32::INFINITY
        },
        z: if dir.z.abs() > f32::EPSILON {
            GRID_SIZE / dir.z
        } else {
            f32::INFINITY
        }
    };

    // cellu
    let p = [cellf.x as u32, cellf.y as u32, cellf.z as u32];
    // let stp = Vector3 {
    //     x: (!(rdinv.x > 0.) as isize * 2 - 1) as f32,
    //     y: (!(rdinv.y > 0.) as isize * 2 - 1) as f32,
    //     z: (!(rdinv.z > 0.) as isize * 2 - 1) as f32
    // };
    let stp = Vector3 {
        x: match rdinv.x > 0. {
            true => 1.,
            false => -1.,
        },
        y: match rdinv.y > 0. {
            true => 1.,
            false => -1.,
        },
        z: match rdinv.z > 0. {
            true => 1.,
            false => -1.,
        }
    };

    let delta = Vector3 {
        x: rdinv.x.abs().min(1.),
        y: rdinv.y.abs().min(1.),
        z: rdinv.z.abs().min(1.)
    };

    let tmax = Vector3{
        x: ((p[0] as f32 + stp.x.max(0.) - cellf.x) * rdinv.x).abs(),
        y: ((p[1] as f32 + stp.x.max(0.) - cellf.y) * rdinv.y).abs(),
        z: ((p[2] as f32 + stp.x.max(0.) - cellf.z) * rdinv.z).abs()
    };

}



pub fn find_surface_on_ray(origin: Vector3, dir: Vector3) -> Option<(Vector3, Vector3)> {

    // Get normalized direction
    let max_len = dir.length();
    if max_len  == 0. {
        return None;
    }
    let normalized_dir = dir.normalized();

    // Get the start and end coords converted to cell-space
    let cell_f = origin / GRID_SIZE;
    // todo, ensure cells are less than i32::MAX, and also not negative
    let cell = [cell_f.x as u32, cell_f.y as u32, cell_f.z as u32];

    let step: [i32;3] = [
        if normalized_dir.x > 0.0 { 1 } else { -1 },
        if normalized_dir.y > 0.0 { 1 } else { -1 },
        if normalized_dir.z > 0.0 { 1 } else { -1 },
    ];
    
    let mut t_max = [0.0; 3];
    let mut t_delta = [0.0; 3];

    let fa_orig = [origin.x, origin.y, origin.z];
    let fa_norm = [normalized_dir.x, normalized_dir.y, normalized_dir.z];
    
    for i in 0..3 {
        let pos = fa_orig[i];
        let dir = fa_norm[i];
        let grid_cell_pos = (cell[i] as f32) * GRID_SIZE;
    
        if dir != 0.0 {
            let next_boundary = if dir > 0.0 {
                grid_cell_pos + GRID_SIZE - pos % GRID_SIZE
            } else {
                pos % GRID_SIZE
            };
            t_max[i] = next_boundary.abs() / dir.abs();
            t_delta[i] = GRID_SIZE / dir.abs();
        } else {
            t_max[i] = f32::INFINITY;
            t_delta[i] = f32::INFINITY;
        }
    }

    let mut t = 0.0;
    let mut cell = cell;

    // Set that no surface has been hit
    let mut out_hit = None;
    let mut closest_hit = f32::INFINITY;

    while t <= max_len {
        // Query surfaces in the current cell
        if let Some(surfaces) = get_surfaces_in_cell(cell[0], cell[1], cell[2]) {
            for surface in surfaces {
                if let Some(hit) = ray_intersects_surface(origin, normalized_dir, surface) {
                    if hit.distance < closest_hit {
                        closest_hit = hit.distance;
                        out_hit = Some((surface.normal, hit.point));
                    }
                }
            }
        }

        if out_hit.is_some() {
            return out_hit;
        }

        // Move to next cell
        let min_axis = if t_max[0] < t_max[1] {
            if t_max[0] < t_max[2] { 0 } else { 2 }
        } else {
            if t_max[1] < t_max[2] { 1 } else { 2 }
        };

        t = t_max[min_axis];
        t_max[min_axis] += t_delta[min_axis];
        cell[min_axis] = cell[min_axis] as i32 + step[min_axis];

        // Optional: check for out-of-bounds here if needed
    }

    out_hit
}

fn get_surfaces_in_cell(x: u32, y: u32, z: u32) -> Option<Vec<Surface>> {
    None
}

fn ray_intersects_surface(origin: Vector3, dir: Vector3, surface: Surface) -> Option<RayCollision> {
    None
}