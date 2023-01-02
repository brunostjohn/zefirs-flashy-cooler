const si = require("systeminformation");
const {createCanvas} = require("@napi-rs/canvas");

const width = 480;
const height = 480;

function renderFrame() {
    const canvas = createCanvas(width, height);
    const context = canvas.getContext("2d");
    context.fillStyle = "#00000";
    context.fillRect(0,0,width,height);
    const cputemp = "3333";
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