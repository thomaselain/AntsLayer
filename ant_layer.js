class Mineral {
    constructor(rarity, tint) {   
        this.rarity = rarity;
        this.tint = tint;
    }
}

let iron;
let gold;
let water;
let rocks;

let d;

function setup() {
    createCanvas(640, 480);
    
    d = pixelDensity();
    iron = new Mineral(166, color(220, 210, 180));
    gold = new Mineral(151, color(220, 180, 30));
    water = new Mineral(171, color(30, 50, 210));
    rocks = new Mineral(186, color(98, 40, 30));
    
    background(20, 16, 30);
    createRessourceMap(iron);
    createRessourceMap(gold);
    createRessourceMap(water);
    createRessourceMap(rocks);
}

function setPixel(c, x, y, light) {
   for (let i = 0; i < d; i += 1) {
    for (let j = 0; j < d; j += 1) {
      let index = 4 * ((y * d + j) * width * d + (x * d + i));
      pixels[index] = red(c) * light / 255;
      pixels[index + 1] = green(c) * light / 255;
      pixels[index + 2] = blue(c) * light / 255;
      pixels[index + 3] = alpha(c) * light / 255;
    }
  }
    //pixels[y * width + x + 0] = red(c);
    //pixels[y * width + x + 1] = green(c);
    //pixels[y * width + x + 2] = blue(c);
    //pixels[y * width + x + 3] = alpha(c);
}

function createRessourceMap(mineral) {
    // Set the noise level and scale.
    noiseSeed(mineral.rarity);
    noiseDetail(2, 1.1);
    loadPixels();
    let noiseLevel = 255;
    let noiseScale = 0.02;
    // Iterate from top to bottom.
    for (let y = 0; y < height; y += 1) {
        // Iterate from left to right.
        for (let x = 0; x < width; x += 1) {
            // Scale the input coordinates.
            let nx = noiseScale * x; (x - height / 2) * height;
            let ny = noiseScale * y; (y - width / 2) * width;

            let c = noiseLevel * noise(nx + (1 * 0.000001 + 10000) * (mineral.rarity - 100 * 100), ny + (1 * 0.000001 + 1000) * (100*mineral.rarity - 100));

            // Draw the point.
            if (c + 3 > mineral.rarity && c - 3 < mineral.rarity) 
            {
                setPixel(color(0), x, y, 255);
                //stroke(mineral.tint);
                //point(x, y);
            }
            else if (c >= mineral.rarity) 
            {
                setPixel(mineral.tint, x, y, 255);
                //stroke(mineral.tint);
                //point(x, y);
            }

        }
    }
    updatePixels();
}

function draw() {
    
    //background(40, 32, 59);
    //createRessourceMap(iron);
    //createRessourceMap(gold);
    //createRessourceMap(water);
    //createRessourceMap(rocks);
}
