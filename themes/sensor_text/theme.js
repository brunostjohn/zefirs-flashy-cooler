const Sensors = require("../../libraries/sensors.js");
const {createCanvas} = require("@napi-rs/canvas");

const width = 480;
const height = 480;
let cputemp = "0";
let cpuvalue = "";

let sensors = new Sensors();
// const emitter = sensors.startMonitoring("/intelcpu/0/temperature/12", "current");
// cputemp = 
// console.log(sensors.query("Select Name, Identifier From Hardware"));
// console.log(sensors.listHardware());
// console.log(sensors.listSensorTypes("/gpu-amd/0"));
// console.log(sensors.listSensorsByType("/intelcpu/0", "Temperature"));
// console.log(sensors.getSensorValueByPath("/intelcpu/0/temperature/12", "current"));
// console.log(sensors.testFunction());
// let sensordata;

// const valueEmitter = new EventEmitter();
// setInterval(function() {valueEmitter.emit("value", sensors.getSensorValueByPath("/intelcpu/0/temperature/12", "current"));}, 500);

// valueEmitter.on("value", (value) => {
//     cputemp = value;
// });

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
    title: "Sensor Text",
    description: "Displays data from system sensors.",
    preview: renderPreview(),
    requiresSensors: true
}};