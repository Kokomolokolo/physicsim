// Ein einfacher Physik Simulator mit Rust im wasm und auch mit Rocket Backend
// TODO: Alles schreib ich jetzt nicht alles hin

import init, { get_positions } from "./wasm_code.js"

async function start () {
    await init();
    requestAnimationFrame(sim_loop)
}

function sim_loop () {
    // update(); //ruft die rust funktionen auf, um updates über die physik zu bekommen

    const positions = get_positions(); // ruft die positonen auf die alle Kugeln haben, gibt nur daten zurück
                                      // get_positon() gibt ein array [x, y]
    draw(positions); // Malt neue Positionen

    requestAnimationFrame(sim_loop)
}


// Context für den Canvas
const canvas = document.querySelector('canvas');
const ctx = canvas.getContext('2d');
// Env Variabeln
canvas.width = 1000;
canvas.height= 750;
let RADIUS = 10;

// Malt
function draw(positions) {
    // Restest des Canvases
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // malt einen Kreis nach den Positionen, die letzen 2 args sind wierd idk 
    ctx.arc(positions[0], positions[1], RADIUS, 0, 2 * Math.PI);
    ctx.fillStyle = "red";
    ctx.fill();
}

start();