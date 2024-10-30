mod node_draw;
mod node_struct;

use eframe::egui;
use std::vec::Vec;
use node_draw::*;
use node_struct::*;

fn main() -> Result<(), eframe::Error> {
    let app: Box<dyn eframe::App> = Box::new(NodeEditorApp::default());
    let options = eframe::NativeOptions::default();
    eframe::run_native("Node Editor", options, Box::new(|_cc| Ok(app)))
}

struct NodeEditorApp {
    nodes: Vec<Node>,
    next_node_id: u32, // Counter for unique node IDs
}

impl Default for NodeEditorApp {
    fn default() -> Self {
        Self {
            nodes: vec![
                Node::new(1, "Node A", egui::Pos2::new(100.0, 100.0), (200.0, 100.0)),
                Node::new(2, "Node B", egui::Pos2::new(400.0, 200.0), (200.0, 100.0)),
            ],
            next_node_id: 3, // Start IDs from 3 since 1 and 2 are already used
        }
    }
}


impl eframe::App for NodeEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Hello").clicked() {
                    if ui.button("Add Node").clicked() {
                        // Add a new node at a default position
                        let new_node = Node::new(
                            self.next_node_id,
                            &format!("Node {}", self.next_node_id),
                            egui::Pos2::new(50.0 * self.next_node_id as f32, 50.0), // Offset each new node
                            (200.0, 100.0),
                        );
                        self.nodes.push(new_node);
                        self.next_node_id += 1; // Increment ID counter for the next node

                    }
                }
            });

            for node in &mut self.nodes {
                draw_node(ui, node);
            }

            // Draw a connection from the right side of Node A to the left side of Node B
            if let (Some(start), Some(end)) = (self.nodes.get(0), self.nodes.get(1)) {
                let start_pos = start.position + egui::vec2(start.size.0, start.size.1 / 3.0); // Right-center of Node A's output
                let end_pos = end.position + egui::vec2(0.0, end.size.1 / 3.0); // Left-center of Node B's input
                draw_bezier_connection(ui, start_pos, end_pos);
            }
        });
    }
}




