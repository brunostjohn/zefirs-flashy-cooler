const {createCanvas} = require("@napi-rs/canvas");

const width = 480;
const height = 480;
let hue = 0;

function renderFrame() {
    const canvas = createCanvas(width, height);
    const context = canvas.getContext("2d");
    context.fillStyle = "hsl(" + hue + ", 100%, 50%)";
    context.fillRect(0, 0, width, height);
    hue += (10);
    if (hue>360) {hue = hue % 360};
    return canvas.toBuffer("image/jpeg").toString("base64");
}

function renderPreview(){
  return renderFrame();
}

module.exports = {renderFrame, renderPreview, info: {
  title: "Rainbow",
  description: "Cycles a rainbow on the screen.",
  preview: renderPreview(), 
  hasConfig: false
}};