//! Spatial boundary of a physical object.
//!
//! Unifies the two legacy "Boundary" concepts without duplication:
//! - `RigidBody3D`'s `enum Boundary { Sphere, Cuboid }` keeps the name
//!   (extended with `Point` for the point-particle case), and
//! - `RigidBody`'s SDF trait survives as [`Sdf`], including the verbatim
//!   central-difference `surface_normal` default, implemented for the enum.

use crate::linalg::{Mat3, Vec3};

/// Trait defining the spatial boundary of a body via a Signed Distance
/// Field (SDF). Negative values represent the interior; positive values
/// represent the exterior. (Legacy `RigidBody` trait, renamed.)
pub trait Sdf {
    /// Evaluates the signed distance from a local point to the surface.
    fn signed_distance(&self, local_point: &Vec3) -> f64;

    /// Computes the local surface normal at a given local point using
    /// central differences (legacy default implementation, eps = 1e-6).
    fn surface_normal(&self, local_point: &Vec3) -> Vec3 {
        let eps = 1e-6;
        let f_x_p = self.signed_distance(&(*local_point + Vec3::new(eps, 0.0, 0.0)));
        let f_x_n = self.signed_distance(&(*local_point - Vec3::new(eps, 0.0, 0.0)));
        let f_y_p = self.signed_distance(&(*local_point + Vec3::new(0.0, eps, 0.0)));
        let f_y_n = self.signed_distance(&(*local_point - Vec3::new(0.0, eps, 0.0)));
        let f_z_p = self.signed_distance(&(*local_point + Vec3::new(0.0, 0.0, eps)));
        let f_z_n = self.signed_distance(&(*local_point - Vec3::new(0.0, 0.0, eps)));

        Vec3::new(f_x_p - f_x_n, f_y_p - f_y_n, f_z_p - f_z_n).normalize()
    }
}

/// Represents the boundary shape of a body (legacy `RigidBody3D` enum,
/// extended with `Point` so a `PointParticle` is representable).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Boundary {
    /// A dimensionless point (a `PointParticle`).
    Point,
    /// A sphere defined by its radius.
    Sphere { radius: f64 },
    /// A cuboid defined by its half-extents (half-width, half-height,
    /// half-depth).
    Cuboid { half_extents: [f64; 3] },
    /// A solid torus about the local z axis: the centerline circle of
    /// radius `ring_radius` lies in the local xy-plane and is swept by
    /// a tube of radius `tube_radius`. (Inner radius = ring − tube,
    /// outer radius = ring + tube.)
    Torus { ring_radius: f64, tube_radius: f64 },
    /// An ideal solid disk of radius `radius` in the local xy-plane
    /// (zero thickness; its "SDF" is the unsigned distance to the disk,
    /// which is continuous and vanishes exactly on the disk).
    Disk { radius: f64 },
    /// A solid cylinder about the local z axis: `radius` and
    /// `half_height` (the full height is `2 · half_height`).
    Cylinder { radius: f64, half_height: f64 },
}

impl Sdf for Boundary {
    fn signed_distance(&self, local_point: &Vec3) -> f64 {
        match self {
            Boundary::Point => local_point.norm(),
            Boundary::Sphere { radius } => local_point.norm() - radius,
            Boundary::Cuboid { half_extents } => {
                // Exact box SDF.
                let q = Vec3::new(
                    local_point.x.abs() - half_extents[0],
                    local_point.y.abs() - half_extents[1],
                    local_point.z.abs() - half_extents[2],
                );
                let outside = Vec3::new(q.x.max(0.0), q.y.max(0.0), q.z.max(0.0));
                let inside = q.x.max(q.y).max(q.z).min(0.0);
                outside.norm() + inside
            }
            Boundary::Torus { ring_radius, tube_radius } => {
                // Exact torus SDF: distance from the centerline circle,
                // minus the tube radius.
                let rho = (local_point.x * local_point.x + local_point.y * local_point.y).sqrt();
                let dq = rho - ring_radius;
                (dq * dq + local_point.z * local_point.z).sqrt() - tube_radius
            }
            Boundary::Disk { radius } => {
                // Unsigned distance to the closed disk {rho <= radius,
                // z = 0}: zero on the disk, positive elsewhere (a
                // zero-thickness body has no interior).
                let rho = (local_point.x * local_point.x + local_point.y * local_point.y).sqrt();
                let dr = (rho - radius).max(0.0);
                (dr * dr + local_point.z * local_point.z).sqrt()
            }
            Boundary::Cylinder { radius, half_height } => {
                // Exact capped-cylinder SDF (2-D box SDF in (rho, z)).
                let rho = (local_point.x * local_point.x + local_point.y * local_point.y).sqrt();
                let dx = rho - radius;
                let dz = local_point.z.abs() - half_height;
                let ox = dx.max(0.0);
                let oz = dz.max(0.0);
                dx.max(dz).min(0.0) + (ox * ox + oz * oz).sqrt()
            }
        }
    }
}

