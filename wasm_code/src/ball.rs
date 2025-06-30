use wasm_bindgen::prelude::*;
// Intitilasiert einen Ball
#[wasm_bindgen]
pub struct Ball {
    pub id: u32, // Die ID wird zurück gegeben
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub radius: f32, // Sind alle in f32 da f32 + i32 = AAAAAHHH
    pub mass: f32,
    pub r: f32, //RGB
    pub g: f32,
    pub b: f32,
}