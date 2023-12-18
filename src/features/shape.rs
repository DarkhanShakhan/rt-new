use super::{consts::EPSILON, point::Point, ray::Ray, vector::Vector, Object};

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub enum Shape {
    Plane,
    #[default]
    Sphere,
    Cube,
    Cylinder(f64, f64, bool),
    Cone(f64, f64, bool),
    Group(Group),
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        match self {
            Shape::Plane => intersect_plane(ray),
            Shape::Sphere => intersect_sphere(ray),
            Shape::Cube => intersect_cube(ray),
            Shape::Cylinder(minimun, maximum, closed) => {
                intersect_cylinder(*minimun, *maximum, *closed, ray)
            }
            Shape::Cone(minimum, maximum, closed) => {
                intersect_cone(*minimum, *maximum, *closed, ray)
            }
            Shape::Group(_) => todo!(),
        }
    }
    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self {
            Shape::Plane => normal_at_plane(),
            Shape::Sphere => normal_at_sphere(object_point),
            Shape::Cube => normal_at_cube(object_point),
            Shape::Cylinder(minimum, maximum, _) => {
                normal_at_cylinder(*minimum, *maximum, object_point)
            }
            Shape::Cone(minimum, maximum, _) => normal_at_cone(*minimum, *maximum, object_point),
            Shape::Group(_) => todo!(),
        }
    }
}

// PLANE
fn normal_at_plane() -> Vector {
    Vector::new(0.0, 1.0, 0.0)
}

fn intersect_plane(ray: &Ray) -> Option<Vec<f64>> {
    if ray.direction.position.y.abs() < EPSILON {
        return None;
    }
    Some(vec![-ray.origin.position.y / ray.direction.position.y])
}

//SHAPE
fn normal_at_sphere(object_point: &Point) -> Vector {
    *object_point - Point::new(0.0, 0.0, 0.0)
}

fn intersect_sphere(ray: &Ray) -> Option<Vec<f64>> {
    let sphere_to_ray = ray.origin - Point::default();
    let a = ray.direction.dot_product(&ray.direction);
    let b = 2.0 * ray.direction.dot_product(&sphere_to_ray);
    let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
    Some(vec![t1, t2])
}

// CUBE
fn normal_at_cube(object_point: &Point) -> Vector {
    let (x, y, z) = (object_point.x(), object_point.y(), object_point.z());
    let maxc = x.abs().max(y.abs()).max(z.abs());
    if maxc == x.abs() {
        Vector::new(x, 0.0, 0.0)
    } else if maxc == y.abs() {
        Vector::new(0.0, y, 0.0)
    } else {
        Vector::new(0.0, 0.0, z)
    }
}

fn intersect_cube(ray: &Ray) -> Option<Vec<f64>> {
    let (xtmin, xtmax) = check_axis(ray.origin.x(), ray.direction.x());
    let (ytmin, ytmax) = check_axis(ray.origin.y(), ray.direction.y());
    let (ztmin, ztmax) = check_axis(ray.origin.z(), ray.direction.z());
    let tmax = xtmax.min(ytmax.min(ztmax));
    if tmax < 0.0 {
        return None;
    }
    let tmin = xtmin.max(ytmin.max(ztmin));
    if tmin <= tmax {
        return Some(vec![tmin, tmax]);
    }
    None
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_enumerator = -1.0 - origin;
    let tmax_enumerator = 1.0 - origin;
    let tmin = tmin_enumerator / direction;
    let tmax = tmax_enumerator / direction;
    if tmin > tmax {
        return (tmax, tmin);
    }
    (tmin, tmax)
}

#[cfg(test)]
mod cube_tests {
    use super::*;

