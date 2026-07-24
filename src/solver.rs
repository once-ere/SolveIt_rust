use nalgebra::Vector3;

const SOFTENING: f64 = 1e-6;

#[derive(Clone, Debug)]
pub struct PointParticle {
    pub id: usize,
    pub mass: f64,
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
}

impl PointParticle {
    pub fn new(id: usize, mass: f64, x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            id,
            mass,
            position: Vector3::new(x, y, z),
            velocity: Vector3::new(vx, vy, vz),
        }
    }

    /// Linear Momentum: p = m * v
    pub fn momentum(&self) -> Vector3<f64> {
        self.mass * self.velocity
    }

    /// Angular Momentum calculated relative to a specific center of mass coordinate point: L = r_relative x p
    pub fn angular_momentum(&self, center_of_mass: Vector3<f64>) -> Vector3<f64> {
        let r_rel = self.position - center_of_mass;
        r_rel.cross(&self.momentum())
    }

    /// Laplace-Runge-Lenz Vector relative to a moving center of mass point: A = p x L - m * k * r_hat
    /// 'k' is the central force parameter (G * Total_System_Mass)
    pub fn laplace_vector(&self, center_of_mass: Vector3<f64>, k: f64) -> Vector3<f64> {
        let p = self.momentum();
        let l = self.angular_momentum(center_of_mass);

        let r_rel = self.position - center_of_mass;
        let r_norm = r_rel.norm();

        // Prevent division by zero if particle lands perfectly on the center of mass
        let r_hat = if r_norm > 0.0 {
            r_rel / r_norm
        } else {
            Vector3::zeros()
        };

        p.cross(&l) - (self.mass * k * r_hat)
    }
}

pub struct GravitationalSystem {
    pub particles: Vec<PointParticle>,
    pub g_constant: f64,
}

impl GravitationalSystem {
    pub fn new(particles: Vec<PointParticle>, g_constant: f64) -> Self {
        Self { particles, g_constant }
    }

    /// Calculates the shared center of mass position vector for the whole system
    pub fn center_of_mass(&self) -> Vector3<f64> {
        let total_mass: f64 = self.particles.iter().map(|p| p.mass).sum();
        if total_mass == 0.0 { return Vector3::zeros(); }

        let mass_pos_sum: Vector3<f64> = self.particles.iter()
        .map(|p| p.mass * p.position)
        .sum();

        mass_pos_sum / total_mass
    }

    /// Calculates the total combined mass of the system to serve as 'M' in the 'k = G*M' calculation
    pub fn total_mass(&self) -> f64 {
        self.particles.iter().map(|p| p.mass).sum()
    }

    /// Computes the collective gravitational accelerations for all bodies
    pub fn compute_accelerations(&self) -> Vec<Vector3<f64>> {
        let n = self.particles.len();
        let mut accelerations = vec![Vector3::zeros(); n];

        for i in 0..n {
            for j in 0..n {
                if i == j { continue; }
                let r_vec = self.particles[j].position - self.particles[i].position;
                let dist_sq = r_vec.norm_squared() + SOFTENING * SOFTENING;
                let dist = dist_sq.sqrt();
                accelerations[i] += (self.g_constant * self.particles[j].mass / (dist_sq * dist)) * r_vec;
            }
        }
        accelerations
    }

    /// Advances the core physical state vectors by a singular time step using Velocity Verlet
    pub fn step(&mut self, dt: f64) {
        let acc_current = self.compute_accelerations();

        for i in 0..self.particles.len() {
            let vel = self.particles[i].velocity;
            self.particles[i].position += vel * dt + 0.5 * acc_current[i] * dt * dt;
        }

        let acc_next = self.compute_accelerations();

        for i in 0..self.particles.len() {
            self.particles[i].velocity += 0.5 * (acc_current[i] + acc_next[i]) * dt;
        }
    }
}
