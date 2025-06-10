// Ein einfacher Physik Simulator mit Rust im wasm und auch mit Rocket Backend
// TODO: Alles schreib ich jetzt nicht alles hin

import init, { BallManager } from "./wasm_code.js"

let ballmanager = undefined; // Ballmanager global in js, ist nicht so schön aber idk

async function start () {
    await init();
    ballmanager = BallManager.new(canvas.width, canvas.height)
    //Die vielleicht in eigene funktion
    console.log(ballmanager.add_ball(100, 100, 0, 5, 10));
    console.log(ballmanager.add_ball(200, 100, 2, 5, 10));

    requestAnimationFrame(sim_loop)
}

function sim_loop () {
    // positions ist ein array: id, x, y, radius
    let positions = ballmanager.update_and_get_positions();
    draw(positions);
    document.getElementById("numballs").innerHTML = positions.length / 4
    addABall();
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
    // Restest des Canvases
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for(let i = 0; i < positions.length; i = i + 4) {
        // malt einen Kreis nach den Positionen, die letzen 2 args sind wierd idk 
        ctx.beginPath();
        ctx.arc(positions[i + 1], positions[i + 2], positions[i + 3], 0, 2 * Math.PI);
        ctx.fillStyle = "red";
        ctx.fill();
        ctx.closePath();
    }
    ctx.fill();
}

window.add_ballyay = () => {
    ballmanager.add_ball(200, 100, 10, 3, 10)
}

function addABall() {
    let rand1 = Math.floor(Math.random() * 10) + 1
    let rand2 = Math.floor(Math.random() * 10) + 1
    ballmanager.add_ball(200, 100, rand1, rand2, 10), 100

}

start();

setInterval(ballmanager.add_ball(200, 100, 10, 3, 10), 100)
