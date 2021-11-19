# Video Poker

## About

Video poker trainer that focuses on the Jacks or Better 9/6 game.
Keeps track of score which is based on strategy.
Credits are also kept track of based on the pay table.

## Goals

* Basic Gameplay
* Computing best cards to hold in less than 7 seconds
  * Results are displayed on screen and in the log
* Keeping size under 32kb
* no_std and embedded style

## Gameplay

* Keyboard
  * Hold buttons (1,2,3,4,5)
  * Deal button (SPACE)

* Strategy
  * Displayed a few seconds after the cards are first dealt

## 🚴 Usage

### 🛠️ Build with `wasm-pack build --target web`

```
wasm-pack build
```

### 🔬 Test in node with `wasm-pack test --node`

```
wasm-pack test --node
```
