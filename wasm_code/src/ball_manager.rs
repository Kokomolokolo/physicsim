use wasm_bindgen::prelude::*;
use js_sys::Float32Array;

use crate::ball::Ball;
use crate::physics::collisions::{ handle_collisons_between_balls_v1, handle_collisons_between_balls_v2 };
// use crate::physics::sph::compute_sph_forces;

#[wasm_bindgen]
pub struct BallManager {
    #[wasm_bindgen(skip)]
    balls: Vec<Ball>, // Vector mit allen Bällen
    next_id: u32, 
    canvas_width: f32, // siehe radius
    canvas_height: f32, // werden später bei kollision verwendet
    elasticity: f32, // 0 = unelastisch, 1 = perfekt elastisch
    gravity: f32, 
}

// Implementierung des BallManagers
#[wasm_bindgen]
impl BallManager {
    // Erstellt neuen Ballmanager, wird anfangs einmalig im js gecalled
    pub fn new(canvas_width: f32, canvas_height: f32) -> Self { // Nimmt width und height, gibt Ballmanager zurück
        BallManager {
            balls: Vec::new(), // Leeren Vector Balls
            next_id: 0,
            canvas_width,
            canvas_height,
            elasticity: 0.9, // 0 = unelastisch, 1 = perfekt elastisch
            gravity: 0.5,
        }
    }

    // Methode fügt Bälle hinzu, wow
    pub fn add_ball(&mut self, x: f32, y: f32, dx: f32, dy: f32, radius: f32, mass: f32, r: f32, g: f32, b: f32) -> u32 {
        let id = self.next_id;
        self.next_id += 1; // Aktualisiert die nächte verfügbare ID in Ballmanager
        self.balls.push( Ball {
            id,
            x,
            y,
            dx,
            dy,
            radius,
            mass,
            r,
            g,
            b,
        });
        if self.balls.len() == 0 {
            panic!("Bälle konnten nicht hinzu gefügt werden")
        } 
        id
    }

    pub fn create_formation(&mut self, num_x: i32, num_y: i32, spacing: f32) {
        let start_x = 50.0;
        let start_y = 50.0;

        for i in 0..num_x {
            for j in 0..num_y {
                self.add_ball(
                    start_x + i as f32 * spacing,
                    start_y + j as f32 * spacing,
                    0.0, 0.0,  // Startgeschwindigkeit
                    5.0,       // Radius
                    1.0,       // Masse
                    255.0, 0.0, 0.0 // Farbe (rot)
                );
            }
        }
    }
    // Berechnet alle bewegungen und schreibt sie in Ballmanager.balls,
    // gibt einen Vector aus. Muss ein Float32Array sein, da wasm_bindgen
    // keine komplexen Datatypen wie ein vec verweden kann. So geht das hoffentlich (foreshadowing?)
    pub fn update_and_get_positions(&mut self) -> Float32Array {
        let mut data = Vec::with_capacity(self.balls.len() * 8);  // Vec mit platz für id, x, y, radius, mass, r, g, b
                                                                  // für jeden ball
        for ball in &mut self.balls {
            // Bewegung
            ball.dy += self.gravity;
            ball.x = ball.x + ball.dx;
            ball.y = ball.y + ball.dy;
           
            // *** Kollisionslogik ***
            // Kolision mit oben und unten
            if ball.y + ball.radius >= self.canvas_height || ball.y - ball.radius <= 0.0 {
                ball.dy *= -self.elasticity;
            }
            // Kolision mit Wänden
            if ball.x + ball.radius >= self.canvas_width || ball.x -ball.radius <= 0.0 {
                ball.dx *= -self.elasticity;
            }
            // Korigierte Positonen bei Feststecken oder Rausfallen
            if ball.x + ball.radius >= self.canvas_width {
                ball.x = self.canvas_width - ball.radius; // Position korrigieren
                // ball.dx *= -1.0;
            } else if ball.x - ball.radius <= 0.0 {
                ball.x = ball.radius; // Position korrigieren
                // ball.dx *= -1.0;
            }
            if ball.y + ball.radius >= self.canvas_height {
                ball.y = self.canvas_height - ball.radius;
            }
            else if ball.y - ball.radius <= 0.0 {
                ball.y = ball.radius;
            }
        }

        // Kolison der Bälle untereinader
        // Nested For loops, keine dopelten checks
        // i ist immer kleiner als j, j iterriert über alle größeren als i, da die kleineren bereits
        // mit allem verglichen wurden
        for i in 0..self.balls.len(){ // range von null bis ball.len()
            for j in (i + 1)..self.balls.len() {
                let (ball1, ball2) = {
                    let (left_part, right_part) = self.balls.split_at_mut(j);
                    // Das teilt das Array an einem index in 2 Hälften. Das ist wohl wegen des Borror checkers wichtig.
                    (&mut left_part[i], &mut right_part[0])
                    // Gibt die mut Referencen zu einem Ball im linken Part und dem ersten im Rechten zurück
                };
                handle_collisons_between_balls_v2(ball1, ball2, self.elasticity); // Schlecht und irritiernend , bei sph nicht benötigt
            }
        }
        for ball in &self.balls {
            //Füllen des Arrays
            data.push(ball.id as f32); // Umwandlung, da es ein f32 Array ist
            data.push(ball.x);
            data.push(ball.y);
            data.push(ball.radius);
            data.push(ball.r);
            data.push(ball.g);
            data.push(ball.b);
        }
        Float32Array::from(&data[..]) // IDK WTF aber Vec<f32> wird zu Float32Array
    }

    // Getters und Setters für Elastizität und Gravitation

    pub fn set_gravity(&mut self, g: f32) {
        self.gravity = g;
    }
    pub fn get_gravity(&self) -> f32 {
        self.gravity
    }

    pub fn set_elasticity(&mut self, e: f32) {
        self.elasticity = e;
    }
    pub fn get_elasticity(&self) -> f32 {
        self.elasticity
    }
}