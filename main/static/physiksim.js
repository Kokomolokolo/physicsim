// Ein einfacher Physik Simulator mit Rust im wasm und auch mit Rocket Backend
// TODO: Bessere Kolisonslogik
//          -> Gravitation via einem Toggle
//          -> Masse und Velocity im Ball integrieren
//          -> Bessere Kolisonslogik: https://www.youtube.com/watch?v=OeCgAj35n7o als vorbild
//       RGB Farben auf den Bällen

import init, { BallManager } from "./wasm_code.js"

let manager = undefined; // Ballmanager global, ist nicht so schön aber idc

async function start () {
    await init();
    manager = BallManager.new(canvas.width, canvas.height);
    requestAnimationFrame(sim_loop);
}

function sim_loop () {
    // positions ist ein array: id, x, y, radius
    let positions = manager.update_and_get_positions();
    draw(positions);
    document.getElementById("numballs").innerHTML = positions.length / 7;
    requestAnimationFrame(sim_loop);
}


// Context für den Canvas
const canvas = document.querySelector('canvas');
const ctx = canvas.getContext('2d');
// Env Variabeln
canvas.width = 1000;
canvas.height= 750;

// Malt
function draw(positions) {
    // positions: arr = id, x, y, radius, r, g, b
    // Restest des Canvases
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for(let i = 0; i < positions.length; i = i + 7) {
        // malt einen Kreis nach den Positionen, die letzen 2 args sind wierd idk 
        ctx.beginPath();
        ctx.arc(positions[i + 1], positions[i + 2], positions[i + 3], 0, 2 * Math.PI);
        ctx.fillStyle = 'rgb('+ positions[i + 4] + ',' + positions[i + 5] + ',' + positions[i + 6] +')'; // Was ist diese "+" Syntaxe hä?? #WhyBrendanEich
        ctx.fill();
        ctx.closePath();
    }
    ctx.fill();
}

window.add_ballyay = () => {
    _add_random_balls(5);
    //manager.create_formation(100, 2, 10)
    // &mut self, num_x: i32, num_y: i32, spacing: f32
}

function _add_random_balls (num) { //x: f32, y: f32, dx: f32, dy: f32, radius: f32, mass: f32, r: f32, g: f32, b: f32
    for (let i = 0; i < num; i++) {
        const x = generate_intager_based_range(10, canvas.width - 10);
        const y = generate_intager_based_range(10, canvas.height - 10);
        const dx = generate_intager_based_range(-10, 10);
        const dy = generate_intager_based_range(-10, 10);
        const mass = 10;
        const radius = 10;
        const r = Math.floor(Math.random() * 256);
        const g = Math.floor(Math.random() * 256);
        const b = Math.floor(Math.random() * 256);
        manager.add_ball(x, y, dx, dy, radius, mass, r, g, b);
    }
}
function generate_intager_based_range(min, max) {
    const range = max - min + 1;
    const rand_int_in_range = Math.floor(Math.random() * range)
    return rand_int_in_range + min;
}
document.getElementById("GravityRange").addEventListener("input", (event) => {
    const value = parseFloat(event.target.value) / 100;
    console.log(value);
    manager.set_gravity(value);
});

document.getElementById("ElasticityRange").addEventListener("input", (event) => {
    const value = parseFloat(event.target.value) / 100;
    console.log(value);
    manager.set_elasticity(value);
});


start();

