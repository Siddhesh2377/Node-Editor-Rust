use eframe::epaint::Color32;
use crate::Node;

pub fn draw_node(ui: &mut egui::Ui, node: &mut Node) {
    let node_rect = egui::Rect::from_min_size(node.position, egui::vec2(node.size.0, node.size.1));
    let node_out_rect = egui::Rect::from_min_size(node.position, egui::vec2(node.size.0, node.size.1 / 3.0));


    // Interact with the node rectangle for dragging
    let mut response_node = ui.interact(node_rect, egui::Id::new(node.id), egui::Sense::click_and_drag());
    let mut  response_node_out = ui.interact(node_out_rect, egui::Id::new(node.id), egui::Sense::click_and_drag());

    // Update node position if it's being dragged
    if response_node.dragged() {
        node.position += response_node.drag_delta();
    }

    if response_node_out.dragged() {
        response_node.dragged = false;
        node.position += response_node.drag_delta();
    }

    // Draw the node background
    let shape = egui::Shape::rect_filled(node_rect, 10.0, Color32::from_rgb(80, 80, 100));
    ui.painter().add(shape);

    // Draw the title of the node
    ui.painter().text(
        node.position + egui::vec2(5.0, 5.0),
        egui::Align2::LEFT_TOP,
        &node.title,
        egui::FontId::default(),
        Color32::WHITE,
    );

    // Draw input ports on the left
    for (i, input) in node.inputs.iter().enumerate() {
        let port_pos = node.position + egui::vec2(5.0, node.size.1 / 3.0 + i as f32 * node.size.1 / 3.0);
        ui.painter().circle_filled(port_pos, 5.0, Color32::GREEN);
        ui.painter().text(port_pos + egui::vec2(10.0, 0.0), egui::Align2::LEFT_CENTER, &input.label, egui::FontId::default(), Color32::WHITE);
    }

    // Draw output ports on the right
    for (i, output) in node.outputs.iter().enumerate() {
        let port_pos = node.position + egui::vec2(node.size.0 - 10.0, node.size.1 / 3.0 + i as f32 * node.size.1 / 3.0);
        ui.painter().circle_filled(port_pos, 5.0, Color32::RED);
        ui.painter().text(port_pos + egui::vec2(-10.0, 0.0), egui::Align2::RIGHT_CENTER, &output.label, egui::FontId::default(), Color32::WHITE);
    }
}


pub fn draw_bezier_connection(ui: &mut egui::Ui, start: egui::Pos2, end: egui::Pos2) {
    // Set up control points for a smooth curve from right to left
    let control_offset = egui::Vec2::new((end.x - start.x) * 0.5, 0.0);
    let control_point_1 = start + control_offset;
    let control_point_2 = end - control_offset;

    let num_segments = 20;
    let mut points = vec![];

    for i in 0..=num_segments {
        let t = i as f32 / num_segments as f32;
        let point = (1.0 - t).powi(3) * start.to_vec2()
            + 3.0 * (1.0 - t).powi(2) * t * control_point_1.to_vec2()
            + 3.0 * (1.0 - t) * t.powi(2) * control_point_2.to_vec2()
            + t.powi(3) * end.to_vec2();
        points.push(egui::Pos2::new(point.x, point.y));
    }

    for i in 0..points.len() - 1 {
        ui.painter().line_segment(
            [points[i], points[i + 1]],
            egui::Stroke::new(2.0, Color32::WHITE),
        );
    }
}