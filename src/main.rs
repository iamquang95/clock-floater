use eframe::egui;
use std::time::Instant;

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: egui::Color32,
    life: f32,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 180.0])
            .with_always_on_top()
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Clock Floater",
        options,
        Box::new(|_cc| Ok(Box::new(ClockFloaterApp::default()))),
    )
}

struct ClockFloaterApp {
    remaining_seconds: u32,
    input_hours: String,
    input_minutes: String,
    input_seconds: String,
    is_running: bool,
    last_update: Option<Instant>,
    alarm_triggered: bool,
    particles: Vec<Particle>,
    celebration_start: Option<Instant>,
}

impl Default for ClockFloaterApp {
    fn default() -> Self {
        Self {
            remaining_seconds: 0,
            input_hours: String::from("0"),
            input_minutes: String::from("0"),
            input_seconds: String::from("0"),
            is_running: false,
            last_update: None,
            alarm_triggered: false,
            particles: Vec::new(),
            celebration_start: None,
        }
    }
}

impl ClockFloaterApp {
    fn format_time(&self) -> String {
        let hours = self.remaining_seconds / 3600;
        let minutes = (self.remaining_seconds % 3600) / 60;
        let seconds = self.remaining_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    fn trigger_alarm(&mut self) {
        // Play system sound on macOS in a separate thread
        std::thread::spawn(|| {
            std::process::Command::new("afplay")
                .arg("/System/Library/Sounds/Sosumi.aiff")
                .spawn()
                .ok();

            // Play multiple times for 5 seconds of sound
            for _ in 0..100 {
                std::thread::sleep(std::time::Duration::from_millis(50));
                std::process::Command::new("afplay")
                    .arg("/System/Library/Sounds/Sosumi.aiff")
                    .spawn()
                    .ok();
            }
        });

        // Start celebration animation
        self.celebration_start = Some(Instant::now());
        self.spawn_particles();
    }

    fn spawn_particles(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..50 {
            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let speed = rng.gen_range(50.0..150.0);
            let color = match rng.gen_range(0..5) {
                0 => egui::Color32::RED,
                1 => egui::Color32::YELLOW,
                2 => egui::Color32::GREEN,
                3 => egui::Color32::BLUE,
                _ => egui::Color32::from_rgb(255, 0, 255),
            };

            self.particles.push(Particle {
                x: 150.0,
                y: 110.0,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                color,
                life: 1.0,
            });
        }
    }

    fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.x += particle.vx * dt;
            particle.y += particle.vy * dt;
            particle.vy += 200.0 * dt; // gravity
            particle.life -= dt;
        }

        self.particles.retain(|p| p.life > 0.0);
    }
}

