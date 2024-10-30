pub struct Node {
    pub(crate) id: u32,
    pub(crate) title: String,
    pub(crate) position: egui::Pos2,
    pub(crate) inputs: Vec<Input>,
    pub(crate) outputs: Vec<Output>,
    pub(crate) size: (f32, f32),
}

impl Node {
    pub(crate) fn new(id: u32, title: &str, position: egui::Pos2, size: (f32, f32)) -> Self {
        Self {
            id,
            title: title.to_string(),
            position,
            inputs: vec![
                Input { id: 0, label: "Input1".to_string(), position },
                Input { id: 1, label: "Input2".to_string(), position },
            ],
            outputs: vec![
                Output { id: 0, label: "Output".to_string(), position },
            ],
            size,
        }
    }
}

pub struct Input {
    id: u32,
    pub(crate) label: String,
    position: egui::Pos2,
}

pub struct Output {
    id: u32,
    pub(crate) label: String,
    position: egui::Pos2,
}