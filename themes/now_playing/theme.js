const { createCanvas, Image } = require("@napi-rs/canvas");
const { app } = require("electron");
const fs = require("fs");
const path = require("path");
// const Player = require("winplayer-node");

const width = 480;
const height = 480;

const canvas = createCanvas(width, height);
const context = canvas.getContext("2d");

let trackTitle = "";
let trackArtist = "";
let elapsed = 0;
let appName = "";

let player;

context.textAlign = "center";

function renderFrame() {
  context.clearRect(0, 0, 480, 480);
  // console.log(trackTitle, trackArtist, elapsed, appName);
  return canvas.toBuffer("image/jpeg", 100).toString("base64");
}

function renderPreview() {
  return renderFrame();
}

player = new Player(onUpdate);

async function onUpdate() {
  // console.log("ooooh update");
  const update = await player.getUpdate();
  // trackTitle = update.metadata.title;
  // trackArtist = update.metadata.artist;
  // elapsed = update.elapsed;
  // appName = update.appName;
}

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
