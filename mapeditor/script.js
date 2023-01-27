let selectedTile = 1;
let div = '<div class="tile" id="tile1" onclick="this.style.backgroundPosition=\'top left calc(calc(-16px * \' + selectedTile + \')* var(--brush-scale))\';"></div>';

function printMap() {
    console.log("0000000000000000000000000000\n0000000");
}

function selectSwatch(n) {
    selectedTile = n;
    console.log(selectedTile)
}

function fillTile() {
    document.getElementById('tile1').style.backgroundPosition="top left calc(calc(-16px * " + selectedTile + ")* var(--brush-scale))";
}

function addTile() {
    document.getElementById("grid").insertAdjacentHTML('beforeend', div)
}