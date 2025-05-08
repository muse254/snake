# SNAKE

I'm taking inspiration from the [Nokia Snake Game](https://en.wikipedia.org/wiki/Snake_(video_game_genre))

The goal for this project was to use [Aseprite](https://www.aseprite.org) and create a full-game with it.

Below are the set of mind maps I use to plan out the game:

- [ ] The playable grid is in units of 13:16.
- [ ] At any given time there is one fruit in the grid, except when the game is completed.
- [ ] The game is completed when there is only one unit of space not occupied by the snake.
- [ ] The snake starts off with a length of 2 units; one unit for the head and the latter for the tail.
- [ ] The game only responds to the arrow keys to move the snake and the enter key to pause the game.
- [ ] The fruit spawns at a random location in the grid.
- [ ] The snake moves at a constant speed of 1 unit per second, with the speed increasing linearly to the length of the snake.
- [ ] The snake can move in any direction except the opposite direction of the current direction.
- [ ] The snake can move through the walls of the grid and appear on the opposite side.
- [ ] The snake grows by 1 unit when it eats the fruit.
- [ ] The game is over when the snake collides with itself.
- [ ] The game offers a pause option.
- [ ] The game offers a restart, quit, and leaderboard option when paused.