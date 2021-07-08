// import * as wasm from "wasm-game-of-life";
import { Universe, Cell } from "wasm-game-of-life";

// wasm.greet("tahsin");

const CELL_SIZE = 15; // px
const GRID_COLOR = "#8bdeee";
const DEAD_COLOR = "#88c0d0";
const ALIVE_COLOR = "#5e81ac";


// Construct the universe, and get its width and height.
const n = 70;
// const universe = Universe.new(128, 72);
const universe = Universe.new(150, 100);
// const universe = Universe.new(256, 144);
// const universe = Universe.new(512, 144 * 2);
// const universe = Universe.new(70, 50);
// const universe = Universe.new(n, n);
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life");
const slider = document.getElementById("range");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let rangess = 1;

slider.oninput = () => {
    rangess = slider.value;
    // console.log(range)
}
let animationId = null;
const renderLoop = () => {
    // debugger;
    fps.render();
    for (let i = 0; i <= rangess; i++) {
        universe.tick();
    }

    drawCells();
    // drawGrid();

    animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");
const resetButton = document.getElementById("reset");
const dead_button = document.getElementById("dead");

resetButton.addEventListener("click", event => {
    universe.set_random();
    if (isPaused) {
        drawCells();
        // drawGrid();
    }
});

dead_button.addEventListener("click", event => {
    universe.ded();
    if (isPaused) {
        drawCells();
        // drawGrid();
    }
});

const play = () => {
    playPauseButton.textContent = "⏸ Pause";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶ Resume";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});



const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
    ctx.lineWidth = 1.75;
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// ...

const getIndex = (row, column) => {
    return row * width + column;
};

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

const drawCells = () => {
    const cellsPtr = universe.cells();

    // This is updated!
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);
    // console.log(cells);
    ctx.beginPath();

    ctx.fillStyle = DEAD_COLOR;

    ctx.fillRect(
        0,
        0,
        canvas.width,
        canvas.height
    );
    // for (let row = 0; row < height; row++) {
    //     for (let col = 0; col < width; col++) {
    //         const idx = getIndex(row, col);
    //         // console.log(cells[idx]);
    //         if (!bitIsSet(idx, cells)) {
    //             continue;
    //         }

    //         ctx.fillRect(
    //             col * (CELL_SIZE + 1) + 1,
    //             row * (CELL_SIZE + 1) + 1,
    //             CELL_SIZE,
    //             CELL_SIZE
    //         );
    //     }
    // }

    // Dead cells.
    ctx.fillStyle = ALIVE_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (!bitIsSet(idx, cells)) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};


canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    if (event.ctrlKey && event.shiftKey) {
        universe.gosper_gun(row, col);
    }
    else if (event.ctrlKey) {
        universe.add_glider(row, col);
    }
    else if (event.shiftKey) {
        universe.add_pulser(row, col);
    }
    else {
        universe.toggle_cell(row, col);
    }

    // drawGrid();
    drawCells();
});


const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.fps.textContent = `Average fps last 100 frames = ${Math.round(mean)}`.trim();
        //         this.fps.textContent = `
        //   Frames per Second:
        //            latest = ${Math.round(fps)}
        //   avg of last 100 = ${Math.round(mean)}
        //   min of last 100 = ${Math.round(min)}
        //   max of last 100 = ${Math.round(max)}
        //   `.trim();
    }
};

drawCells();
// drawGrid();
play();
// requestAnimationFrame(renderLoop);