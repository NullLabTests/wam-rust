use eframe::egui;
use wam_rust::{Wam, Instruction, Register, Term, WamError};

struct WamApp {
    wam: Wam,
    program: Vec<Instruction>,
    current_step: usize,
    execution_log: Vec<String>,
    program_loaded: bool,
}

impl WamApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let program = vec![
            Instruction::PutVariable(Register::Temporary(0), Register::Temporary(1)),
            Instruction::GetValue(Register::Temporary(0), Register::Temporary(2)),
            Instruction::Proceed,
        ];

        Self {
            wam: Wam::new(),
            program,
            current_step: 0,
            execution_log: Vec::new(),
            program_loaded: false,
        }
    }

    fn load_program(&mut self) {
        self.wam = Wam::new();
        self.wam.load_program(self.program.clone());
        self.current_step = 0;
        self.execution_log.clear();
        self.program_loaded = true;
        self.execution_log.push("Program loaded successfully.".to_string());
    }

    fn step_execution(&mut self) {
        if !self.program_loaded {
            return;
        }

        match self.wam.execute_next() {
            Ok(true) => {
                self.current_step += 1;
                self.execution_log.push(format!(
                    "Step {}: Executed {:?}",
                    self.current_step,
                    self.program[self.current_step - 1]
                ));
            }
            Ok(false) => {
                self.execution_log.push("Program completed successfully!".to_string());
                self.program_loaded = false;
            }
            Err(e) => {
                self.execution_log.push(format!("Error: {}", e));
                self.program_loaded = false;
            }
        }
    }
}

impl eframe::App for WamApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Warren Abstract Machine Demonstrator");
            
            ui.horizontal(|ui| {
                if ui.button("Load Program").clicked() {
                    self.load_program();
                }
                
                if ui.button("Step").clicked() {
                    self.step_execution();
                }
            });

            ui.separator();
            
            // Display program
            ui.heading("Program Instructions:");
            egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                for (i, instruction) in self.program.iter().enumerate() {
                    let text = if i == self.current_step && self.program_loaded {
                        format!("> {:?}", instruction)
                    } else {
                        format!("  {:?}", instruction)
                    };
                    ui.label(text);
                }
            });

            ui.separator();

            // Display execution log
            ui.heading("Execution Log:");
            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                for log in self.execution_log.iter().rev() {
                    ui.label(log);
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "WAM Demonstrator",
        options,
        Box::new(|cc| Box::new(WamApp::new(cc))),
    )
}
