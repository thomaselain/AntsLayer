
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