/// Analytic diagonal inertia tensors from shape (legacy
/// `RigidBody3D::calculate_inertia_tensor`, with `Point` → zero tensor).
pub fn analytic_inertia_tensor(mass: f64, boundary: &Boundary) -> Mat3 {
    match boundary {
        Boundary::Point => Mat3::zeros(),
        Boundary::Sphere { radius } => {
            let i = 0.4 * mass * radius * radius;
            Mat3([[i, 0.0, 0.0], [0.0, i, 0.0], [0.0, 0.0, i]])
        }
        Boundary::Cuboid { half_extents } => {
            let hx = half_extents[0];
            let hy = half_extents[1];
            let hz = half_extents[2];
            let ix = (1.0 / 3.0) * mass * (hy * hy + hz * hz);
            let iy = (1.0 / 3.0) * mass * (hx * hx + hz * hz);
            let iz = (1.0 / 3.0) * mass * (hx * hx + hy * hy);
            Mat3([[ix, 0.0, 0.0], [0.0, iy, 0.0], [0.0, 0.0, iz]])
        }
        Boundary::Torus { ring_radius, tube_radius } => {
            // Solid torus about z (c = ring radius, a = tube radius):
            // I_z = m (c² + ¾ a²), I_x = I_y = m (½ c² + ⅝ a²).
            let c2 = ring_radius * ring_radius;
            let a2 = tube_radius * tube_radius;
            let iz = mass * (c2 + 0.75 * a2);
            let ixy = mass * (0.5 * c2 + 0.625 * a2);
            Mat3([[ixy, 0.0, 0.0], [0.0, ixy, 0.0], [0.0, 0.0, iz]])
        }
        Boundary::Disk { radius } => {
            // Ideal thin solid disk about z: I_x = I_y = ¼ m a²,
            // I_z = ½ m a² (perpendicular-axis theorem).
            let a2 = radius * radius;
            let ixy = 0.25 * mass * a2;
            let iz = 0.5 * mass * a2;
            Mat3([[ixy, 0.0, 0.0], [0.0, ixy, 0.0], [0.0, 0.0, iz]])
        }
        Boundary::Cylinder { radius, half_height } => {
            // Solid cylinder about z, full height H = 2h:
            // I_z = ½ m r², I_x = I_y = m (3 r² + H²) / 12.
            let r2 = radius * radius;
            let h = *half_height;
            let iz = 0.5 * mass * r2;
            let ixy = mass * (3.0 * r2 + 4.0 * h * h) / 12.0;
            Mat3([[ixy, 0.0, 0.0], [0.0, ixy, 0.0], [0.0, 0.0, iz]])
        }
    }
}

/// Half-extent of the boundary along a **unit** direction `u` given in
/// the body frame — the support function `h(u) = max_{x∈B} x·u` of the
/// shape (for the torus this is the support of its convex hull, since a
/// support function cannot see the hole). Exact closed forms for every
/// variant; every shape here is centrally symmetric, so the extent
/// along `−u` equals the extent along `+u`.
pub fn support_extent(boundary: &Boundary, u: Vec3) -> f64 {
    let s_xy = (u.x * u.x + u.y * u.y).sqrt();
    match boundary {
        Boundary::Point => 0.0,
        Boundary::Sphere { radius } => *radius,
        Boundary::Cuboid { half_extents } => {
            half_extents[0] * u.x.abs() + half_extents[1] * u.y.abs() + half_extents[2] * u.z.abs()
        }
        Boundary::Torus { ring_radius, tube_radius } => ring_radius * s_xy + tube_radius,
        Boundary::Disk { radius } => radius * s_xy,
        Boundary::Cylinder { radius, half_height } => radius * s_xy + half_height * u.z.abs(),
    }
}

