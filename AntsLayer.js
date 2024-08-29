//test
let Thomas = new Unit(320, 240, "Human", "Warrior");

let storedMap = [];
let d;

function setup() {
  createCanvas(640, 480);

  // ????? (NE PAS TOUCHER)
  d = pixelDensity();
  for (let i = 0; i < width; i++) {
    storedMap[i] = []; // create nested array
    for (let j = 0; j < height; j++) {
      storedMap[i][j] = 0;
    }
  }

  // Listing mineral types and their properties
  //    air = new Mineral(50, 0);
  iron = new Mineral(50, 1);
  gold = new Mineral(25, 2);
  water = new Mineral(140, 3);
  rocks = new Mineral(186, 4);

  createRessourceMap(water);
  createRessourceMap(rocks);
  createRessourceMap(iron);
  createRessourceMap(gold);

  drawMap();
}

function draw() {
  background(20, 16, 28);
  drawMap();
  //    drawBuildings();
  drawUnits();
}
