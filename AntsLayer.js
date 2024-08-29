let storedMap = [];
let d;
let UP    = 0;
let DOWN  = 1;
let LEFT  = 2;
let RIGHT = 3;

let Units = [];


function setup() {
  createCanvas(640, 480);

  // ????? (NE PAS TOUCHER)
  d = pixelDensity();
  for (let i = 0; i < width; i++)
  {
    storedMap[i] = []; // create nested array
    for (let j = 0; j < height; j++)
    {
      storedMap[i][j] = 0;
    }
  }

  //see Terrain.js
  setupTerrain();

  // Build an array of Units
  for (let u = 0; u < 250; u++)
  {
    Units.push(new Unit(0, 0, "Human", "Miner"));
  }

  // TEMP CODE : find somewhere to put Units
  let stop = false;
  let i = 320;
  let j = 240;

  while (i++ < storedMap.length && !stop)
  {
    while (j++ < storedMap[i].length && !stop)
    {
      if (storedMap[i][j] === 0)
      {
        for (let u = 0; u < Units.length; u++)
          Units[u].coords = [i, j];
        stop = true;
      }
    }
  }
}

function draw() {
  background(20, 16, 28);


  drawMap();
  //    drawBuildings();
  drawUnits();
}