/// A body-frame point achieving [`support_extent`] along the **unit**
/// body-frame direction `u`. When the supporting set is a face, edge,
/// circle or cap (not a single vertex) the **centroid of that set** is
/// returned — so a flat-on contact places its contact point at the
/// center of the touching face and carries no spurious lever arm. The
/// invariant `p·u = support_extent(u)` holds in every case.
pub fn support_point(boundary: &Boundary, u: Vec3) -> Vec3 {
    let xy = Vec3::new(u.x, u.y, 0.0);
    let s_xy = xy.norm();
    // Radial unit vector; zero when u is axis-parallel (the supporting
    // set is then a full circle/face whose centroid sits on the axis).
    let radial = if s_xy > 1e-12 { xy / s_xy } else { Vec3::zeros() };
    // Signum that maps ~0 components to 0 (edge/face centroids).
    let sg = |x: f64| {
        if x > 1e-12 {
            1.0
        } else if x < -1e-12 {
            -1.0
        } else {
            0.0
        }
    };
    match boundary {
        Boundary::Point => Vec3::zeros(),
        Boundary::Sphere { radius } => u * *radius,
        Boundary::Cuboid { half_extents } => Vec3::new(
            half_extents[0] * sg(u.x),
            half_extents[1] * sg(u.y),
            half_extents[2] * sg(u.z),
        ),
        Boundary::Torus { ring_radius, tube_radius } => radial * *ring_radius + u * *tube_radius,
        Boundary::Disk { radius } => radial * *radius,
        Boundary::Cylinder { radius, half_height } => {
            radial * *radius + Vec3::new(0.0, 0.0, half_height * sg(u.z))
        }
    }
}

/// Dimension of the supporting set along the **unit** body-frame
/// direction `u`: 0 = a single point (vertex / rim point), 1 = an edge,
/// line or circle, 2 = a whole face / disk / cap. Used to pick the
/// contact point of a support-axis contact: the lower-rank body's
/// support point is the true deepest point (its higher-rank partner
/// only contributes a face whose centroid may sit far from the
/// contact).
pub fn support_rank(boundary: &Boundary, u: Vec3) -> u8 {
    let eps = 1e-9;
    let s_xy = (u.x * u.x + u.y * u.y).sqrt();
    match boundary {
        Boundary::Point | Boundary::Sphere { .. } => 0,
        Boundary::Cuboid { .. } => {
            let mut zeros = 0u8;
            if u.x.abs() < eps {
                zeros += 1;
            }
            if u.y.abs() < eps {
                zeros += 1;
            }
            if u.z.abs() < eps {
                zeros += 1;
            }
            zeros
        }
        Boundary::Torus { .. } => {
            if s_xy < eps {
                1 // flat on the plane: the supporting set is a circle
            } else {
                0
            }
        }
        Boundary::Disk { .. } => {
            if s_xy < eps {
                2 // face-on: the whole disk supports
            } else {
                0 // a unique rim point
            }
        }
        Boundary::Cylinder { .. } => {
            if s_xy < eps {
                2 // cap face
            } else if u.z.abs() < eps {
                1 // side line
            } else {
                0 // rim point
            }
        }
    }
}

/// Lateral circumradius of the supporting set along the **unit**
/// body-frame direction `u`, measured about its centroid: 0 for a
/// vertex or single point, the half-length for a supporting edge/line,
/// the circumradius for a supporting face/cap/circle. Lets a contact
/// choose the body whose flat supporting set is SMALLER (its centroid
/// is then inside — or closest to — the true contact patch).
pub fn support_footprint_radius(boundary: &Boundary, u: Vec3) -> f64 {
    let eps = 1e-9;
    let s_xy = (u.x * u.x + u.y * u.y).sqrt();
    match boundary {
        Boundary::Point | Boundary::Sphere { .. } => 0.0,
        Boundary::Cuboid { half_extents } => {
            let mut r2 = 0.0;
            if u.x.abs() < eps {
                r2 += half_extents[0] * half_extents[0];
            }
            if u.y.abs() < eps {
                r2 += half_extents[1] * half_extents[1];
            }
            if u.z.abs() < eps {
                r2 += half_extents[2] * half_extents[2];
            }
            r2.sqrt()
        }
        Boundary::Torus { ring_radius, .. } => {
            if s_xy < eps {
                *ring_radius // flat on the plane: the supporting circle
            } else {
                0.0
            }
        }
        Boundary::Disk { radius } => {
            if s_xy < eps {
                *radius
            } else {
                0.0
            }
        }
        Boundary::Cylinder { radius, half_height } => {
            if s_xy < eps {
                *radius // cap face
            } else if u.z.abs() < eps {
                *half_height // side line
            } else {
                0.0
            }
        }
    }
}

