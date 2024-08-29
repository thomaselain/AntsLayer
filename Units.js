class Unit {
    constructor(x, y, race, type) {
        this.x = x;
        this.y = y;
        this.race = race;
        this.type = type;
    }
}

function drawUnits()
{
  loadPixels();

  setPixel(color(35, 206, 235), Thomas.x, Thomas.y+1, 255);
  setPixel(color(35, 206, 235), Thomas.x+1, Thomas.y, 255);
  setPixel(color(255, 255, 255), Thomas.x, Thomas.y, 255);
  setPixel(color(35, 206, 235), Thomas.x, Thomas.y-1, 255);
  setPixel(color(35, 206, 235), Thomas.x-1, Thomas.y, 255);

  updatePixels();
}
