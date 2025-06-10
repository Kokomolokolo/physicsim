use wasm_bindgen::prelude::*;
use js_sys::Float32Array;

// Allg Aufbau: BallManager hat ein Vector, worin alle Bälle mit id, x, y, dx, dy, radius (obwohl der unnötig ist)
// sind. Js ruft eine rust funktion update_and_get_positions auf. Diese itteriert durch
// den Vector via for ball in &mut self balls. Errechnet für jeden Ball die neuen positionen in den Vector.
// Die Methode gibt bestimme Werte wie id, x, y, radius, zurück und so in js gemalt.
// Es wird ein flaches Array verwendet also wird in js for (let x = 0, x <= arr.len, x = x + 4)

// Methode add_ball schreibt einen neuen Ball type in Balls vie balls.push(Ball)

// Dann wird für dieses gemalt. Wow warum ist das so ein großes Projekt??



// Intitilasiert einen Ball, später vielleicht auch noch rgb werte?
// Kann nicht einfach initialisiert werden,
// da es nicht global bzw. mut geht ohne data races.
// Das ist so hoffentlich mehr oop. yay?
#[wasm_bindgen]
pub struct Ball {
    pub id: u32, // Die ID wird zurück gegeben
    pub dx: f32,
    pub dy: f32,
    pub x: f32,
    pub y: f32,
    pub radius: f32, // Sind alle in f32 da f32 + i32 = AAAAAHHH
} 
#[wasm_bindgen]
pub struct BallManager {
    #[wasm_bindgen(skip)]
    balls: Vec<Ball>, // Vector mit allen Bällen
    next_id: u32, 
    canvas_width: f32, // siehe radius
    canvas_height: f32, // werden später bei kollision verwendet
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
        }
    }

    // Methode fügt Bälle hinzu, wow
    pub fn add_ball(&mut self, x: f32, y: f32, dx: f32, dy: f32, radius: f32) -> u32 {
        let id = self.next_id;
        self.next_id += 1; // Aktualisiert die nächte verfügbare ID in Ballmanager
        self.balls.push( Ball {
            x,
            y,
            dx,
            dy,
            radius,
            id,
        });
        id
    }
    // Berechnet alle bewegungen und schreibt sie in Ballmanager.balls,
    // gibt einen Vector aus. Muss ein Float32Array sein, da wasm_bindgen
    // keine komplexen Datatypen wie ein vec verweden kann. So geht das hoffentlich (foreshadowing?)
    pub fn update_and_get_positions(&mut self) -> Float32Array {
        let mut data = Vec::with_capacity(self.balls.len() * 4);  //Vec mit platz für id, x, y, radius für jeden ball

        for ball in &mut self.balls {
             // Kollisionslogik
             // Kolision mit oben und unten
            if ball.y + ball.radius >= self.canvas_height || ball.y - ball.radius <= 0.0 {
                ball.dy *= -1.0;
            }
            // Kolision mit Wänden
            if ball.x + ball.radius >= self.canvas_width || ball.x -ball.radius <= 0.0 {
                ball.dx *= -1.0;
            }

            // Bewegung
            ball.x = ball.x + ball.dx;
            ball.y = ball.y + ball.dy;
           

            //Füllen des Arrays
            data.push(ball.id as f32); // Umwandlung, da es ein f32 Array ist
            data.push(ball.x);
            data.push(ball.y);
            data.push(ball.radius);
        }
        Float32Array::from(&data[..]) // IDK WTF aber Vec<f32> wird zu Float32Array
    }
}

