/****************************************************/
// Created by: Tucker McCulloch
// Description: JavaScript functions for generating GUI and 
// performing logic for editing, saving, and loading maps for the Rustymon map editor
/****************************************************/

// Defines the scaaling factor for the map editor UI
const BRUSH_SCALE = 3;

// Defines the tile that is currently being added to the map.
var SELECTED_TILE = floors[1];
// Defines the dimensions of the map
var WIDTH = 44;
var HEIGHT = 34;
// Defines the size of the input tilemap in square pixels
var TILEMAP_SIZE = 512;

// Initialize map object
// A map object represents the different components that make up the total map.
// This includes the dimensions and different layers.
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

// Loop returns a map arrays with some datatype based on some dimensions 
function initArray(w, h, fill) {
    f = new Array(h);
    for (var i = 0; i < h; i++) {
        f[i] = new Array(w).fill(fill);
    }
    return f;
}

// Helper function for front end UI component.
// Hides or shows different layer elements, so if you want to show
// only the walls or only the floors you can do that.
function showLayer(layer) {
    c = document.getElementById(layer);
    switch (layer) {
        case "floorCheck":
            document.getElementById("gridFloor").style.display = c.checked ? "block" : "none"; // change the display CSS w/ ternary
            break;
        case "wallCheck":
            document.getElementById("gridWall").style.display = c.checked ? "block" : "none"; // "                "
            break;
        case "objectCheck":
            document.getElementById("gridObject").style.display = c.checked ? "block" : "none"; // "              "
            break;                         
    }
}

window.onload = function () {
    // Load the swatches grids once on load time:

    // Generate an empty grid of swatches of appropriate size.
    // 
    for(var y = 0; y < TILEMAP_SIZE; y += 16){
        for(var x = 0; x < TILEMAP_SIZE; x += 16){
            document.getElementById("swatches")
            .insertAdjacentHTML('beforeend', `<div class='empty' id="${x}_${y}"></div>`) // creates ids for each swatch based on x/y dimensions
            document.getElementById(`${x}_${y}`).style.backgroundPosition = `0px 0px` // empty swatch at (0,0)
        }
    }

    // For every swatch in the tilemap, set the corresponding empty swatch to that tile position
    // Set the class and create a new click event that will update the 'SELECTED_SWATCH' variable
    // with that particular swatch.
    // Each swatch is defined in the 'tiles.js' file, and each tile (swatch) has its corresponding coordinates stored in itself (x, y).
    for (f of floors) {
        document.getElementById(`${f.x}_${f.y}`).style.backgroundPosition = `${f.x * BRUSH_SCALE * -1}px ${f.y * BRUSH_SCALE * -1}px`
        document.getElementById(`${f.x}_${f.y}`).className = "floor" // set classname
        document.getElementById(`${f.x}_${f.y}`).onclick = function() { // create new onlick function for each individual tile.
            var pos = this.id.split('_');
            selectSwatch(pos[0], pos[1]);
        }
    }
    // same as above but for walls
    for (w of walls) {
        document.getElementById(`${w.x}_${w.y}`).style.backgroundPosition = `${w.x * BRUSH_SCALE * -1}px ${w.y * BRUSH_SCALE * -1}px`
        document.getElementById(`${w.x}_${w.y}`).className = "wall"
        document.getElementById(`${w.x}_${w.y}`).onclick = function() {
            var pos = this.id.split('_');
            selectSwatch(pos[0], pos[1]);
        }
    }
    // same as above but for objects
    for (o of objects) {
        document.getElementById(`${o.x}_${o.y}`).style.backgroundPosition = `${o.x * BRUSH_SCALE * -1}px ${o.y * BRUSH_SCALE * -1}px`
        document.getElementById(`${o.x}_${o.y}`).className = "object"
        document.getElementById(`${o.x}_${o.y}`).onclick = function() {
            var pos = this.id.split('_');
            selectSwatch(pos[0], pos[1]);
        }
    }
    // Removes any excess swatch elements after the last non-empty tile.
    // This is so we don't have all this extra whitespace between the swatches and the editor.
    loop1: for(var y = TILEMAP_SIZE-16; y >= 0; y -= 16){
            for(var x = TILEMAP_SIZE-16; x >= 0; x -= 16){
                var e = document.getElementById(`${x}_${y}`);
                if(e.className != "empty") {
                    break loop1;
                } else {
                    e.remove();
                }
            }
        }

    // Initial redraw of the map.
    // Calling this will read the map data and draw it out.
    redrawMap();
}

// Selects swatch object based on passed in coordinate values.
function selectSwatch(x, y) {
    // There can only be one tile in one position, so it can act as the primary key
    t = floors.find(p => p.x == x && p.y == y) || walls.find(p => p.x == x && p.y == y) || objects.find(p => p.x == x && p.y == y)
    SELECTED_TILE = t;
    console.log(t); // select and log tile
    console.log("Selected swatch: " + SELECTED_TILE.name)
}

// --------------------------------
// START IMPORT/EXPORT CODE
// --------------------------------

