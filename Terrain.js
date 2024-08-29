class Mineral {
  constructor(rarity, type) {
    this.rarity = rarity;
    this.type = type;
  }
}

let air;
let iron;
let gold;
let water;
let rocks;

function setupTerrain()
{
  // Listing mineral types and their properties
  air = new Mineral(200, 0);
  iron = new Mineral(50, 1);
  gold = new Mineral(25, 2);
  water = new Mineral(50, 3);
  rocks = new Mineral(170, 4);

  createRessourceMap(air);
  createRessourceMap(rocks);
  createRessourceMap(water);
  createRessourceMap(iron);
  createRessourceMap(gold);
}

// Store minerals in storedMap
function createRessourceMap(mineral) {
  noiseSeed();
  noiseDetail(2, 1.1);
  let noiseLevel = 255;
  let noiseScale = 0.035;
  for (let y = 0; y < height; y += 1) {
    for (let x = 0; x < width; x += 1) {
      let nx = noiseScale * x; //* (x - height / 2) * height;
      let ny = noiseScale * y; //* (y - width / 2) * width;

      let c = noiseLevel * noise(nx + (1 * 0.000001 + 10000) * (mineral.rarity - 100 * 100), ny + (1 * 0.000001 + 1000) * (100*mineral.rarity - 100));


      // TODO dégradé de terre vers la pierre
      if (mineral.type===4 && c < mineral.rarity /2)
        storePixel(mineral.type, x, y);


      else if (c <= mineral.rarity)
        storePixel(mineral.type, x, y);
    }
  }
}
