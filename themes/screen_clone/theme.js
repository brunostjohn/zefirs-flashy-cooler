const { createCanvas, Image } = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");
const screenshot = require("screenshot-desktop");

const width = 480;
const height = 480;

const shot = new Image();

const canvas = createCanvas(width, height);
const context = canvas.getContext("2d");

function renderFrame() {
  context.clearRect(0, 0, 480, 480);
  screenshot()
    .then((img) => {
      shot.src = img;
    })
    .catch((err) => {
      console.log(err);
    });
  context.drawImage(shot, 0, 0);
  return canvas.toBuffer("image/jpeg", 100).toString("base64");
}

function renderPreview() {
  return renderFrame();
}

module.exports = {
  renderFrame,
  renderPreview,
  info: {
    title: "Screen Clone",
    description: "Clones the screen's content.",
    preview: renderPreview(),
    hasConfig: false,
  },
};