/// Radius of the smallest origin-centered bounding sphere.
pub fn bounding_radius(boundary: &Boundary) -> f64 {
    match boundary {
        Boundary::Point => 0.0,
        Boundary::Sphere { radius } => *radius,
        Boundary::Cuboid { half_extents } => {
            let [hx, hy, hz] = *half_extents;
            (hx * hx + hy * hy + hz * hz).sqrt()
        }
        Boundary::Torus { ring_radius, tube_radius } => ring_radius + tube_radius,
        Boundary::Disk { radius } => *radius,
        Boundary::Cylinder { radius, half_height } => {
            (radius * radius + half_height * half_height).sqrt()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_sdf_and_normal() {
        let s = Boundary::Sphere { radius: 2.0 };
        assert_eq!(s.signed_distance(&Vec3::new(3.0, 0.0, 0.0)), 1.0);
        assert_eq!(s.signed_distance(&Vec3::new(0.0, 1.0, 0.0)), -1.0);
        let n = s.surface_normal(&Vec3::new(2.0, 0.0, 0.0));
        assert!((n.x - 1.0).abs() < 1e-9 && n.y.abs() < 1e-9 && n.z.abs() < 1e-9);
    }

    #[test]
    fn cuboid_sdf() {
        let c = Boundary::Cuboid { half_extents: [1.0, 2.0, 3.0] };
        assert_eq!(c.signed_distance(&Vec3::new(2.0, 0.0, 0.0)), 1.0);
        assert_eq!(c.signed_distance(&Vec3::zeros()), -1.0);
        let n = c.surface_normal(&Vec3::new(1.0, 0.0, 0.0));
        assert!((n.x - 1.0).abs() < 1e-6);
    }

    #[test]
    fn torus_disk_cylinder_sdfs() {
        // Torus: ring 1.5, tube 0.5 (inner 1, outer 2).
        let t = Boundary::Torus { ring_radius: 1.5, tube_radius: 0.5 };
        assert!((t.signed_distance(&Vec3::new(2.5, 0.0, 0.0)) - 0.5).abs() < 1e-15);
        assert!((t.signed_distance(&Vec3::new(1.5, 0.0, 0.0)) - (-0.5)).abs() < 1e-15);
        // Center of the hole: distance to the centerline is 1.5 → sdf = 1.0.
        assert!((t.signed_distance(&Vec3::zeros()) - 1.0).abs() < 1e-15);
        // On the tube surface directly above the centerline.
        assert!(t.signed_distance(&Vec3::new(0.0, 1.5, 0.5)).abs() < 1e-15);

        // Disk r=1: unsigned distance (no interior).
        let d = Boundary::Disk { radius: 1.0 };
        assert!((d.signed_distance(&Vec3::new(0.5, 0.0, 0.7)) - 0.7).abs() < 1e-15);
        assert!((d.signed_distance(&Vec3::new(2.0, 0.0, 0.0)) - 1.0).abs() < 1e-15);
        assert!((d.signed_distance(&Vec3::new(1.0 + 3.0, 0.0, 4.0)) - 5.0).abs() < 1e-15);
        assert_eq!(d.signed_distance(&Vec3::new(0.3, 0.4, 0.0)), 0.0);

        // Cylinder r=1, h=2: side, cap, corner, interior.
        let c = Boundary::Cylinder { radius: 1.0, half_height: 2.0 };
        assert!((c.signed_distance(&Vec3::new(3.0, 0.0, 0.0)) - 2.0).abs() < 1e-15);
        assert!((c.signed_distance(&Vec3::new(0.0, 0.0, 5.0)) - 3.0).abs() < 1e-15);
        assert!((c.signed_distance(&Vec3::new(4.0, 0.0, 6.0)) - 5.0).abs() < 1e-15);
        assert!((c.signed_distance(&Vec3::new(0.0, 0.5, 0.0)) - (-0.5)).abs() < 1e-15);
    }

    #[test]
    fn support_extents_and_points_are_exact() {
        let ez = Vec3::new(0.0, 0.0, 1.0);
        let ex = Vec3::new(1.0, 0.0, 0.0);
        let diag = Vec3::new(1.0, 1.0, 1.0).normalize();

        let t = Boundary::Torus { ring_radius: 1.5, tube_radius: 0.5 };
        assert!((support_extent(&t, ex) - 2.0).abs() < 1e-15, "outer radius");
        assert!((support_extent(&t, ez) - 0.5).abs() < 1e-15, "tube half-thickness");
        // Tilted-axis fit fact used by the bounding-box demo: along a
        // direction with s_xy = sqrt(2/3), the extent is
        // 1.5·sqrt(2/3) + 0.5 ≈ 1.7247 < 2.
        let s = support_extent(&t, diag);
        assert!((s - (1.5 * (2.0f64 / 3.0).sqrt() + 0.5)).abs() < 1e-12);
        assert!(s < 2.0);

        let d = Boundary::Disk { radius: 1.0 };
        assert!((support_extent(&d, ex) - 1.0).abs() < 1e-15);
        assert_eq!(support_extent(&d, ez), 0.0);

        let c = Boundary::Cylinder { radius: 0.25, half_height: 0.75 };
        assert!((support_extent(&c, ez) - 0.75).abs() < 1e-15);
        assert!((support_extent(&c, ex) - 0.25).abs() < 1e-15);

        // Support points achieve their extents: p·u == h(u).
        for b in [t, d, c, Boundary::Cuboid { half_extents: [1.0, 2.0, 0.5] }] {
            for u in [ex, ez, diag, Vec3::new(-0.3, 0.9, -0.6).normalize()] {
                let p = support_point(&b, u);
                assert!(
                    (p.dot(u) - support_extent(&b, u)).abs() < 1e-12,
                    "support point mismatch for {b:?} along {u:?}"
                );
            }
        }

        assert!((bounding_radius(&t) - 2.0).abs() < 1e-15);
        assert!((bounding_radius(&Boundary::Cylinder { radius: 3.0, half_height: 4.0 }) - 5.0).abs() < 1e-15);
    }

    #[test]
    fn inertia_matches_legacy_formulas() {
        let s = analytic_inertia_tensor(2.0, &Boundary::Sphere { radius: 0.5 });
        assert!((s.0[0][0] - 0.4 * 2.0 * 0.25).abs() < 1e-15);
        let c = analytic_inertia_tensor(3.0, &Boundary::Cuboid { half_extents: [1.0, 2.0, 3.0] });
        assert!((c.0[0][0] - (3.0 / 3.0) * (4.0 + 9.0)).abs() < 1e-12);
        assert!((c.0[1][1] - (3.0 / 3.0) * (1.0 + 9.0)).abs() < 1e-12);
        assert!((c.0[2][2] - (3.0 / 3.0) * (1.0 + 4.0)).abs() < 1e-12);
        assert_eq!(analytic_inertia_tensor(1.0, &Boundary::Point), Mat3::zeros());

        // Torus m=1, c=1.5, a=0.5: Iz = c² + ¾a² = 2.4375;
        // Ix = Iy = ½c² + ⅝a² = 1.28125.
        let t = analytic_inertia_tensor(1.0, &Boundary::Torus { ring_radius: 1.5, tube_radius: 0.5 });
        assert!((t.0[2][2] - 2.4375).abs() < 1e-15);
        assert!((t.0[0][0] - 1.28125).abs() < 1e-15);
        assert!((t.0[1][1] - 1.28125).abs() < 1e-15);

        // Disk m=2/3, a=1: Iz = ½·(2/3) = 1/3; Ix = Iy = ¼·(2/3) = 1/6
        // (perpendicular-axis: Iz = Ix + Iy).
        let d = analytic_inertia_tensor(2.0 / 3.0, &Boundary::Disk { radius: 1.0 });
        assert!((d.0[2][2] - 1.0 / 3.0).abs() < 1e-15);
        assert!((d.0[0][0] - 1.0 / 6.0).abs() < 1e-15);
        assert!((d.0[2][2] - (d.0[0][0] + d.0[1][1])).abs() < 1e-15);

        // Cylinder m=2, r=0.25, H=1.5 (h=0.75): Iz = ½·2·0.0625 = 0.0625;
        // Ixy = 2·(3·0.0625 + 2.25)/12 = 0.40625.
        let cy = analytic_inertia_tensor(2.0, &Boundary::Cylinder { radius: 0.25, half_height: 0.75 });
        assert!((cy.0[2][2] - 0.0625).abs() < 1e-15);
        assert!((cy.0[0][0] - 0.40625).abs() < 1e-15);
    }
}
