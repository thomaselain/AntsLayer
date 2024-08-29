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

      if (c <= mineral.rarity)
        storePixel(mineral.type, x, y);
    }
  }
}
