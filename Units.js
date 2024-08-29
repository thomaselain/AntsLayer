class Unit {
  constructor(x, y, race, type) {
    this.coords = [x || 0, y || 0];
    this.race = race;
    this.type = type;
  }
  draw()
  {
//    setPixel(color(35, 206, 235), this.coords[0]+1, this.coords[1], 255);
//    setPixel(color(35, 206, 235), this.coords[0]-1, this.coords[1], 255);
    setPixel(color(200, 0, 0), this.coords[0], this.coords[1], 255);
//    setPixel(color(35, 206, 235), this.coords[0], this.coords[1]+1, 255);
//    setPixel(color(35, 206, 235), this.coords[0], this.coords[1]-1, 255);
    console.log(this.coords);
  }
  
  dig(strength)
  {
    ;
  }

  move(direction)
  {
    console.log(direction);
    switch(direction)
    {
    case UP:
      if (storedMap[this.coords[0]][this.coords[1]-1] === 0)
      {
        this.coords[1]--;
      }
      break;
    case DOWN:
      if (storedMap[this.coords[0]][this.coords[1]+1] === 0)
      {
        this.coords[1]++;
      }
      break;
    case LEFT:
      if (storedMap[this.coords[0]-1][this.coords[1]] === 0)
      {
        this.coords[0]--;
      }
      break;
    case RIGHT:
      if (storedMap[this.coords[0]+1][this.coords[1]] === 0)
      {
        this.coords[0]++;
      }
      break;
    default:
      break;
    }
  }
}

function drawUnits()
{
  loadPixels();

  for (let u = 0; u < Units.length; u++)
  {
    randomSeed();
    Units[u].move(round(random(0, 4)));
    Units[u].draw();
  }

  updatePixels();
}
