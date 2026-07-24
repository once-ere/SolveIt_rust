// Legacy reference app. solver.rs is a read-only donor for the
// physical_object_simulator workspace; dead_code is allowed crate-wide so
// donor fields (e.g. PointParticle::id) stay verbatim yet build warning-free.
#![allow(dead_code)]

mod solver;

use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use solver::{GravitationalSystem, PointParticle};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Holds snapshot state arrays to pass down to our GUI renderer frames
struct TrajectoryFrame {
    p1_pos: [f64; 2],
    p2_pos: [f64; 2],
    p3_pos: [f64; 2],
    // New fields to hold live vector tracking data for the presentation readout panel
    p1_momentum: [f64; 3],
    p1_angular_l: [f64; 3],
    p1_laplace_a: [f64; 3],
}

enum PhysicsCommand {
    Reset,
}

struct OrbitApp {
    receiver: Receiver<TrajectoryFrame>,
    command_sender: Sender<PhysicsCommand>,
    is_running: Arc<AtomicBool>,
    p1_history: Vec<[f64; 2]>,
    p2_history: Vec<[f64; 2]>,
    p3_history: Vec<[f64; 2]>,
    // Store the active vectors of Particle 1 to read them out on the GUI panel
    current_p1_momentum: [f64; 3],
    current_p1_angular_l: [f64; 3],
    current_p1_laplace_a: [f64; 3],
}

impl OrbitApp {
    fn new(
        cc: &eframe::CreationContext<'_>,
        receiver: Receiver<TrajectoryFrame>,
        command_sender: Sender<PhysicsCommand>,
        is_running: Arc<AtomicBool>,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            receiver,
            command_sender,
            is_running,
            p1_history: Vec::new(),
            p2_history: Vec::new(),
            p3_history: Vec::new(),
            current_p1_momentum: [0.0; 3],
            current_p1_angular_l: [0.0; 3],
            current_p1_laplace_a: [0.0; 3],
        }
    }
}

