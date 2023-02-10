const Sensors = require("../../libraries/sensors.js");
const {createCanvas} = require("@napi-rs/canvas");

const width = 480;
const height = 480;
let cputemp = "0";

let sensors = new Sensors();
console.log(sensors.query("Select Name, Identifier From Hardware"));
// let sensordata;

function renderFrame() {
    // sensordata = sensors.fetchSensors();
    // console.log(sensordata);
    // if (sensordata === undefined || sensordata === {}){
    //     cputemp = "0";
    // } else {
    //     console.log(sensordata);
    //     cputemp = sensordata["12th Gen Intel Core i7-12700K"]["Main Sensors"]["values"][20];
    // }
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
    title: "Sensor Text",
    description: "Displays data from system sensors.",
    preview: renderPreview(),
    requresSensors: true
}};