impl eframe::App for ClockFloaterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update countdown logic
        if self.is_running {
            if let Some(last_update) = self.last_update {
                let elapsed = last_update.elapsed().as_secs() as u32;
                if elapsed > 0 {
                    if self.remaining_seconds > elapsed {
                        self.remaining_seconds -= elapsed;
                    } else {
                        self.remaining_seconds = 0;
                        self.is_running = false;

                        // Trigger alarm
                        if !self.alarm_triggered {
                            self.alarm_triggered = true;
                            self.trigger_alarm();
                        }
                    }
                    self.last_update = Some(Instant::now());
                }
            }
            ctx.request_repaint();
        }

        // Update celebration particles
        if self.celebration_start.is_some() {
            let dt = 0.016; // ~60fps
            self.update_particles(dt);

            // Keep spawning new particles
            if self.particles.len() < 50 {
                self.spawn_particles();
            }

            ctx.request_repaint();
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(egui::Margin::same(15.0)))
            .show(ctx, |ui| {
            // Make the window draggable by clicking anywhere
            if ui.input(|i| i.pointer.primary_down()) {
                if let Some(_pos) = ui.input(|i| i.pointer.interact_pos()) {
                    if ui.rect_contains_pointer(ui.max_rect()) {
                        // Stop celebration if active
                        if self.celebration_start.is_some() {
                            self.celebration_start = None;
                            self.particles.clear();
                        } else {
                            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }
                    }
                }
            }

            ui.vertical_centered(|ui| {
                // Timer display
                ui.heading(egui::RichText::new(self.format_time()).size(48.0));

                ui.add_space(5.0);

                // Input fields (only show when not running)
                if !self.is_running {
                    ui.horizontal(|ui| {
                        ui.label("H:");
                        let h_edit = egui::TextEdit::singleline(&mut self.input_hours)
                            .desired_width(30.0);
                        if ui.add(h_edit).changed() {
                            self.input_hours.retain(|c| c.is_ascii_digit());
                            if let Ok(val) = self.input_hours.parse::<u32>() {
                                self.input_hours = val.to_string();
                            }
                        }

                        ui.label("M:");
                        let m_edit = egui::TextEdit::singleline(&mut self.input_minutes)
                            .desired_width(30.0);
                        if ui.add(m_edit).changed() {
                            self.input_minutes.retain(|c| c.is_ascii_digit());
                            if let Ok(val) = self.input_minutes.parse::<u32>() {
                                self.input_minutes = val.to_string();
                            }
                        }

                        ui.label("S:");
                        let s_edit = egui::TextEdit::singleline(&mut self.input_seconds)
                            .desired_width(30.0);
                        if ui.add(s_edit).changed() {
                            self.input_seconds.retain(|c| c.is_ascii_digit());
                            if let Ok(val) = self.input_seconds.parse::<u32>() {
                                self.input_seconds = val.to_string();
                            }
                        }
                    });

                    ui.add_space(5.0);

                    // Quick preset buttons
                    ui.horizontal(|ui| {
                        let button_size = egui::vec2(60.0, 25.0);

                        if ui.add_sized(button_size, egui::Button::new("5 min")).clicked() {
                            self.input_hours = "0".to_string();
                            self.input_minutes = "5".to_string();
                            self.input_seconds = "0".to_string();
                            self.remaining_seconds = 5 * 60;
                            self.is_running = true;
                            self.last_update = Some(Instant::now());
                            self.alarm_triggered = false;
                        }
                        if ui.add_sized(button_size, egui::Button::new("20 min")).clicked() {
                            self.input_hours = "0".to_string();
                            self.input_minutes = "20".to_string();
                            self.input_seconds = "0".to_string();
                            self.remaining_seconds = 20 * 60;
                            self.is_running = true;
                            self.last_update = Some(Instant::now());
                            self.alarm_triggered = false;
                        }
                        if ui.add_sized(button_size, egui::Button::new("30 min")).clicked() {
                            self.input_hours = "0".to_string();
                            self.input_minutes = "30".to_string();
                            self.input_seconds = "0".to_string();
                            self.remaining_seconds = 30 * 60;
                            self.is_running = true;
                            self.last_update = Some(Instant::now());
                            self.alarm_triggered = false;
                        }
                    });
                }

                ui.add_space(5.0);

                // Control buttons
                ui.horizontal(|ui| {
                    let button_size = egui::vec2(60.0, 25.0);

                    if !self.is_running {
                        if ui.add_sized(button_size, egui::Button::new("▶ Start")).clicked() {
                            // Parse input and start countdown
                            let hours: u32 = self.input_hours.parse().unwrap_or(0);
                            let minutes: u32 = self.input_minutes.parse().unwrap_or(0);
                            let seconds: u32 = self.input_seconds.parse().unwrap_or(0);
                            self.remaining_seconds = hours * 3600 + minutes * 60 + seconds;
                            self.is_running = true;
                            self.last_update = Some(Instant::now());
                            self.alarm_triggered = false;
                        }
                    } else {
                        if ui.add_sized(button_size, egui::Button::new("⏸ Stop")).clicked() {
                            self.is_running = false;
                            self.last_update = None;
                        }
                    }

                    if ui.add_sized(button_size, egui::Button::new("↻ Reset")).clicked() {
                        self.remaining_seconds = 0;
                        self.is_running = false;
                        self.last_update = None;
                        self.alarm_triggered = false;
                        self.input_hours = String::from("0");
                        self.input_minutes = String::from("0");
                        self.input_seconds = String::from("0");
                    }
                });

                // Draw celebration particles
                for particle in &self.particles {
                    let alpha = (particle.life * 255.0) as u8;
                    let mut color = particle.color;
                    color[3] = alpha;

                    ui.painter().circle_filled(
                        egui::pos2(particle.x, particle.y),
                        4.0,
                        color,
                    );
                }
            });
        });
    }
}
