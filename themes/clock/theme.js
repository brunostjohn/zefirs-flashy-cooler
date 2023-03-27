const { createCanvas } = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

const width = 480;
const height = 480;

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
  context.fillStyle = config.backgroundColour;
  context.fillRect(0, 0, width, height);
  const today = new Date();
  const time =
    addZero(today.getHours()) +
    ":" +
    addZero(today.getMinutes()) +
    ":" +
    addZero(today.getSeconds());
  context.fillStyle = config.textColour;
  context.fillText(time, 240, 240);
  return canvas.toBuffer("image/jpeg", 100);
}

function renderPreview() {
  reloadConfig();
  return renderFrame();
}

function reloadConfig() {
  config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));
}

module.exports = {
  renderFrame,
  renderPreview,
  info: {
    title: "Clock",
    description: "Displays the current time.",
    preview: renderPreview(),
    hasConfig: true,
    controllableParameters: {
      backgroundColour: {
        type: "colour",
        title: "Background Colour",
        defaultValue: "#1862d9",
      },
      textColour: {
        type: "colour",
        title: "Text Colour",
        defaultValue: "#d318d9",
      },
    },
  },
};
