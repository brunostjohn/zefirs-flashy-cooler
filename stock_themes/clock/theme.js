const {createCanvas} = require("@napi-rs/canvas");

const width = 480;
const height = 480;

function addZero(i) {
    if (i < 10) {i = "0" + i}
    return i;
  }

function renderFrame() {
    const canvas = createCanvas(width, height);
    const context = canvas.getContext("2d");
    context.fillStyle = "#00000";
    context.fillRect(0,0,width,height);
    const today = new Date();
    const time = addZero(today.getHours()) + ":" + addZero(today.getMinutes()) + ":" + addZero(today.getSeconds());
    context.font = "bold 40pt Menlo";
    context.textAlign = "center";
    context.fillStyle = "#fff";
    context.fillText(time, 240, 240);
    return canvas.toBuffer("image/jpeg").toString("base64");
}

module.exports = {renderFrame};