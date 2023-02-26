const BRUSH_SCALE = 3;

var SELECTED_TILE = floors[1];
var WIDTH = 15;
var HEIGHT = 10;

// initialize map object:
var map = {
    w: WIDTH,
    h: HEIGHT,
    layers: [],
    init: function() {
        this.layers.push(initArray(this.w, this.h, floors[2])); // 0: floor
        this.layers.push(initArray(this.w, this.h, empty)); // 1: walls
        this.layers.push(initArray(this.w, this.h, empty)); // 2: objects
        return this;
    }
}.init()

function initArray(w, h, fill) {
    f = new Array(h);
    for (var i = 0; i < h; i++) {
        f[i] = new Array(w).fill(fill);
    }
    return f;
}

function showLayer(layer) {
    c = document.getElementById(layer);
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
    // load the swatches grids once on load time
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
    
    // initial redraw of the map
    redrawMap();
}

function selectSwatch(id) {
    i = id.split('_');
    SELECTED_TILE = i[0] == 'floor' ? floors[i[1]] : i[0] == 'wall' ? walls[i[1]] : objects[i[1]];
    document.getElementById(id).selected = true;
    console.log("Selected swatch: " + SELECTED_TILE.name)
}

function exportMap() {
    var zip = new JSZip(); 

    var floorText = ""
    var wallsText = ""
    var collisionText = ""

    var c = false

    // convert object-based map to string format
    for (var i = 0; i < map.h; i++) {
        for (var j = 0; j < map.w; j++) {
            // fill each layer string with tile IDs from layer array
            floorText += j==map.w-1 ? map.layers[0][i][j].id : map.layers[0][i][j].id + ' '
            wallsText += j==map.w-1 ? map.layers[1][i][j].id : map.layers[1][i][j].id + ' '
            
            // check if a cell is collidable on any of the three layers
            c = map.layers[0][i][j].hasCollision
            || map.layers[1][i][j].hasCollision
            || map.layers[2][i][j].hasCollision

            // type conversion to integer from boolean
            c = c === true ? 1 : 0
            collisionText += j==map.w-1 ? c : c + ' '
        }
        floorText += '\n'
        wallsText  += '\n'
        collisionText  += '\n'
    }

    // create folder and new files in zip
    var folder = zip.folder("map");  
    folder.file("floor.txt", floorText)
    folder.file("walls.txt", wallsText)
    folder.file("collision.txt", collisionText)

    // download zip blobs asynchronously:
    folder.generateAsync({type: "blob"}).then(function(file) {
        var e = document.createElement('a'),
        url = URL.createObjectURL(file);
        e.href = url;
        e.download = "map.zip";
        document.body.appendChild(e);
        e.click();
        setTimeout(function() {
            document.body.removeChild(e);
            window.URL.revokeObjectURL(url);  
        }, 0);
    })
}

async function importMap() {
    files = document.getElementById("file-selector").files
    var floor, wall, object;

    // Check file names
    for(f in files) {
        if(files[f].name == "floor.txt") {floor = files[f]}
        if(files[f].name == "walls.txt") {wall = files[f]}
        // if(files[f].name == "collision.txt") {object = files[f]}
    }

    processFiles(floor, wall).then((res) => {
        console.log(res);
        WIDTH = res[0][0].length; 
        HEIGHT = res[0].length;
        map.w = WIDTH;
        map.h = HEIGHT;
        map.layers = res;
        redrawMap();
    })
}

async function processFiles(f, w) {
    var layers = [];
    
    try {
        for(arg in arguments) {
            var a = [];
            await readFileAsync(arguments[arg]).then((res) => {
                buf = arrayBufferToString(res);
                var lines = buf.split(/[\r\n]+/g);
                lines.forEach(function(l) {
                    if(String(l).length > 0) {
                        var line = String(l).split(/[\b\s\b]+/g)
                        if(arg == 0) {
                            a.push(line.map(x => floors[x]));
                        } else if (arg == 1) {
                            a.push(line.map(x => walls[x]));
                        }
                    }
                });
                layers.push(a)
            });
        }
        // temporary fix for object/layer mismatch (creates new empty object layer)
        // we need to create a new object structure
        // It is no longer possible to import object layer
        layers.push(initArray(WIDTH, HEIGHT, empty));
        return layers;
    
    } catch (err) {
        console.log(err);
    }
}

// process files helper functions
function readFileAsync(file) {
    return new Promise((resolve, reject) => {
        var reader = new FileReader();
            reader.onload = () => {
            resolve(reader.result);
        };

        reader.onerror = reject;
        reader.readAsArrayBuffer(file);
    })
}
  