impl eframe::App for OrbitApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Drain incoming cross-thread packets
        while let Ok(frame_data) = self.receiver.try_recv() {
            self.p1_history.push(frame_data.p1_pos);
            self.p2_history.push(frame_data.p2_pos);
            self.p3_history.push(frame_data.p3_pos);

            // Cache the vector values locally for the UI state
            self.current_p1_momentum = frame_data.p1_momentum;
            self.current_p1_angular_l = frame_data.p1_angular_l;
            self.current_p1_laplace_a = frame_data.p1_laplace_a;
        }

        // 2. Render Left Side Panel: Real-Time Vector Readout Dashboard        // 2. Render Left Side Panel: Real-Time Vector Readout Dashboard
        egui::SidePanel::left("vector_dashboard")
        .resizable(false)
        .default_width(320.0)
        .show(ctx, |ui| {
            ui.heading("📊 Live Vector Tracking");
            ui.label("Displaying active physics metrics for Particle 1.");
            ui.separator();

            ui.add_space(10.0);
            ui.colored_label(egui::Color32::from_rgb(100, 149, 237), "Linear Momentum (p = m*v):");
            ui.label(format!("X: {:.4}", self.current_p1_momentum[0]));
            ui.label(format!("Y: {:.4}", self.current_p1_momentum[1]));
            ui.label(format!("Z: {:.4}", self.current_p1_momentum[2]));

            ui.add_space(15.0);
            ui.colored_label(egui::Color32::from_rgb(46, 139, 87), "Angular Momentum (L = r x p):");
            ui.label(format!("X: {:.4}", self.current_p1_angular_l[0]));
            ui.label(format!("Y: {:.4}", self.current_p1_angular_l[1]));
            ui.label(format!("Z: {:.4}", self.current_p1_angular_l[2]));

            ui.add_space(15.0);
            ui.colored_label(egui::Color32::from_rgb(220, 20, 60), "Laplace-Runge-Lenz Vector (A):");
            ui.label(format!("X: {:.4}", self.current_p1_laplace_a[0]));
            ui.label(format!("Y: {:.4}", self.current_p1_laplace_a[1]));
            ui.label(format!("Z: {:.4}", self.current_p1_laplace_a[2]));

            ui.add_space(20.0);
            ui.separator();
            ui.label("💡 Presentation Tip:");
            ui.label("Watch how Lz and Ax/Ay remain highly stable when the simulation runs smoothly!");
        });


        // 3. Render Central Layout Panel: Controls and Plots
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Real-Time 3-Body Desktop Gravity Simulator");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                let currently_running = self.is_running.load(Ordering::Relaxed);
                if currently_running {
                    if ui.button("⏸ Pause").clicked() {
                        self.is_running.store(false, Ordering::Relaxed);
                    }
                } else {
                    if ui.button("▶ Play").clicked() {
                        self.is_running.store(true, Ordering::Relaxed);
                    }
                }

                if ui.button("🔄 Reset").clicked() {
                    self.is_running.store(false, Ordering::Relaxed);
                    self.p1_history.clear();
                    self.p2_history.clear();
                    self.p3_history.clear();
                    self.current_p1_momentum = [0.0; 3];
                    self.current_p1_angular_l = [0.0; 3];
                    self.current_p1_laplace_a = [0.0; 3];
                    let _ = self.command_sender.send(PhysicsCommand::Reset);
                }

                if currently_running {
                    ui.colored_label(egui::Color32::from_rgb(46, 139, 87), "Running...");
                } else {
                    ui.colored_label(egui::Color32::from_rgb(220, 20, 60), "Paused");
                }
            });
            ui.add_space(5.0);

            Plot::new("orbit_plot")
            .view_aspect(1.0)
            .data_aspect(1.0)
            .include_x(-1.5)
            .include_x(1.5)
            .include_y(-1.5)
            .include_y(1.5)
            .legend(egui_plot::Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::new(self.p1_history.clone())).color(egui::Color32::from_rgb(100, 149, 237)).name("Particle 1 (Mass 1.0)"));
                plot_ui.line(Line::new(PlotPoints::new(self.p2_history.clone())).color(egui::Color32::from_rgb(46, 139, 87)).name("Particle 2 (Mass 4.0)"));
                plot_ui.line(Line::new(PlotPoints::new(self.p3_history.clone())).color(egui::Color32::from_rgb(220, 20, 60)).name("Particle 3 (Mass 256.0)"));
            });
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let (data_sender, data_receiver): (Sender<TrajectoryFrame>, Receiver<TrajectoryFrame>) = channel();
    let (cmd_sender, cmd_receiver): (Sender<PhysicsCommand>, Receiver<PhysicsCommand>) = channel();

    let is_running = Arc::new(AtomicBool::new(false));
    let is_running_clone = Arc::clone(&is_running);

    thread::spawn(move || {
        let g_constant = 1.0;
        let dt = 0.0005;

        let init_system = || {
            let p1 = PointParticle::new(1, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            let p2 = PointParticle::new(2, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0, 2.0);
            let p3 = PointParticle::new(3, 256.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            GravitationalSystem::new(vec![p1, p2, p3], g_constant)
        };

        let mut system = init_system();

        loop {
            if let Ok(PhysicsCommand::Reset) = cmd_receiver.try_recv() {
                system = init_system();
                let packet = TrajectoryFrame {
                    p1_pos: [system.particles[0].position.x, system.particles[0].position.y],
                    p2_pos: [system.particles[1].position.x, system.particles[1].position.y],
                    p3_pos: [system.particles[2].position.x, system.particles[2].position.y],
                    p1_momentum: [0.0; 3],
                    p1_angular_l: [0.0; 3],
                    p1_laplace_a: [0.0; 3],
                };
                let _ = data_sender.send(packet);
            }

            if is_running_clone.load(Ordering::Relaxed) {
                system.step(dt);

                let com = system.center_of_mass();
                let k_eff = system.g_constant * system.total_mass();

                // Compute the real-time vector values directly from the refactored struct methods
                let p1_m = system.particles[0].momentum();
                let p1_l = system.particles[0].angular_momentum(com);
                let p1_a = system.particles[0].laplace_vector(com, k_eff);

                let packet = TrajectoryFrame {
                    p1_pos: [system.particles[0].position.x, system.particles[0].position.y],
                    p2_pos: [system.particles[1].position.x, system.particles[1].position.y],
                    p3_pos: [system.particles[2].position.x, system.particles[2].position.y],
                    p1_momentum: [p1_m.x, p1_m.y, p1_m.z],
                    p1_angular_l: [p1_l.x, p1_l.y, p1_l.z],
                    p1_laplace_a: [p1_a.x, p1_a.y, p1_a.z],
                };

                if data_sender.send(packet).is_err() {
                    break;
                }
            }

            thread::sleep(Duration::from_micros(100));
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Orbital Precession Tracker Dashboard",
        options,
        Box::new(|cc| Box::new(OrbitApp::new(cc, data_receiver, cmd_sender, is_running))),
    )
}
