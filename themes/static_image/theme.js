const path = require("path");
const fs = require("fs");
const sizeOf = require("image-size");
const { createCanvas, Image } = require("@napi-rs/canvas");

const toRender = new Image();

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

let dimensions = { width: 0, height: 0 };

try {
  dimensions = sizeOf(config.imagePath);
} catch {
  toRender.src = fs.readFileSync(path.join(__dirname, "image.jpeg"));
}

try {
  toRender.src = fs.readFileSync(config.imagePath);
} catch {
  toRender.src = fs.readFileSync(path.join(__dirname, "image.jpeg"));
}

//TODO: fix this
if (
  !(dimensions.width == 480 && dimensions.height == 480) ||
  dimensions.type != "jpg"
) {
  toRender.src = fs.readFileSync(path.join(__dirname, "fallback.jpeg"));
}

const canvas = createCanvas(480, 480);
const context = canvas.getContext("2d");

function renderFrame() {
  context.clearRect(0, 0, 480, 480);
  context.drawImage(toRender, 0, 0, 480, 480);
  return canvas.toBuffer("image/jpeg");
}

function renderPreview() {
  return renderFrame();
}

module.exports = {
  renderFrame,
  renderPreview,
  info: {
    title: "Static Image",
    description: "Displays a static image.",
    preview: renderPreview(),
    hasConfig: true,
    controllableParameters: {
      imagePath: {
        type: "file",
        title: "Image",
        defaultValue: path.join(__dirname, "image.jpeg"),
      },
    },
  },
};
