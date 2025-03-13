



pub struct State{
    pub camera_pos: cgmath::Vector3<f32>,
    pub camera_yaw: f32,
    pub camera_pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for State{
    fn default() -> Self{
        State{
            camera_pos: cgmath::vec3(0.0, 0.0, 3.0),
            camera_yaw: -90.0,
            camera_pitch: 0.0,
            speed: 2.5,
            sensitivity: 0.1,
        }
    }
}