use std::{sync::Arc, time::Instant};

use cgmath::{InnerSpace, Zero};
use winit::{keyboard::KeyCode, window::Window};

use crate::{camera::Camera, input_context::InputContext};

pub struct State {
    // camera stuff
    pub camera: Camera,
    // accumulated time
    pub timer: Instant,
    pub prev_time: Option<f32>,
    pub focus: bool,
    pub prev_device_mouse_delta: Option<(f64, f64)>,
}
impl State {
    pub fn update(&mut self, input_context: &mut InputContext, window: Arc<Window>) {
        let current_time = self.timer.elapsed().as_secs_f32();
        let delta_time = current_time - self.prev_time.unwrap_or(current_time);
        assert!(delta_time >= 0.0);
        self.prev_time = Some(current_time);
        let delta_speed = self.camera.acceleration * delta_time;
        assert!(delta_speed >= 0.0);
        let damp_factor = self.camera.damp_factor * delta_time;
        assert!(damp_factor >= 0.0);
        let max_speed = self.camera.max_speed;
        fn update_camera_speed(
            curr_speed: &mut f32,
            delta_speed: f32,
            max_speed: f32,
            negative: bool,
        ) {
            assert!(delta_speed >= 0.0);
            let unit = if negative { -1.0 } else { 1.0 };
            let delta_speed = if *curr_speed * unit < 0.0 {
                3.0 * delta_speed
            } else {
                delta_speed
            };
            *curr_speed = if negative {
                *curr_speed - delta_speed
            } else {
                *curr_speed + delta_speed
            };
            *curr_speed = curr_speed.clamp(-max_speed, max_speed);
        }
        let mut w_s_pressed = false;
        let mut a_d_pressed = false;
        let mut space_shift_pressed = false;

        if input_context.get_key(KeyCode::KeyW) {
            println!("W pressed");
            update_camera_speed(
                &mut self.camera.curr_local_speed.z,
                delta_speed,
                max_speed,
                true,
            );
            println!("Camera pos: {:?}", self.camera.pos);
            w_s_pressed = true;
        }
        if input_context.get_key(KeyCode::KeyS) {
            println!("S pressed");
            update_camera_speed(
                &mut self.camera.curr_local_speed.z,
                delta_speed,
                max_speed,
                false,
            );
            println!("Camera pos: {:?}", self.camera.pos);
            w_s_pressed = true;
        }
        if input_context.get_key(KeyCode::KeyA) {
            println!("A pressed");
            update_camera_speed(
                &mut self.camera.curr_local_speed.x,
                delta_speed,
                max_speed,
                true,
            );
            println!("Camera pos: {:?}", self.camera.pos);
            a_d_pressed = true;
        }
        if input_context.get_key(KeyCode::KeyD) {
            println!("D pressed");
            update_camera_speed(
                &mut self.camera.curr_local_speed.x,
                delta_speed,
                max_speed,
                false,
            );
            println!("Camera pos: {:?}", self.camera.pos);
            a_d_pressed = true;
        }
        if input_context.get_key(KeyCode::Space) {
            println!("Space pressed");
            update_camera_speed(
                &mut self.camera.curr_local_speed.y,
                delta_speed,
                max_speed,
                false,
            );
            println!("Camera pos: {:?}", self.camera.pos);
            space_shift_pressed = true;
        }
        if input_context.get_key(KeyCode::ShiftLeft) {
            println!("Shift pressed");
            update_camera_speed(
                &mut self.camera.curr_local_speed.y,
                delta_speed,
                max_speed,
                true,
            );
            println!("Camera pos: {:?}", self.camera.pos);
            space_shift_pressed = true;
        }
        fn damp_camera(curr_speed: &mut f32, damp_factor: f32) {
            assert!(damp_factor >= 0.0);
            let old_speed = *curr_speed;
            let delta_speed = curr_speed.abs() * damp_factor;
            assert!(delta_speed >= 0.0);
            let new_speed = if *curr_speed < 0.0 {
                *curr_speed + delta_speed
            } else {
                *curr_speed - delta_speed
            };
            let new_speed = if old_speed * new_speed <= 0.0 || new_speed.abs() < 0.001 {
                0.0
            } else {
                new_speed
            };
            *curr_speed = new_speed;
        }
        if !w_s_pressed {
            damp_camera(&mut self.camera.curr_local_speed.z, damp_factor);
        }
        if !a_d_pressed {
            damp_camera(&mut self.camera.curr_local_speed.x, damp_factor);
        }
        if !space_shift_pressed {
            damp_camera(&mut self.camera.curr_local_speed.y, damp_factor);
        }

        // convert local speed to global speed
        fn local_to_global(curr_speed: cgmath::Vector3<f32>, yaw: f32) -> cgmath::Vector3<f32> {
            let forward = cgmath::Vector3::new(yaw.to_radians().cos(), 0.0, yaw.to_radians().sin());
            let up = cgmath::Vector3::unit_y();
            let right = forward.cross(up).normalize();
            let global_speed = forward * -curr_speed.z + right * curr_speed.x + up * curr_speed.y;
            global_speed
        }

        let global_speed = local_to_global(self.camera.curr_local_speed, self.camera.yaw);
        self.camera.pos += global_speed * delta_time;
        println!("Camera pos: {:?}", self.camera.pos);

        if input_context.mouse_left_down() {
            println!("Mouse left down");
            self.focus = !self.focus;
            if self.focus {
                window.set_cursor_visible(false);
                window
                    .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                    .or_else(|_| window.set_cursor_grab(winit::window::CursorGrabMode::Confined))
                    .ok();
            } else {
                window.set_cursor_visible(true);
                window
                    .set_cursor_grab(winit::window::CursorGrabMode::None)
                    .ok();
            }
        }
        if input_context.mouse_left_up() {
            println!("Mouse left up");
        }
        if self.focus {
            let new_accumulated = input_context.device_mouse_delta_accumulated();
            let delta = match self.prev_device_mouse_delta {
                Some(old_accumulated) => (
                    new_accumulated.0 - old_accumulated.0,
                    new_accumulated.1 - old_accumulated.1,
                ),
                None => (0.0, 0.0),
            };
            self.prev_device_mouse_delta = Some(new_accumulated);
            println!("Mouse delta: {:?}", delta);
            self.camera.yaw += delta.0 as f32 * self.camera.sensitivity;
            println!("Camera yaw: {:?}", self.camera.yaw);
            self.camera.pitch -= delta.1 as f32 * self.camera.sensitivity;
            self.camera.pitch = self.camera.pitch.clamp(-89.0, 89.0);
        } else {
            self.prev_device_mouse_delta = None;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            camera: Camera::default(),
            timer: Instant::now(),
            prev_time: None,
            focus: false,
            prev_device_mouse_delta: None,
        }
    }
}
