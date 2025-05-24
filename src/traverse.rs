
use raylib::collision;
use raylib::math::{Ray, Vector3};

#[derive(Clone, Copy)]
pub struct Surface {
    normal: Vector3,
    vertices: [Vector3;3],
}

fn sign(x: i64) -> i64 {
    (x > 0) as i64 - (x < 0) as i64
}

fn visit(x: i64, y: i64, z: i64) {
	eprintln!("[ {}, {}, {} ]", x, y, z);
}

const GRID_SIZE: f32 = 10.0;

pub fn find_surface_on_ray (origin: Vector3, delta: Vector3) -> Option<(Vector3, Vector3)>
{
    const U23_MAX: i64 = 2_i64.pow(23);

    // these are i64s so that the math works with
    // delta_xyz, and then we can safely cast to u32s
    let mut x = (origin.x / GRID_SIZE).floor() as i64;
    let mut y = (origin.y / GRID_SIZE).floor() as i64;
    let mut z = (origin.z / GRID_SIZE).floor() as i64;

    // 23 bit maximum for dimensions
    // before floor starts omitting whole numbers
    //
    // could upcast to f64s before the floor,
    // and then check against u32max...
    // 16 million units (* GRID_SIZE (10m)) still huge on a 1m scale
    if x < 0 || x > U23_MAX { panic!("OOB x: {}", x) }
    if y < 0 || y > U23_MAX { panic!("OOB y: {}", y) }
    if z < 0 || z > U23_MAX { panic!("OOB z: {}", z) }

    let dx = delta.x.floor() as i64;
    let dy = delta.y.floor() as i64;
    let dz = delta.z.floor() as i64;

	eprintln!("start [ {}, {}, {} ]", x, y, z);
	eprintln!("delta [ {}, {}, {} ]", dx, dy, dz);

    // step_xyz
    let step_x = sign(dx);
    let step_y = sign(dy);
    let step_z = sign(dz);

    // abs_delta_xyz
    let ax = dx.abs();
    let ay = dy.abs();
    let az = dz.abs();

    // double_delta_xyz
    let bx = 2*ax;
    let by = 2*ay;
    let bz = 2*az;

    // step_pending_xyz
    let mut exy = ay-ax;
    let mut exz = az-ax;
    let mut ezy = ay-az;

    // Set that no surface has been hit
    let mut out_hit = None;
    let mut closest_hit = f32::INFINITY;

    let r = Ray::new(origin, origin - (origin + delta));

    // total_steps
    let n = ax+ay+az;
    for _ in 0..=n {

        visit(x, y, z);
        
        // for all triangles in cell
        for surface in get_surfaces_in_cell(x as u32, y as u32, z as u32) {

            let coll = collision::get_ray_collision_triangle(r, surface.vertices[0], surface.vertices[1], surface.vertices[2]);

            // if collides in current grid cell
            if coll.hit &&
                (coll.point.x / GRID_SIZE).floor() as i64 == x &&
                (coll.point.y / GRID_SIZE).floor() as i64 == y &&
                (coll.point.z / GRID_SIZE).floor() as i64 == z
            {
                if coll.distance < closest_hit {
                    closest_hit = coll.distance;
                    out_hit = Some((surface.normal, coll.point));
                }
            }
        }

        if out_hit.is_some() {
            return out_hit;
        }

        if exy < 0 {
            if exz < 0 {
            x += step_x;
            exy += by; exz += bz;
            }
            else  {
            z += step_z;
            exz -= bx; ezy += by;
            }
        }
        else {
            if ezy < 0 {
            z += step_z;
            exz -= bx; ezy += by;
            }
            else  {
            y += step_y;
            exy -= bx; ezy -= bz;
            }
        }
    }

    None
}


fn get_surfaces_in_cell(x: u32, y: u32, z: u32) -> Vec<Surface> {
    let mut surfs = vec![vec![vec![]]];

    surfs[0][0][0] = Surface{ 
        normal: Vector3::zero(),
        vertices: [Vector3::zero(), Vector3::zero(), Vector3::zero()]
    };

    vec![]
}

fn main(){

    let mut args = std::env::args();
    if args.len() != 7 {
        eprintln!("  USAGE: {} x y z dx dy dz", args.nth(0).unwrap());
        return;
    }

    let nums: Vec<_> = args.skip(1).map(|a| a.parse().unwrap()).collect();
	
	find_surface_on_ray(
        Vector3 { x: nums[0], y: nums[1], z: nums[2] },
        Vector3 { x: nums[3], y: nums[4], z: nums[5] },
        );
}
