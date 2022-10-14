# Geo-Fight
Wasm/Rust implementation of Geo Fight, a strategy game powered by NEAR Protocol.

## Main Purpose of the Repository
The main purpose is to create a template application that uses the Rust programming language for both frontend and smart-contracts. 
The frontend is written using Yew, a modern Rust framework for creating frontend web-apps using WebAssembly (wasm).

### Why Wasm?
JavaScript runs everywhere because browser runs everywhere. But it is not as efficient as c and c++ in doing heavy computations. On the other hand, people do not want to download and run executables in c because of security issues.

Wasm is the solution to this: fast and efficient programs can be compiled to a low level language with wasm to run in the browser. These browser programs are indeed faster than JavaScript. 

Therefore, Wasm is a perfect fit for browser games.


### Why rust? 
Rust is the number one language when it comes to web assembly programming. Rust is capable of doing systems level programming, is blazingly fast, it can reach the performance of C and C++ and it enhances the memory safeness with respect those two options. 

Performance, security and browser usability, all in one programming language.

Wasm + rust = ❤️ 

Rust is also the programming language used by the near blockchain.

### Why NEAR?
NEAR protocol provides a simple, secure, fast and developer firendly platform. Ideal for game development.

## Gameplay
Geofight consists of a tile-world, where each tile has its own representation in the NEAR blockchain. Users can spawn an agent in this world to play in this strategy game. Agents can move to nearby tiles, attack other players or build/improve their fortress.

## High Level Game Design
* Pages
  * **Game**: Main game page.
  * **Ranking**: Ranking of players sorted by power.
  * **How to play**: Gameplay guidelines.
* Components
  * **cell.rs**: The minimum unit, a unique tile.
  * **map.rs**: This component gathers all the playable tiles. 
  * **info.rs**: Game state display.
  * **navbar.rs**: Navigation bar.
  * **utils.rs**: Various utility functions. 
* Agents
  * **agent.rs**: Agent class and related functions. 



