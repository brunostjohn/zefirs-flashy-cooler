const { createCanvas, Image } = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");
// const extractFrames = require("gif-extract-frames");

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

// dissectGif();

const width = 480;
const height = 480;

const canvas = createCanvas(width, height);
const context = canvas.getContext("2d");

let frames = [];

function renderFrame() {
  context.clearRect(0, 0, 480, 480);

  return canvas.toBuffer("image/jpeg", 100).toString("base64");
}

function renderPreview() {
  reloadConfig();
  return renderFrame();
}

function reloadConfig() {
  // dissectGif();
  config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));
}

// function dissectGif() {
//   let notCompleted = true;

//   let gif = fs.existsSync(config.gifPath)
//     ? config.gifPath
//     : path.join(__dirname, "image.gif");

//   if (fs.existsSync(path.join(__dirname, "extracted"))) {
//     try {
//       // fs.rmdirSync(path.join(__dirname, "extracted"), {
//       //   recursive: true,
//       //   force: true,
//       // });
//     } catch (err) {
//       console.log(err);
//     }
//   }

//   fs.mkdirSync(path.join(__dirname, "extracted"));

//   extractFrames({
//     input: gif,
//     output: path.join(__dirname, "extracted", "%d.png"),
//   }).then(() => {
//     notCompleted = false;
//   });
// }

module.exports = {
  renderFrame,
  renderPreview,
  info: {
    title: "GIF",
    description: "Displays a gif.",
    preview: renderPreview(),
    hasConfig: true,
    controllableParameters: {
      gifPath: {
        type: "file",
        title: "GIF",
        defaultValue: path.join(__dirname, "image.gif"),
      },
    },
  },
};
