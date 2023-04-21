const { createCanvas, Image, GlobalFonts } = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");
const Player = require("winplayer-node");

const width = 480;
const height = 480;

const canvas = createCanvas(width, height);
const context = canvas.getContext("2d");

GlobalFonts.registerFromPath(
  path.join(__dirname, "OpenSans-Regular.ttf"),
  "OpenSans"
);

let trackTitle = "";
let trackArtist = "";
let elapsed = 0;
let trackLength = 0;
let id = "";
let albumArt = new Image();

let player;

let nowPlayingComposite = new Image();

function setFallback() {
  elapsed = 1;
  trackLength = 1;
  trackTitle = "Nothing playing";
  trackArtist = "...";
}

function loadAppIcon(appString) {
  const appIcon = new Image();
  appIcon.src = fs.readFileSync(
    path.join(__dirname, "app_icons", "fallback.png")
  );
  fs.readdirSync(path.join(__dirname, "app_icons")).forEach((icon) => {
    if (appString.toLowerCase().includes(icon.slice(0, -4))) {
      appIcon.src = fs.readFileSync(path.join(__dirname, "app_icons", icon));
    }
  });
  return appIcon;
}

const albumArtDimension = 280;
const albumArtLocation = 240 - albumArtDimension / 2;

function createNowPlayingComposite(update) {
  let prefabCanvas = createCanvas(width, height);
  let prefabContext = prefabCanvas.getContext("2d");

  prefabContext.textAlign = "center";
  prefabContext.textBaseline = "middle";

  let appIcon = loadAppIcon(update.appName);

  id = update.metadata.id;
  albumArt.src =
    update.metadata.artData.data != undefined
      ? update.metadata.artData.data
      : fs.readFileSync(path.join(__dirname, "fallback_art.png"));
  trackTitle = update.metadata.title;
  trackArtist = update.metadata.artist;
  trackLength = update.metadata.length;
  elapsed = update.elapsed.howMuch;

  prefabContext.clearRect(0, 0, 480, 480);
  prefabContext.filter = "blur(10px)";
  prefabContext.drawImage(albumArt, -10, -10, 500, 500);
  prefabContext.filter = "none";
  prefabContext.drawImage(
    albumArt,
    albumArtLocation,
    albumArtLocation - 20,
    albumArtDimension,
    albumArtDimension
  );

  prefabContext.drawImage(appIcon, 240 - appIcon.width / 2, 20);

  prefabContext.font = "40px OpenSans";
  prefabContext.fillText(trackTitle, 240, 390);
  prefabContext.font = "30px OpenSans";
  prefabContext.fillText(trackArtist, 240, 430);
  const result = prefabCanvas.toBuffer("image/jpeg");
  nowPlayingComposite.src = result;
  prefabCanvas = null;
  prefabContext = null;
}

async function onUpdate() {
  const update = await player.getUpdate();

  if (update == null) setFallback();
  if (update != null || update.metadata.id != id) {
    createNowPlayingComposite(update);
  }
}

player = new Player(onUpdate);

let lastTime = 0;
function updateProgress() {
  // if (lastTime > Date.now() - 500) return;
  // lastTime = Date.now();
  elapsed = player.GetPosition().howMuch;
}

function scale(value, inMin, inMax, outMin, outMax) {
  const result =
    ((value - inMin) * (outMax - outMin)) / (inMax - inMin) + outMin;
  return result;
}

context.textAlign = "center";
context.textBaseline = "middle";
context.font = "12px OpenSans";

context.lineCap = "round";
context.lineWidth = 9;

function calculateArcFromProgress() {
  const percent = (elapsed / trackLength) * 100;
  if (percent < 25) {
    return scale(percent, 0, 25, 1.5, 2);
  }
  return scale(percent, 25, 100, 0, 1.5);
}

function renderFrame() {
  context.clearRect(0, 0, 480, 480);

  updateProgress();

  context.drawImage(nowPlayingComposite, 0, 0);

  context.beginPath();
  context.arc(
    width / 2,
    height / 2,
    240,
    1.5 * Math.PI,
    calculateArcFromProgress() * Math.PI
  );
  context.stroke();
  context.closePath();

  return canvas.toBuffer("image/jpeg", 100);
}

function renderPreview() {
  return renderFrame();
}

function wrapUp() {
  console.log(require.cache);
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
