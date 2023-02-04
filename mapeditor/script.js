const BRUSH_SCALE = 3;
const floors = [
        {
            "name": "NONE",
            "id": 0,
            "x": 0,
            "y": 0
        },
        {
            "name": "GRASS1",
            "id": 1,
            "x": 32,
            "y": 0
        },
        {
            "name": "GRASS2",
            "id": 2,
            "x": 48,
            "y": 0
        },
        {
            "name": "WATER1",
            "id": 3,
            "x": 16,
            "y": 64
        },
        {
            "name": "WGTL",
            "id": 4,
            "x": 0,
            "y": 48
        },
        {
            "name": "WGT",
            "id": 5,
            "x": 16,
            "y": 48
        },
        {
            "name": "WGTR",
            "id": 6,
            "x": 32,
            "y": 48
        },
        {
            "name": "WGL",
            "id": 7,
            "x": 0,
            "y": 64
        },
        {
            "name": "WGR",
            "id": 8,
            "x": 32,
            "y": 64
        },
        {
            "name": "WGBL",
            "id": 9,
            "x": 0,
            "y": 80
        },
        {
            "name": "WGB",
            "id": 10,
            "x": 16,
            "y": 80
        },
        {
            "name": "WGBR",
            "id": 11,
            "x": 32,
            "y": 80
        },
        {
            "name": "GWTL",
            "id": 12,
            "x": 48,
            "y": 48
        },
        {
            "name": "GWTR",
            "id": 13,
            "x": 80,
            "y": 48
        },
        {
            "name": "GWBL",
            "id": 14,
            "x": 48,
            "y": 80
        },
        {
            "name": "GWBR",
            "id": 15,
            "x": 80,
            "y": 80
        },
        {
            "name": "FB1",
            "id": 16,
            "x": 112,
            "y": 0
        },
    ]
const walls = [
        {
            "name": "NONE",
            "id": 0,
            "x": 0,
            "y": 0
        },
        {
            "name": "WOODL",
            "id": 1,
            "x": 128,
            "y": 0
        },
        {
            "name": "WOODR",
            "id": 2,
            "x": 160,
            "y": 0
        },
        {
            "name": "WOOD",
            "id": 3,
            "x": 134,
            "y": 0
        },
    ]
const objects = [
        {
            "name": "NONE",
            "id": 0,
            "x": 0,
            "y": 0
        },
        {
            "name": "BERRY1",
            "id": 1,
            "x": 16,
            "y": 0
        },
        {
            "name": "BERRY2",
            "id": 2,
            "x": 32,
            "y": 0
        },
    ]
const empty = {}

let SELECTED_TILE = floors[1];
let WIDTH = 15;
let HEIGHT = 10;

// initialize map object
let map = {
    w: WIDTH,
    h: HEIGHT,
    layers: [],
    init: function() {
        this.dim = [this.w, this.h];
        this.layers.push(initArray(this.w, this.h, floors[2])); // 0: floor
        this.layers.push(initArray(this.w, this.h, empty)); // 1: walls
        this.layers.push(initArray(this.w, this.h, empty)); // 2: objects
        return this;
    }
}.init()

function initArray(w, h, fill) {
    f = new Array(h);
    for (let i = 0; i < h; i++) {
        f[i] = new Array(w).fill(fill);
    }
    return f;
}

function showLayer(layer) {
    c = document.getElementById(layer);
    console.log(layer)
    switch (layer) {
        case "floorCheck":
            document.getElementById("gridFloor").style.display = c.checked ? "block" : "none";
            break;
        case "wallCheck":
            document.getElementById("gridWall").style.display = c.checked ? "block" : "none";
            break;
        case "objectCheck":
            document.getElementById("gridObject").style.display = c.checked ? "block" : "none";
            break;                         
    }
}

window.onload = function () {
    for (f of floors) {
        document.getElementById("floorSwatches")
        .insertAdjacentHTML('beforeend', `<div class='floor' id="floor_${f.id}" onclick='selectSwatch(this.id)'></div>`)
        document.getElementById(`floor_${f.id}`).style.backgroundPosition = `${f.x * BRUSH_SCALE * -1}px ${f.y * BRUSH_SCALE * -1}px`
    }
    for (w of walls) {
        document.getElementById("wallSwatches")
        .insertAdjacentHTML('beforeend', `<div class='wall' id="wall_${w.id}" onclick='selectSwatch(this.id)'></div>`)
        document.getElementById(`wall_${w.id}`).style.backgroundPosition = `${w.x * BRUSH_SCALE * -1}px ${w.y * BRUSH_SCALE * -1}px`
    }
    for (o of objects) {
        document.getElementById("objectSwatches")
        .insertAdjacentHTML('beforeend', `<div class='object' id="object_${o.id}" onclick='selectSwatch(this.id)'></div>`)
        document.getElementById(`object_${o.id}`).style.backgroundPosition = `${o.x * BRUSH_SCALE * -1}px ${o.y * BRUSH_SCALE * -1}px`
    }
    
    redrawMap();
}

