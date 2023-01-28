let selectedTile = 1;
let WIDTH = 15
let HEIGHT = 10
let BRUSH_SCALE = 3;

// initialize map arrays
let dim = [WIDTH, HEIGHT]
let floor = new Array(HEIGHT)

for (let i = 0; i < HEIGHT; i++) {
    floor[i] = new Array(WIDTH)
}

window.onload = function () {
    for (let i = 1; i <= 25; ++i) {
        document.getElementById("swatches").insertAdjacentHTML('beforeend', `<div class='swatch' style='--nth-swatch: ${i};' onclick='selectSwatch(${i})'></div>`)
    }

    for (let i = 0; i < HEIGHT; ++i) {
        for (let j = 0; j < WIDTH; ++j) {
            document.getElementById("grid").insertAdjacentHTML('beforeend', 
            `<div class="tile" id="${[i,j]}" onclick="this.style.backgroundPosition=\'top left calc(calc(-16px * \' + selectedTile + \')* var(--brush-scale))\';updateMapArray(this.id);"></div>`)
            floor[i][j] = selectedTile
        }
    }
}

function printMap() {
    console.log(floor.toString())
}

function selectSwatch(n) {
    selectedTile = n;
    console.log("Selected swatch: " + selectedTile)
}

function expandMap(dir) {
    let size = +document.getElementById("expandSize").value
    let tmp
    switch (dir) {
        case 0: // north
            // clone floor array
            tmp = [...floor].map(row => [...row])
            HEIGHT += size
            // create new main array
            floor = new Array(HEIGHT)
            for (let i = 0; i < HEIGHT; i++) {
                floor[i] = new Array(WIDTH).fill(1)
            }
            // shift and copy old floor array
            for (let i = size; i < HEIGHT; i++) {
                for (let j = 0; j < WIDTH; j++) {
                    floor[i][j] = tmp[i-size][j]
                }
            }
            break;
        case 1: // east
            // clone floor array
            tmp = [...floor].map(row => [...row])
            WIDTH += size
            // create new main array
            floor = new Array(HEIGHT)
            for (let i = 0; i < HEIGHT; i++) {
                floor[i] = new Array(WIDTH).fill(1)
            }
            // shift and copy old floor array
            for (let i = 0; i < HEIGHT; i++) {
                for (let j = 0; j < WIDTH-size; j++) {
                    floor[i][j] = tmp[i][j]
                }
            }
            break;
        case 2: // south
            // clone floor array
            tmp = [...floor].map(row => [...row])
            HEIGHT += size
            // create new main array
            floor = new Array(HEIGHT)
            for (let i = 0; i < HEIGHT; i++) {
                floor[i] = new Array(WIDTH).fill(1)
            }
            // shift and copy old floor array
            for (let i = 0; i < HEIGHT-size; i++) {
                for (let j = 0; j < WIDTH; j++) {
                    floor[i][j] = tmp[i][j]
                }
            }
            break;
        case 3: // west
            // clone floor array
            tmp = [...floor].map(row => [...row])
            WIDTH += size
            // create new main array
            floor = new Array(HEIGHT)
            for (let i = 0; i < HEIGHT; i++) {
                floor[i] = new Array(WIDTH).fill(1)
            }
            // shift and copy old floor array
            for (let i = 0; i < HEIGHT; i++) {
                for (let j = 0; j < WIDTH; j++) {
                    floor[i][j] = tmp[i][j-size]
                }
            }
            break;
        default:
    }

    redraw();
}

function updateMapArray(id) {
    // update map array to the selected tile
    id = id.split(',');
    floor[id[0]][id[1]] = selectedTile
}

function redraw() {
    const element = document.querySelector('.grid');
    element.innerHTML = '';
    for (let i = 0; i < HEIGHT; ++i) {
        for (let j = 0; j < WIDTH; ++j) {
            document.getElementById("grid").insertAdjacentHTML('beforeend', 
            `<div 
            class="tile" id="${[i,j]}" 
            onclick="this.style.backgroundPosition=\'top left calc(calc(-16px * \' + selectedTile + \') * var(--brush-scale))\';updateMapArray(this.id);"
            style='background-position: top left calc(calc(-16px * \' + ${floor[i][j]} + \') * var(--brush-scale));'"></div>`)
        }
    }

    element.style.width = WIDTH*16*BRUSH_SCALE + 'px';
    element.style.height = HEIGHT*16*BRUSH_SCALE + 'px';
}