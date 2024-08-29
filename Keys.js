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