    #[test]
    fn ray_intersect_cube() {
        let c = Shape::Cube;
        let inputs = [
            (
                Point::new(5.0, 0.5, 0.0),
                Vector::new(-1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(-5.0, 0.5, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 5.0, 0.0),
                Vector::new(0.0, -1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, -5.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, 5.0),
                Vector::new(0.0, 0.0, -1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.0, 0.5, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                -1.0,
                1.0,
            ),
        ];
        for (origin, direction, t1, t2) in inputs {
            let r = Ray::new(origin, direction);
            let xs = c.intersect(&r).unwrap();
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], t1);
            assert_eq!(xs[1], t2);
        }
    }
    #[test]
    fn ray_misses_cube() {
        let c = Shape::Cube;
        let inputs = [
            (
                Point::new(-2.0, 0.0, 0.0),
                Vector::new(0.2673, 0.5345, 0.8018),
            ),
            (
                Point::new(0.0, -2.0, 0.0),
                Vector::new(0.8018, 0.2673, 0.5345),
            ),
            (
                Point::new(0.0, 0.0, -2.0),
                Vector::new(0.5345, 0.8018, 0.2673),
            ),
            (Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (origin, direction) in inputs {
            let r = Ray::new(origin, direction);
            let xs = c.intersect(&r);
            assert!(xs.is_none());
        }
    }
    #[test]
    fn normal_on_surface_of_cube() {
        let c = Shape::Cube;
        let inputs = [
            (Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)),
            (Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (point, normal) in inputs {
            let cube_normal = c.normal_at(&point);
            assert_eq!(normal, cube_normal);
        }
    }
}

// CYLINDER
fn normal_at_cylinder(minimum: f64, maximum: f64, object_point: &Point) -> Vector {
    let (point_x, point_y, point_z) = (object_point.x(), object_point.y(), object_point.z());
    let dist = point_x.powi(2) + point_z.powi(2);
    if dist < 1.0 && point_y >= maximum - EPSILON {
        Vector::new(0.0, 1.0, 0.0)
    } else if dist < 1.0 && point_y <= minimum + EPSILON {
        Vector::new(0.0, -1.0, 0.0)
    } else {
        Vector::new(point_x, 0.0, point_z)
    }
}

fn intersect_cylinder(minimum: f64, maximum: f64, closed: bool, ray: &Ray) -> Option<Vec<f64>> {
    let (direction_x, direction_y, direction_z) =
        (ray.direction.x(), ray.direction.y(), ray.direction.z());
    let a = direction_x.powi(2) + direction_z.powi(2);
    if a.abs() < EPSILON {
        return intersect_cap(minimum, maximum, closed, ray);
    }
    let (origin_x, origin_y, origin_z) = (ray.origin.x(), ray.origin.y(), ray.origin.z());
    let b = 2.0 * origin_x * direction_x + 2.0 * origin_z * direction_z;
    let c = origin_x.powi(2) + origin_z.powi(2) - 1.0;
    let disc = b.powi(2) - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }

    let t0 = (-b - disc.sqrt()) / (2.0 * a);
    let t1 = (-b + disc.sqrt()) / (2.0 * a);
    let mut res = vec![];
    let y_0 = origin_y + t0 * direction_y;
    if minimum < y_0 && y_0 < maximum {
        res.push(t0);
    }
    let y_1 = origin_y + t1 * direction_y;
    if minimum < y_1 && y_1 < maximum {
        res.push(t1);
    }
    if let Some(mut caps) = intersect_cap(minimum, maximum, closed, ray) {
        res.append(&mut caps);
    }
    if !res.is_empty() {
        return Some(res);
    }
    None
}
fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin.x() + t * ray.direction.x();
    let z = ray.origin.z() + t * ray.direction.z();
    x.powi(2) + z.powi(2) <= 1.0
}

fn intersect_cap(minimum: f64, maximum: f64, closed: bool, ray: &Ray) -> Option<Vec<f64>> {
    let direction_y = ray.direction.y();
    if !closed || direction_y.abs() <= EPSILON {
        return None;
    }
    let origin_y = ray.origin.y();
    let t = (minimum - origin_y) / direction_y;
    let mut res = vec![];
    if check_cap(ray, t) {
        res.push(t);
    }
    let t = (maximum - origin_y) / direction_y;
    if check_cap(ray, t) {
        res.push(t);
    }
    if res.is_empty() {
        return None;
    }
    Some(res)
}

#[cfg(test)]
mod cylinder_tests {
    use std::f64::INFINITY;

