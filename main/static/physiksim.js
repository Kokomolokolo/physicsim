// Ein einfacher Physik Simulator mit Rust im wasm und auch mit Rocket Backend
// TODO: Bessere Kolisonslogik
//       RGB Farben auf den Bällen

import init, { BallManager } from "./wasm_code.js"

let ballmanager = undefined; // Ballmanager global in js, ist nicht so schön aber idc

async function start () {
    await init();
    ballmanager = BallManager.new(canvas.width, canvas.height)
    requestAnimationFrame(sim_loop)
}

function sim_loop () {
    // positions ist ein array: id, x, y, radius
    let positions = ballmanager.update_and_get_positions();
    draw(positions);
    if (positions) {
        console.log(` ${positions}`)
    } 
    document.getElementById("numballs").innerHTML = positions.length / 7;
    // addABall();
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
    // add_ball: x, y, dx, dy, radius, r, g, b
    _add_random_balls(5);
    // ballmanager.add_ball(300, 400, 7, 4, 10);
    // ballmanager.add_ball(10, 500, 3, 10, 10);
    // ballmanager.add_ball(300, 400, -8, -2, 10);
    // ballmanager.add_ball(300, 400, 2, -5, 10);
    // ballmanager.add_ball(300, 400, -1, 15, 10);
}

function _add_random_balls (num) {
    for (let i = 0; i < num; i++) {
        const x = generate_intager_based_range(10, canvas.width - 10);
        const y = generate_intager_based_range(10, canvas.height - 10);
        const dx = generate_intager_based_range(-10, 10);
        const dy = generate_intager_based_range(-10, 10);
        const r = Math.floor(Math.random() * 256);
        const g = Math.floor(Math.random() * 256);
        const b = Math.floor(Math.random() * 256);
        ballmanager.add_ball(x, y, dx, dy, 10, r, g, b);
    }
}
function generate_intager_based_range(min, max) {
    const range = max - min + 1;
    const rand_int_in_range = Math.floor(Math.random() * range)
    return rand_int_in_range + min;
}

start();

