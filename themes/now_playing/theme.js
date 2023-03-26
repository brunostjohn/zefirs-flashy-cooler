const { createCanvas, Image } = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

const width = 480;
const height = 480;

const canvas = createCanvas(width, height);
const context = canvas.getContext("2d");

let trackTitle = "";
let trackArtist = "";
let elapsed = 0;
let appName = "";

context.textAlign = "center";

function renderFrame() {
  context.clearRect(0, 0, 480, 480);
  console.log(trackTitle, trackArtist, elapsed, appName);
  return canvas.toBuffer("image/jpeg", 100);
}

function renderPreview() {
  return renderFrame();
}

function wrapUp() {}

module.exports = {
  renderFrame,
  renderPreview,
  info: {
    title: "Now Playing",
    description: "Shows the currently playing music.",
    preview: renderPreview(),
    hasConfig: false,
  },
};