function arrayBufferToString(arrayBuffer, decoderType = 'utf-8') {
    var decoder = new TextDecoder(decoderType);
    return decoder.decode(arrayBuffer);
}

function expandMap(dir) {
    var size = +document.getElementById("expandSize").value
    var tmp;
    switch (dir) {
        case 0: // north
            console.log("North + " + size);
            for (l of map.layers) {
                // clone array and remove
                tmp = map.layers.shift();
                // create new main array
                l = initArray(map.w, map.h + size, empty)
                // shift and copy old floor array
                for (var i = size; i < map.h + size; i++) {
                    for (var j = 0; j < map.w; j++) {
                        l[i][j] = tmp[i-size][j]
                    }
                }
                map.layers.push(l);
            }
            map.h += size;
            HEIGHT += size;
            break;
        case 1: // east
            console.log("East + " + size);
            for (l of map.layers) {
                // clone array and remove
                tmp = map.layers.shift();
                // create new main array
                l = initArray(map.w+size, map.h, empty)
                // shift and copy old floor array
                for (var i = 0; i < map.h; i++) {
                    for (var j = 0; j < map.w; j++) {
                        l[i][j] = tmp[i][j]
                    }
                }
                map.layers.push(l);
            }
            map.w += size;
            WIDTH += size;  
            break;
        case 2: // south
            console.log("South + " + size);
            for (l of map.layers) {
                // clone array and remove
                tmp = map.layers.shift();
                // create new main array
                l = initArray(map.w, map.h+size, empty)
                // shift and copy old floor array
                for (var i = 0; i < map.h; i++) {
                    for (var j = 0; j < map.w; j++) {
                        l[i][j] = tmp[i][j]
                    }
                }
                map.layers.push(l);
            }
            map.h += size;
            HEIGHT += size;
            break;
        case 3: // west
        console.log("West + " + size);
            for (l of map.layers) {
                // clone array and remove
                tmp = map.layers.shift();
                // create new main array
                l = initArray(map.w+size, map.h, empty)
                // shift and copy old floor array
                for (var i = 0; i < map.h; i++) {
                    for (var j = size; j < map.w + size; j++) {
                        l[i][j] = tmp[i][j-size]
                    }
                }
                map.layers.push(l);
            }
            map.w += size;
            WIDTH += size;  
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
    console.log(i, l);
    console.log(`${[i[0],i[1],l]}`)
    document.getElementById(`${[i[0],i[1],l]}`).style.backgroundPosition = `${map.layers[l][i[0]][i[1]].x * BRUSH_SCALE * -1}px ${map.layers[l][i[0]][i[1]].y * BRUSH_SCALE * -1}px`
}

function redrawMap() {
    const elements = [document.querySelector('#gridObject'), document.querySelector('#gridFloor'), document.querySelector('#gridWall')]
    for (e of elements) {
        e.innerHTML = '';
        // adjust the width + height of grid to match the new map dimensions
        e.style.width = map.w*16*BRUSH_SCALE + 'px';
        e.style.height = map.h*16*BRUSH_SCALE + 'px';
    }

    for (var i = 0; i < map.h; ++i) {
        for (var j = 0; j < map.w; ++j) {
            document.getElementById("gridFloor").insertAdjacentHTML('beforeend', `<div class="floor" id="${[i,j,0]}" onclick="updateMapArray(this.id)"></div>`)
            document.getElementById(`${[i,j,0]}`).style.backgroundPosition = `${map.layers[0][i][j].x * BRUSH_SCALE * -1}px ${map.layers[0][i][j].y * BRUSH_SCALE * -1}px`
            document.getElementById("gridWall").insertAdjacentHTML('beforeend', `<div class="wall" id="${[i,j,1]}" onclick="updateMapArray(this.id)"></div>`)
            document.getElementById(`${[i,j,1]}`).style.backgroundPosition = `${map.layers[1][i][j].x * BRUSH_SCALE * -1}px ${map.layers[1][i][j].y * BRUSH_SCALE * -1}px`
            document.getElementById("gridObject").insertAdjacentHTML('beforeend', `<div class="object" id="${[i,j,2]}" onclick="updateMapArray(this.id)"></div>`)
            document.getElementById(`${[i,j,2]}`).style.backgroundPosition = `${map.layers[2][i][j].x * BRUSH_SCALE * -1}px ${map.layers[2][i][j].y * BRUSH_SCALE * -1}px`
        }
    }
}