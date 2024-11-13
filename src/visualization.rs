use crate::protein::Protein;
use anyhow::Result;
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::event::{Action, Key};
use kiss3d::nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

#[derive(PartialEq)]
enum ViewMode {
    Backbone,
    Cartoon,
}

pub struct ProteinVisualizer<'a> {
    window: &'a mut Window,
    camera: ArcBall,
    protein: &'a Protein,
    view_mode: ViewMode,
    backbone_group: SceneNode,
    cartoon_group: Option<SceneNode>,
    key_pressed: bool,
    axes: [SceneNode; 3], // X, Y, Z axes
}

impl<'a> ProteinVisualizer<'a> {
    pub fn new(window: &'a mut Window, protein: &'a Protein) -> Result<Self> {
        // Create both visualizations immediately
        let mut backbone_group = window.add_group();
        create_backbone_visualization(protein, &mut backbone_group);
        backbone_group.set_visible(false); // Hide backbone initially

        let mut cartoon_group = window.add_group();
        create_cartoon_visualization(protein, &mut cartoon_group);

        // Setup camera with better initial position
        let camera_distance = protein.maximum_radius * 3.0;
        let eye = Point3::new(camera_distance, camera_distance * 0.5, camera_distance);
        let mut camera = ArcBall::new(eye, Point3::origin());
        camera.set_up_axis(Vector3::new(0.0, 1.0, 0.0));

        // Set minimum distance (maximum zoom)
        camera.set_min_dist(protein.maximum_radius * 1.0);

        // Set maximum distance (minimum zoom)
        camera.set_max_dist(protein.maximum_radius * 5.0);

        // Create coordinate axes
        let axes = create_axes(window, protein.maximum_radius * 0.5);

        Ok(Self {
            window,
            camera,
            protein,
            view_mode: ViewMode::Cartoon,
            backbone_group,
            cartoon_group: Some(cartoon_group),
            key_pressed: false,
            axes,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while self.window.render_with_camera(&mut self.camera) {
            // Handle view mode switching with semicolon key
            if self.window.get_key(Key::Semicolon) == Action::Press {
                if !self.key_pressed {
                    self.key_pressed = true;
                    self.toggle_view_mode();
                }
            } else {
                self.key_pressed = false;
            }

            // Update axes positions to stay in view corner
            self.update_axes_position();
        }
        Ok(())
    }

    fn update_axes_position(&mut self) {
        let view = self.camera.view_transform();
        let scale = self.protein.maximum_radius * 0.15;

        let offset = Vector3::new(
            self.window.size()[0] as f32 * 0.4,
            -(self.window.size()[1] as f32) * 0.4,
            0.0,
        );

        for (_i, axis) in self.axes.iter_mut().enumerate() {
            axis.set_local_scale(scale, scale, scale);
            axis.set_local_rotation(view.rotation);
            axis.set_local_translation(Translation3::from(offset));
        }
    }

    fn toggle_view_mode(&mut self) {
        match self.view_mode {
            ViewMode::Backbone => {
                self.view_mode = ViewMode::Cartoon;
                self.backbone_group.set_visible(false);
                if let Some(ref mut group) = self.cartoon_group {
                    group.set_visible(true);
                }
            }
            ViewMode::Cartoon => {
                self.view_mode = ViewMode::Backbone;
                self.backbone_group.set_visible(true);
                if let Some(ref mut group) = self.cartoon_group {
                    group.set_visible(false);
                }
            }
        }
    }
}

fn create_axes(window: &mut Window, _size: f32) -> [SceneNode; 3] {
    let mut x_axis = window.add_cylinder(0.1, 1.0);
    let mut y_axis = window.add_cylinder(0.1, 1.0);
    let mut z_axis = window.add_cylinder(0.1, 1.0);

    // Set colors for each axis (RGB)
    x_axis.set_color(1.0, 0.0, 0.0); // Red for X
    y_axis.set_color(0.0, 1.0, 0.0); // Green for Y
    z_axis.set_color(0.0, 0.0, 1.0); // Blue for Z

    // Set rotations for Y and Z axes
    y_axis.set_local_rotation(UnitQuaternion::from_axis_angle(
        &Vector3::x_axis(),
        std::f32::consts::FRAC_PI_2,
    ));
    z_axis.set_local_rotation(UnitQuaternion::from_axis_angle(
        &Vector3::y_axis(),
        std::f32::consts::FRAC_PI_2,
    ));

    [x_axis, y_axis, z_axis]
}

fn create_backbone_visualization(protein: &Protein, group: &mut SceneNode) {
    let mut previous_point = None;
    for atom in &protein.atoms {
        if atom.atom_type == "CA" {
            let centered_pos = atom.position - protein.center.coords;
            let mut sphere = group.add_sphere(0.5);

            // Color spheres based on temperature factor (B-factor)
            let normalized_temp = atom.temperature_factor / 100.0; // Typical B-factor range 0-100
            sphere.set_color(
                normalized_temp,               // More red = higher temperature factor
                0.7 * (1.0 - normalized_temp), // Less blue = higher temperature factor
                1.0 - normalized_temp,         // Less green = higher temperature factor
            );

            // Scale sphere based on occupancy
            let scale = atom.occupancy * 0.5;
            sphere.set_local_scale(scale, scale, scale);

            sphere.set_local_translation(Translation3::from(centered_pos));

            if let Some(previous) = previous_point {
                let mut line = group.add_cylinder(0.1, 1.0);
                // Color lines based on chain ID
                match atom.chain_id {
                    'A' => line.set_color(1.0, 0.0, 0.0), // Red for chain A
                    'B' => line.set_color(0.0, 1.0, 0.0), // Green for chain B
                    _ => line.set_color(0.0, 0.0, 1.0),   // Blue for other chains
                }

                let direction = centered_pos - previous;
                let middle_point = previous + direction * 0.5;
                line.set_local_translation(Translation3::from(middle_point));
            }
            previous_point = Some(centered_pos);
        }
    }
}

fn create_cartoon_visualization(protein: &Protein, group: &mut SceneNode) {
    let mut previous_point = None;
    let mut previous_direction = None;

    for atom in &protein.atoms {
        if atom.atom_type == "CA" {
            let centered_pos = atom.position - protein.center.coords;

            if let Some(previous) = previous_point {
                let direction: Vector3<f32> = centered_pos - previous;
                let length = direction.magnitude();

                // Create ribbon segment (thinner and taller)
                let mut ribbon = group.add_cube(0.4, 0.8, length);
                ribbon.set_color(0.0, 0.8, 0.2);

                // Position at midpoint
                let middle_point = previous + direction * 0.5;
                ribbon.set_local_translation(Translation3::from(middle_point));

                // Orient ribbon
                if let Some(_previous_direction) = previous_direction {
                    let rotation =
                        UnitQuaternion::rotation_between(&Vector3::z(), &direction.normalize())
                            .unwrap_or(UnitQuaternion::identity());
                    ribbon.set_local_rotation(rotation);
                }

                previous_direction = Some(direction);
            }
            previous_point = Some(centered_pos);
        }
    }
}
