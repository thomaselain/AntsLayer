class Mineral {
    constructor(rarity, type) {   
        this.rarity = rarity;
        this.type = type;
    }
}

class Unit {
    constructor(x, y, race, type) {
        this.x = x;
        this.y = y;
        this.race = race;
        this.type = type;
    }
}

//test 
let Thomas = new Unit(320, 240, "Human", "Warrior");


let storedMap = [];
let air;
let iron;
let gold;
let water;
let rocks;

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
}

function storePixel(cellType, x, y)
{
    storedMap[x][y] = cellType;
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

            if (c <= mineral.rarity) 
                storePixel(mineral.type, x, y);
        }
    }
}

function drawMap()
{
    loadPixels();

    // Loop in storedMap
    for (let i = 0; i < storedMap.length; i += 1)
    {
        for (let j = 0; j < storedMap[i].length; j += 1)
        {
            switch (storedMap[i][j])
            {
                case 1: 
                  c = color(220, 210, 180);
                  break;
                case 2:
                  c= color(220, 180, 30);
                  break;
                case 3:
                  c = color(30, 50, 210);
                  break;
                case 4:
                  c = color(98, 40, 30);
                  break;
                //case 0:
                default:
                    c = color(0, 0, 0, 255);
            }
        setPixel(c, i, j, 255);
        }
    }

    updatePixels();
}

// to remove
function keyPressed()
{
     if (keyIsDown(UP_ARROW) === true)
        Thomas.y--;
     if (keyCode === LEFT_ARROW)
        Thomas.x--;
     if (keyCode === DOWN_ARROW)
        Thomas.y++;
     if (keyCode === RIGHT_ARROW)
        Thomas.x++;
    console.log(Thomas.x, Thomas.y);
}

function drawUnits()
{
     loadPixels();

     setPixel(color(35, 206, 235), Thomas.x, Thomas.y+1, 255);
     setPixel(color(35, 206, 235), Thomas.x+1, Thomas.y, 255);
     setPixel(color(255,255, 255), Thomas.x, Thomas.y, 255);
     setPixel(color(35, 206, 235), Thomas.x, Thomas.y-1, 255);
     setPixel(color(35, 206, 235), Thomas.x-1, Thomas.y, 255);
     
     updatePixels();
}

function draw() {
    background(20, 16, 28);
    drawMap();
//    drawBuildings();
    drawUnits();
}
