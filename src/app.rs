

use egui::{Shape, pos2, Vec2, emath, Pos2, Sense, Rect, epaint::PathShape, Stroke, Color32};
#[path = "delaunay.rs"] mod delaunay;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] 
pub struct TemplateApp {
    label: String,
    value: f32,
    control_points: Vec<Pos2>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            control_points: vec![pos2(2., 3.), pos2(100., 100.)]
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn clear(&mut self) {
        let new_pts: Vec<Pos2> = Vec::new();
        self.control_points = new_pts;
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Clear").clicked() {
                        self.clear();
                        // _frame.close();
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            
            let (response, painter) =
            ui.allocate_painter(Vec2::new(ui.available_width(), 300.0), Sense::click_and_drag());

            let to_screen = emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );
            let inverse_to_screen = to_screen.inverse();

            let control_point_radius = 5.;
            let control_point_shapes: Vec<Shape> = self
            .control_points
            .iter_mut()
            .enumerate()
            .map(|(i, point)| {
                let size = Vec2::splat(2.0 * control_point_radius);

                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
                let point_response = ui.interact(point_rect, point_id, Sense::drag());

                *point += point_response.drag_delta();
                *point = to_screen.from().clamp(*point);

                let point_in_screen = to_screen.transform_pos(*point);
                let stroke = ui.style().interact(&point_response).fg_stroke;

                Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
            })
            .collect();
            let mut points: Vec<Pos2> = self.control_points.iter().map(|p| to_screen.transform_pos(*p)).collect();
            let mut control_point_mouse: Vec<Pos2> = Vec::new();

            if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                // TODO: if circle with mouse overlaps with existing circles, nuke below
                if points.iter().all(|&pt| pt.distance(pointer_pos) >  control_point_radius *  f32::sqrt(3.)) {
                    points.push(pointer_pos);
                    control_point_mouse.push(to_screen.transform_pos(pointer_pos));
                }
                if response.clicked() {
                    self.control_points.push(inverse_to_screen.transform_pos(pointer_pos));
                }
            }

            let mouse_shape: Vec<Shape> = control_point_mouse.iter_mut()
            .enumerate()
            .map(|(_i, point)| {
                let size = Vec2::splat(2.0 * control_point_radius);

                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with("pointer");
                let point_response = ui.interact(point_rect, point_id, Sense::drag());

                *point += point_response.drag_delta();
                *point = to_screen.from().clamp(*point);

                let point_in_screen = inverse_to_screen.transform_pos(*point);
                let stroke = ui.style().interact(&point_response).fg_stroke;

                Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
            })
            .collect();

           let stroke = Stroke::new(3.0, Color32::RED.linear_multiply(0.25));
            for edge in delaunay::export_boyer_watson(points) {
                painter.add(PathShape::line(edge, stroke));
            }
            painter.extend(control_point_shapes);
            painter.extend(mouse_shape);

            egui::warn_if_debug_build(ui);
            
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}

