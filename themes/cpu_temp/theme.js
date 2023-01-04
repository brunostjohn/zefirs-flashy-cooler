const Sensors = require("../../libraries/sensors.js");
const {createCanvas} = require("@napi-rs/canvas");

const width = 480;
const height = 480;
let cputemp = 0;

const sensors = new Sensors();
console.log(sensors.getGPUTemps());

function renderFrame() {
    const canvas = createCanvas(width, height);
    const context = canvas.getContext("2d");
    context.fillStyle = "#00000";
    context.fillRect(0,0,width,height);
    const time = "CPU Temperature: " + cputemp;
    context.font = "bold 20pt Menlo";
    context.textAlign = "center";
    context.fillStyle = "#fff";
    context.fillText(time, 240, 240);
    return canvas.toBuffer("image/jpeg").toString("base64");
}

function renderPreview(){
    return renderFrame();
}

module.exports = {renderFrame, info: {
    title: "CPU Temperature",
    description: "Displays the CPU temperature.",
    preview: renderPreview()
}};