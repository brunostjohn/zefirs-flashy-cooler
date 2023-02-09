const {createCanvas} = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

const width = 480;
const height = 480;

const a = 2 * Math.PI / 6;
const r = 50;

function drawHexagon(x, y, ctx) {
    ctx.beginPath();
    for (var i = 0; i < 6; i++) {
      ctx.lineTo(x + r * Math.cos(a * i), y + r * Math.sin(a * i));
    }
    ctx.closePath();
    ctx.stroke();
  }

function renderFrame() {
    const canvas = createCanvas(width, height);
    const context = canvas.getContext("2d");

    return canvas.toBuffer("image/jpeg", 100).toString("base64");
}

function renderPreview(){
  reloadConfig();
  return renderFrame();
}

function reloadConfig() {
  config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));
}

module.exports = {renderFrame, renderPreview, info: {
  title: "Hexagons!",
  description: "Displays a bunch of hexagons.",
  preview: renderPreview(), 
  hasConfig: true,
  controllableParameters: {
    backgroundColour: {
      type: "colour",
      title: "Hexagon Colour 1",
      defaultValue: "#1862d9"
    },
    textColour: {
      type: "colour",
      title: "Hexagon Colour 1",
      defaultValue: "#d318d9"
    }
  }
}};