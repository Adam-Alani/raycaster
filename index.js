import init, { GameEngine } from "./pkg/rustcast.js";

const canvas = document.getElementById("canvas");

const controller = {
  87: { pressed: false, func: "w" },
  83: { pressed: false, func: "s" },
  65: { pressed: false, func: "a" },
  68: { pressed: false, func: "d" },
};

init().then(() => {
  const game = GameEngine.new();
  canvas.width = game.width();
  canvas.height = game.height();
  canvas.focus();
  window.addEventListener("keydown", (event) => onKeyDown(game, event));
  window.addEventListener("keyup", (event) => onKeyUp(game, event));
  requestAnimationFrame(() => onFrame(game));
});

function onFrame(game) {
  game.render(canvas.getContext("2d"));
  for (const key in controller) {
    if (controller[key].pressed) {
      game.handle_key(controller[key].func);
    }
  }
  requestAnimationFrame(() => onFrame(game));
}

// Handle w,a,s,d keys
function onKeyDown(game, event) {
  const key = event.key;
  if (key === "w" || key === "a" || key === "s" || key === "d") {
    event.preventDefault();
    controller[event.keyCode].pressed = true;
  }
}

function onKeyUp(game, event) {
  const key = event.key;
  if (key === "w" || key === "a" || key === "s" || key === "d") {
    event.preventDefault();
    controller[event.keyCode].pressed = false;
  }
}
