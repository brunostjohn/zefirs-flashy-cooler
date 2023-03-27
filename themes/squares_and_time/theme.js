const { createCanvas } = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

const width = 480;
const height = 480;

let cellsPerLine = config.amountOfSquares;

let cellSize = width / cellsPerLine;

let colours = [];

function getRandomInt(min, max) {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min) + min);
}

for (let i = 1; i <= cellsPerLine; i++) {
  for (let j = 1; j <= cellsPerLine; j++) {
    colours.push(getRandomInt(0, 360));
  }
}

function addZero(i) {
  if (i < 10) {
    i = "0" + i;
  }
  return i;
}

const canvas = createCanvas(width, height);
const context = canvas.getContext("2d");

context.font = "70px Arial";
context.textAlign = "center";
context.textBaseline = "middle";

function renderFrame() {
  context.clearRect(0, 0, 480, 480);
  context.filter = "blur(10px)";
  for (let i = 1; i <= cellsPerLine; i++) {
    for (let j = 1; j <= cellsPerLine; j++) {
      context.fillStyle =
        "hsl(" + colours[i * cellsPerLine + j] + ", 100%, 50%)";
      colours[i * cellsPerLine + j] += 10;
      if (colours[i * cellsPerLine + j] > 360)
        colours[i * cellsPerLine + j] = colours[i * cellsPerLine + j] % 360;
      context.fillRect(
        cellSize * (i - 1),
        cellSize * (j - 1),
        cellSize,
        cellSize
      );
    }
  }
  context.filter = "none";
  const today = new Date();
  const time =
    addZero(today.getHours()) +
    ":" +
    addZero(today.getMinutes()) +
    ":" +
    addZero(today.getSeconds());
  context.fillStyle = "white";
  context.fillText(time, 240, 240);
  return canvas.toBuffer("image/jpeg", 70);
}

function renderPreview() {
  reloadConfig();
  cellsPerLine = config.amountOfSquares;
  cellSize = width / cellsPerLine;
  colours = [];
  for (let i = 1; i <= cellsPerLine; i++) {
    for (let j = 1; j <= cellsPerLine; j++) {
      colours.push(getRandomInt(0, 360));
    }
  }
  return renderFrame();
}

function reloadConfig() {
  config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));
}

module.exports = {
  renderFrame,
  renderPreview,
  info: {
    title: "Squares & Time",
    description: "Fancy Timeface",
    preview: renderPreview(),
    hasConfig: true,
    controllableParameters: {
      amountOfSquares: {
        type: "range",
        title: "Amount of Squares",
        defaultValue: 8,
        min: 2,
        max: 12,
        step: 2,
        typeofValue: "integer",
      },
    },
  },
};
