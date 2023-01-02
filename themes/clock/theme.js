const {createCanvas} = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

const config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

const width = 480;
const height = 480;

const controllableParameters = {
  textColour: {
    
  },
  backgroundColour: {
    
  }
};

function addZero(i) {
    if (i < 10) {i = "0" + i}
    return i;
  }

function renderFrame() {
    const canvas = createCanvas(width, height);
    const context = canvas.getContext("2d");
    context.fillStyle = config.backgroundColour;
    context.fillRect(0,0,width,height);
    const today = new Date();
    const time = addZero(today.getHours()) + ":" + addZero(today.getMinutes()) + ":" + addZero(today.getSeconds());
    context.font = "bold 40pt Menlo";
    context.textAlign = "center";
    context.fillStyle = config.textColour;
    context.fillText(time, 240, 240);
    return canvas.toBuffer("image/jpeg").toString("base64");
}

function renderPreview(){
  return renderFrame();
}

module.exports = {renderFrame, info: {
  title: "Clock",
  description: "Displays the current time.",
  preview: renderPreview(), 
  hasConfig: true
}};