function exportMap() {
    filename = document.getElementById('filename').value
    console.table(map.layers.floorArray)
    let text = ''

    for (let i = 0; i < map.h; i++) {
        for (let j = 0; j < map.w; j++) {
            text += j==map.w-1 ? map.layers[0][i][j] : map.layers[0][i][j] + ' '
        }
        text += '\n'
    }

    // download:
    var element = document.createElement('a');
    element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
    element.setAttribute('download', filename);
  
    element.style.display = 'none';
    document.body.appendChild(element);
  
    element.click();

    document.body.removeChild(element);
}

function importMap() {}

function selectSwatch(id) {
    id = id.split('_');
    SELECTED_TILE = id[0] == 'floor' ? floors[id[1]] : id[0] == 'wall' ? walls[id[1]] : objects[id[1]];
    console.log(SELECTED_TILE)
    console.log("Selected swatch: " + SELECTED_TILE.name)
}

function expandMap(dir) {
    let size = +document.getElementById("expandSize").value
    let tmp;
    switch (dir) {
        case 0: // north
            console.log("North + " + size);
            for (l of map.layers) {
                // clone array and remove
                tmp = map.layers.shift();
                // create new main array
                l = initArray(map.w, map.h + size, empty)
                // shift and copy old floor array
                for (let i = size; i < map.h + size; i++) {
                    for (let j = 0; j < map.w; j++) {
                        l[i][j] = tmp[i-size][j]
                    }
                }
                map.layers.push(l);
            }
            map.h += size;
            break;
        case 1: // east
            console.log("East + " + size);
            for (l of map.layers) {
                // clone array and remove
                tmp = map.layers.shift();
                // create new main array
                l = initArray(map.w+size, map.h, empty)
                // shift and copy old floor array
                for (let i = 0; i < map.h; i++) {
                    for (let j = 0; j < map.w; j++) {
                        l[i][j] = tmp[i][j]
                    }
                }
                map.layers.push(l);
            }
            map.w += size;
            break;
        case 2: // south
            console.log("South + " + size);
            for (l of map.layers) {
                // clone array and remove
                tmp = map.layers.shift();
                // create new main array
                l = initArray(map.w, map.h+size, empty)
                // shift and copy old floor array
                for (let i = 0; i < map.h; i++) {
                    for (let j = 0; j < map.w; j++) {
                        l[i][j] = tmp[i][j]
                    }
                }
                map.layers.push(l);
            }
            map.h += size;
            break;
        case 3: // west
        console.log("West + " + size);
            for (l of map.layers) {
                // clone array and remove
                tmp = map.layers.shift();
                // create new main array
                l = initArray(map.w+size, map.h, empty)
                // shift and copy old floor array
                for (let i = 0; i < map.h; i++) {
                    for (let j = size; j < map.w + size; j++) {
                        l[i][j] = tmp[i][j-size]
                    }
                }
                map.layers.push(l);
            }
            map.w += size;
            break;
        default:
    }
    redrawMap();
}

function updateMapArray(id) {
    // update map array to the selected tile
    i = id.split(',');
    layer = floors.includes(SELECTED_TILE) ? 0 : walls.includes(SELECTED_TILE) ? 1 : 2;
    map.layers[layer][i[0]][i[1]] = SELECTED_TILE
    redrawTile(id, layer)
}

function redrawTile(id, l) {
    i = id.split(',');
    document.getElementById(`${[id[0],i[1],l]}`).style.backgroundPosition = `${map.layers[l][i[0]][i[1]].x * BRUSH_SCALE * -1}px ${map.layers[l][i[0]][i[1]].y * BRUSH_SCALE * -1}px`
}

function redrawMap() {
    const elements = [document.querySelector('.gridObject'), document.querySelector('.gridFloor'), document.querySelector('.gridWall')]
    for (e of elements) {
        e.innerHTML = '';
        // adjust the width + height of grid to match the new map dimensions
        e.style.width = map.w*16*BRUSH_SCALE + 'px';
        e.style.height = map.h*16*BRUSH_SCALE + 'px';
    }

    for (let i = 0; i < map.h; ++i) {
        for (let j = 0; j < map.w; ++j) {
            document.getElementById("gridFloor").insertAdjacentHTML('beforeend', `<div class="floor" id="${[i,j,0]}" onclick="updateMapArray(this.id)"></div>`)
            document.getElementById(`${[i,j,0]}`).style.backgroundPosition = `${map.layers[0][i][j].x * BRUSH_SCALE * -1}px ${map.layers[0][i][j].y * BRUSH_SCALE * -1}px`
            document.getElementById("gridWall").insertAdjacentHTML('beforeend', `<div class="wall" id="${[i,j,1]}" onclick="updateMapArray(this.id)"></div>`)
            document.getElementById(`${[i,j,1]}`).style.backgroundPosition = `${map.layers[1][i][j].x * BRUSH_SCALE * -1}px ${map.layers[1][i][j].y * BRUSH_SCALE * -1}px`
            document.getElementById("gridObject").insertAdjacentHTML('beforeend', `<div class="object" id="${[i,j,2]}" onclick="updateMapArray(this.id)"></div>`)
            document.getElementById(`${[i,j,2]}`).style.backgroundPosition = `${map.layers[2][i][j].x * BRUSH_SCALE * -1}px ${map.layers[2][i][j].y * BRUSH_SCALE * -1}px`
        }
    }
}