    use super::*;
    #[test]
    fn ray_misses_cylinder() {
        let cyl = Shape::Cylinder(-INFINITY, INFINITY, false);
        let inputs = [
            (Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)),
        ];
        for (origin, direction) in inputs {
            let r = Ray::new(origin, direction.normalize());
            let xs = cyl.intersect(&r);
            assert!(xs.is_none())
        }
    }
    #[test]
    fn ray_hits_cylinder() {
        let cyl = Shape::Cylinder(-INFINITY, INFINITY, false);
        let inputs = [
            (
                Point::new(1.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0),
                6.80798191702732,
                7.088723439378861,
            ),
        ];
        for (origin, direction, t0, t1) in inputs {
            let r = Ray::new(origin, direction.normalize());
            let xs = cyl.intersect(&r).unwrap();
            assert_eq!(xs[0], t0);
            assert_eq!(xs[1], t1);
        }
    }
    #[test]
    fn normal_vector_on_cylinder() {
        let cyl = Shape::Cylinder(-INFINITY, INFINITY, true);
        let inputs = [
            (Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (point, vector) in inputs {
            let n = cyl.normal_at(&point);
            assert_eq!(n, vector)
        }
    }
    #[test]
    fn normal_vector_on_cylinder_end_caps() {
        let cyl = Shape::Cylinder(1.0, 2.0, true);
        let inputs = [
            (Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.5, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.5), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.5, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        ];
        for (point, normal) in inputs {
            let n = cyl.normal_at(&point);
            assert_eq!(n, normal)
        }
    }
    #[test]
    fn intersect_constrained_cylinder() {
        let cyl = Shape::Cylinder(1.0, 2.0, false);
        let inputs = [
            (Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
            (Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
        ];
        for (origin, direction, len) in inputs {
            let r = Ray::new(origin, direction);
            let xs = cyl.intersect(&r).unwrap_or(vec![]);
            assert_eq!(xs.len(), len)
        }
    }
    #[test]
    fn intersect_caps_of_closed_cylinder() {
        let cyl = Shape::Cylinder(1.0, 2.0, true);
        let inputs = [
            (Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 2),
            (Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2),
            (Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 2),
            (Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2),
        ];
        for (origin, direction, count) in inputs {
            let r = Ray::new(origin, direction.normalize());
            let xs = cyl.intersect(&r).unwrap_or_default();
            assert_eq!(xs.len(), count)
        }
    }
}

// CONE

fn intersect_cone(minimum: f64, maximum: f64, closed: bool, ray: &Ray) -> Option<Vec<f64>> {
    let (direction_x, direction_y, direction_z) =
        (ray.direction.x(), ray.direction.y(), ray.direction.z());
    let (origin_x, origin_y, origin_z) = (ray.origin.x(), ray.origin.y(), ray.origin.z());
    let a = direction_x.powi(2) - direction_y.powi(2) + direction_z.powi(2);
    let b =
        2.0 * origin_x * direction_x - 2.0 * origin_y * direction_y + 2.0 * origin_z * direction_z;
    let c = origin_x.powi(2) - origin_y.powi(2) + origin_z.powi(2);
    if a.abs() <= EPSILON && b.abs() >= EPSILON {
        let mut res = vec![-c / (2.0 * b)];
        if let Some(mut caps) = intersect_cone_cap(minimum, maximum, closed, ray) {
            res.append(&mut caps);
        }
        return Some(res);
    }
    let disc = b.powi(2) - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }

    let t0 = (-b - disc.sqrt()) / (2.0 * a);
    let t1 = (-b + disc.sqrt()) / (2.0 * a);
    let mut res = vec![];
    let y_0 = origin_y + t0 * direction_y;
    if minimum < y_0 && y_0 < maximum {
        res.push(t0);
    }
    let y_1 = origin_y + t1 * direction_y;
    if minimum < y_1 && y_1 < maximum {
        res.push(t1);
    }
    if let Some(mut caps) = intersect_cone_cap(minimum, maximum, closed, ray) {
        res.append(&mut caps);
    }
    if !res.is_empty() {
        return Some(res);
    }
    None
}

fn check_cone_cap(ray: &Ray, t: f64, radius: f64) -> bool {
    let x = ray.origin.x() + t * ray.direction.x();
    let z = ray.origin.z() + t * ray.direction.z();

    (x.powi(2) + z.powi(2)) <= radius.powi(2)
}

fn intersect_cone_cap(minimum: f64, maximum: f64, closed: bool, ray: &Ray) -> Option<Vec<f64>> {
    let direction_y = ray.direction.y();
    if !closed || direction_y.abs() <= EPSILON {
        return None;
    }
    let origin_y = ray.origin.y();
    let t = (minimum - origin_y) / direction_y;
    let mut res = vec![];
    if check_cone_cap(ray, t, minimum) {
        res.push(t);
    }
    let t = (maximum - origin_y) / direction_y;
    if check_cone_cap(ray, t, maximum) {
        res.push(t);
    }
    if res.is_empty() {
        return None;
    }
    Some(res)
}

fn normal_at_cone(minimum: f64, maximum: f64, object_point: &Point) -> Vector {
    let dist = object_point.x().powi(2) + object_point.z().powi(2);

    if dist < 1.0 && object_point.y() >= (maximum - EPSILON) {
        Vector::new(0.0, 1.0, 0.0)
    } else if dist < 1.0 && object_point.y() <= (minimum + EPSILON) {
        Vector::new(0.0, -1.0, 0.0)
    } else {
        Vector::new(
            object_point.x(),
            if object_point.y() > 0.0 {
                -dist.sqrt()
            } else {
                dist.sqrt()
            },
            object_point.z(),
        )
    }
}

#[cfg(test)]
mod cone_tests {
    use std::f64::INFINITY;

    use super::*;
    #[test]
    fn intersect_cone_with_ray() {
        let shape = Shape::Cone(-INFINITY, INFINITY, false);
        let inputs = [
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0),
                8.660254037844386,
                8.660254037844386,
            ),
            (
                Point::new(1.0, 1.0, -5.0),
                Vector::new(-0.5, -1.0, 1.0),
                4.550055679356349,
                49.449944320643645,
            ),
        ];
        for (origin, direction, t0, t1) in inputs {
            let r = Ray::new(origin, direction.normalize());
            let xs = shape.intersect(&r).unwrap_or_default();
            assert_eq!(xs.len(), 2);
            assert_eq!(t0, xs[0]);
            assert_eq!(t1, xs[1]);
        }
    }

    #[test]
    fn intersect_cone_ray_parallel_to_one_half() {
        let shape = Shape::Cone(-INFINITY, INFINITY, false);
        let direction = Vector::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), direction);
        let xs = shape.intersect(&r).unwrap();
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 0.3535533905932738);
    }
    #[test]
    fn intersect_cone_caps() {
        let shape = Shape::Cone(-0.5, 0.5, true);
        let inputs = [
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
        ];
        for (origin, direction, t) in inputs {
            let ray = Ray::new(origin, direction.normalize());
            let xs = shape.intersect(&ray).unwrap_or_default();
            assert_eq!(xs.len(), t);
        }
    }
    #[test]
    fn normal_vector_on_cone() {
        let shape = Shape::Cone(-INFINITY, INFINITY, false);
        let inputs = [
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (
                Point::new(1.0, 1.0, 1.0),
                Vector::new(1.0, -(2.0_f64.sqrt()), 1.0),
            ),
            (Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)),
        ];
        for (point, normal) in inputs {
            let n = shape.normal_at(&point);
            assert_eq!(n, normal)
        }
    }
}

// GROUP
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Group {
    children: Vec<Object>,
}