// Handles main export functionality
function exportMap() {
    // file contents and Zip initialization
    var zip = new JSZip(); 

    var floorText = ""
    var wallsText = ""
    var collisionText = ""

    var c = false

    // convert object-based map to string format for the files.
    // sets the __text variables
    for (var i = 0; i < map.h; i++) {
        for (var j = 0; j < map.w; j++) {
            // Fill each layer string with tile IDs from layer array.
            // Pulls more information from the tile objects for export.
            floorText += j==map.w-1 ? floors.indexOf(map.layers[0][i][j]) : floors.indexOf(map.layers[0][i][j]) + ' '
            wallsText += j==map.w-1 ? walls.indexOf(map.layers[1][i][j]) : walls.indexOf(map.layers[1][i][j]) + ' '
            
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

    // download zip blobs asynchronously, pass the generated blob into the callback:
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

// Import-map entrypoint
async function importMap() {
    files = document.getElementById("file-selector").files // get filepaths from client UI element
    var floor, wall, object;

    // Check file names (will error on import of incorrectly formatted or name files)
    // Area for improvement would be to add some checking here for that.
    for(f in files) {
        if(files[f].name == "floor.txt") {floor = files[f]}
        if(files[f].name == "walls.txt") {wall = files[f]}
        // if(files[f].name == "collision.txt") {object = files[f]}
    }

    // processFiles asynchronously parses the floor and wall file contents
    processFiles(floor, wall).then((res) => {
        console.log(res);
        WIDTH = res[0][0].length;
        HEIGHT = res[0].length;
        map.w = WIDTH;
        map.h = HEIGHT;
        map.layers = res;
        console.log(WIDTH,HEIGHT)
        redrawMap();
    })
}

async function processFiles(f, w) {
    var layers = [];
    
    try {
        for(arg in arguments) { // for both files (can add more in the future)
            var a = [];
            await readFileAsync(arguments[arg]).then((res) => {
                buf = arrayBufferToString(res);
                var lines = buf.split(/[\r\n]+/g); // splits file buffer into whole lines 
                lines.forEach(function(l) {
                    if(String(l).length > 0) {
                        var line = String(l).split(/[\b\s\b]+/g) // cleans lines and converts string to array
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
// take from the JS FileReader docs page
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
// process files helper functions
// take from the JS FileReader docs page 
function arrayBufferToString(arrayBuffer, decoderType = 'utf-8') {
    var decoder = new TextDecoder(decoderType);
    return decoder.decode(arrayBuffer);
}

// --------------------------------
// END
// --------------------------------

// Adds area to the tilemap by the given amount and cardinality.
function expandMap(dir) {
    var size = +document.getElementById("expandSize").value
    var tmp;
    // For each direction, each layer is appropriately resized and the old tile are positioned correctly
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
    // redraw after the cardinal shift
    redrawMap();
}

function updateMapArray(id) {
    // update map array to the selected tile
    i = id.split(',');
    layer = floors.includes(SELECTED_TILE) ? 0 : walls.includes(SELECTED_TILE) ? 1 : 2; // check which layer the given position is in
    map.layers[layer][i[0]][i[1]] = SELECTED_TILE // update specific tile to match the selected tile
    redrawTile(id, layer) // re-render specific tile
}

function redrawTile(id, l) {
    // re-renders a specific tile instead of redrawing the entire map.
    i = id.split(',');
    console.log(i, l);
    console.log(`${[i[0],i[1],l]}`)
    // rendering is done by referencing the id (ie, the position) of some tile.
    document.getElementById(`${[i[0],i[1],l]}`).style.backgroundPosition = `${map.layers[l][i[0]][i[1]].x * BRUSH_SCALE * -1}px ${map.layers[l][i[0]][i[1]].y * BRUSH_SCALE * -1}px`
}

function redrawMap() {
    // same as redrawTile(), but iterates over all tiles
    const elements = [document.querySelector('#gridObject'), document.querySelector('#gridFloor'), document.querySelector('#gridWall')]
    for (e of elements) {
        e.innerHTML = '';
        // adjust the width + height of grid to match the new map dimensions
        e.style.width = map.w*16*BRUSH_SCALE + 'px';
        e.style.height = map.h*16*BRUSH_SCALE + 'px';
    }

    for (var i = 0; i < map.h; ++i) {
        for (var j = 0; j < map.w; ++j) {
            // update all the elements: 
            document.getElementById("gridFloor").insertAdjacentHTML('beforeend', `<div class="floor" id="${[i,j,0]}" onclick="updateMapArray(this.id)"></div>`)
            document.getElementById(`${[i,j,0]}`).style.backgroundPosition = `${map.layers[0][i][j].x * BRUSH_SCALE * -1}px ${map.layers[0][i][j].y * BRUSH_SCALE * -1}px`
            document.getElementById("gridWall").insertAdjacentHTML('beforeend', `<div class="wall" id="${[i,j,1]}" onclick="updateMapArray(this.id)"></div>`)
            document.getElementById(`${[i,j,1]}`).style.backgroundPosition = `${map.layers[1][i][j].x * BRUSH_SCALE * -1}px ${map.layers[1][i][j].y * BRUSH_SCALE * -1}px`
            document.getElementById("gridObject").insertAdjacentHTML('beforeend', `<div class="object" id="${[i,j,2]}" onclick="updateMapArray(this.id)"></div>`)
            document.getElementById(`${[i,j,2]}`).style.backgroundPosition = `${map.layers[2][i][j].x * BRUSH_SCALE * -1}px ${map.layers[2][i][j].y * BRUSH_SCALE * -1}px`
        }
    }
}