# Introduction
This is a solver for the iMessage Game Pigeon avalanche version of Mancala.
## Game Setup
The game is set up with 14 pockets that hold stones.
There are 6 on each side of the board, and 2 larger ones on each end.
The 6 pockets on each side start out with some number of stones in each of them, with the same amounts of stones mirrored between the 2 sides.
The left side of the board is the player's side, and the right side is the opponent's side.
The large pocket close to the player is their scoring pocket, while the one opposite is the opponent's.
## Playing The Game
The goal of the player is to score stones into their scoring pocket and prevent their opponent from scoring stones.
The players take turns picking stones and moving them around until one of the sides has no more stones in its 6 pockets.
Once one of the sides is empty, the player with more stones in their scoring pocket wins, regardless of which side was emptied.
## Moving Stones
The player can select any of the 6 pockets that have stones on their side of the board.
They will pick up all the stones in the pocket, and drop them one by one in the pockets going counter-clockwise, starting at the first pocket below where they were picked up.
The stones will go around in a cycle through each pocket on the sides, and the players scoring pocket, but not the opponent's scoring pocket.
When the player places the last stone in a pocket, 3 things can happen:
1. If the last stone is placed in an empty pocket, the players's turn is over, and it becomes the opponent's turn (unless the game is over)
2. If the last stone is placed in the player's scoring pocket, they get a free turn and get to go again.
3. If the last stone is placed in any side pocket that has stones in it, all the stones are picked up including the stone just placed. They then repeat the process of placing them one by one going counter-clockwise.
# Using the solver
The solver is written in Rust and uses Cargo as the build tool and package manager.
To install Rust and Cargo, use Rustup, the installer for Rust.
You can download and use Rustup by following the instructions at https://rustup.rs/.
Once you have Rust and Cargo installed, you can run the following command in the root directory of the project:
```sh
cargo run
```
This will start the program and display a menu of options.
The board will start with 4 stones in each side pocket.
To select an entry in the menu, enter the letter wrapped in "()" and press enter.
After each action, the menu and prompt will reappear.
Below are the available functions in the menu.
1. (D)isplay Current Game: Prints out the current state of each pocket, who's turn it is, and the state of the game (Either InProgress, or Over. In the game Over state, it can be either a win or technical win which will be described in more detail below) Example output:
```
> d
          0
      4        4
      4        4
      4        4
      4        4
      4        4
      4        4
          0

Player's turn
Game state: InProgress
```
The pockets displayed are each numbered. The "Player Side" is the left side pockets and the bottom score pocket. The "Opponent Side" is the right side pockets and the top score pocket.
The pockets are numbered starting at 0 on each side, going counter clockwise.
The numbers start for the player side at the top left side pocket, while the numbers for the opponent side pocket start at the bottom right.
Below is the layout of the board with each pocket numbered. The player side numbers are prefixed with "p" and the opponent side numbers with "o"
```
           o6
      p0        o5
      p1        o4
      p2        o3
      p3        o2
      p4        o1
      p5        o0
           p6
```
2. (R)eset Game: Resets the game to the default state (all side pockets filled with 4 stones)
3. (M)anually Enter Board State: Allows the user to manually enter the state of an existing game. The user will first need to enter the number of stones in each of the pockets on their side and their score pocket, each seperated by a space. Then they will be prompted to do the same for the opponent's side. The order of the pockets goes in counter-clockwise order, starting at the position furthest away from the player. Finally, they will be asked which player's turn it is. For example:
```
> m
Enter the player side pockets:
> 5 3 2 0 1 7 12
Enter the opponent side pockets:
> 7 2 4 9 0 2 9
Select current player turn:
1: Player
2: Opponent
> 1

<Main Menu>
> d
          9
      5        2
      3        0
      2        9
      0        4
      1        2
      7        7
          12

Player's turn
Game state: InProgress
```
4. (S)tash Game: Store the current game state in the stash. The stash can only hold one game state at a time so stashing a game while one is already stashed will overwrite the stash.
5. (L)oad Game: Load the game from the stash into the current game.
6. (T)est Move: Displays the result of playing a move on the current state, without actually changing the state of the current game.
7. (P)lay Move: Plays a move and updates the current state of the board with the result of the move. The user should enter a number between 0 and 5 to indicate which pocket should be played. 0 refers to the side pocket furthest from the player, and 5 being the side pocket closest to the player.
8. (F)ind best move: Computes the most optimal sequence of free moves for the player to play to score the most points. The sequence of numbers printed are the numbers of the pockets that should be played in the order printed. It will also display the board that would result from the sequence of moves. For example, with the game state from the prior example:
```
> f
Generating sequence tree...
Finding best move...
2 5 4 2 1 2 4 5 5 5 0 2 4
          9
      2        0
      3        1
      0        2
      2        0
      5        1
      2        1
          35

Opponent's turn
Game state: Over(TechnicalWin(Player))
```
This means that the player should first play pocket 2 (the 3rd one down from the top left), then 5 (the bottom left), then 4, 2, and so on until the last move of playing pocket 4. Each of these moves, except for the last one will always result in a free turn.
# Algorithm
The principle observation made to develop this algorithm is that a single turn can consist of many individual moves by chaining together free turns.
The algorithm finds the sequence of free moves that results in the greatest number of points scored in a single turn.
The program recursively builds a tree of moves that can be made in a single turn.
The algorithm stops recursing in one of two base cases:
1. The move ends the players turn
2. The move ends the game
    - This can also happen in a "Technical Win." This happens when there are no longer enough stones on the board for the other player to be able to win. In the previous example, the game state is a Technical Win for the Player because in order for the opponent to win, they would need to score 27 more points (35 (player points) - 9 (opponent points) + 1 (to win instead of tie)). However, there are only 19 stones left on the board, so the opponent has no way to win.
    - Considering the technical win as a leaf node was a massive performance optimization. At each game state, there are up to 6 legal moves that can be played. This means that at each level in the tree, the number of nodes grows by up to 6x the size of the previous one. This leads to a very rapid growth of the game tree. For example, starting out at the default state, without this optimization, there are 36,411 game states in the search tree, whereas with this optimization, this gets reduced down to only 9,513.

After the move tree is constructed, the alorithm searches through the leaf nodes for the one with the best evaluation. (Evaluation is generalized here to allow for other evaluation functions and algorithms to be tested. In this case though, it simply uses the difference between the 2 scores of the players)
Note that in this algorithm, the evaluation of a sequence is determined only by the evaluation of the final state, so only the leaf nodes need to be evaluated.
Finally the move sequence is reconstructed by following the path from the maximal leaf node up to the root.
