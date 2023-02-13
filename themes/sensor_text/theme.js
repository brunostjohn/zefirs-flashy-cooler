const Sensors = require("../../libraries/sensors.js");
const {createCanvas} = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

const width = 480;
const height = 480;
let cputemp = "0";
let cpuvalue = "";

let sensors = new Sensors();

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

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

if(!sensors.checkIfSensorExists(config.sensorPath)) {
    config.sensorPath = sensors.provideExistingSensor();
}


function wrapUp() {
    sensors.disableSensors();
}

function reloadConfig() {
    config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));
}

function renderFrame() {
    const canvas = createCanvas(width, height);
    const context = canvas.getContext("2d");
    cputemp = sensors.rateLimitedGetSensorValueByPath(config.sensorPath, "current", config.refreshFrequency);
    context.fillStyle = config.backgroundColour;
    context.fillRect(0,0,width,height);
    const time = config.sensorText + ": " + cputemp;
    context.font = "bold 20pt Menlo";
    context.textAlign = "center";
    context.fillStyle = config.textColour;
    context.fillText(time, 240, 240);
    return canvas.toBuffer("image/jpeg").toString("base64");
}

function renderPreview(){
    sensors = new Sensors();
    reloadConfig();
    return renderFrame();
}

module.exports = {renderFrame, renderPreview, info: {
    title: "Sensor Text",
    description: "Displays data from system sensors.",
    preview: renderPreview(),
    requiresSensors: true,
    hasConfig: true,
    controllableParameters: {
        sensorPath: {
            type: "sensor",
            title: "Sensor"
        },
        sensorText: {
            type: "text",
            title: "Display sensor as..."
        },
        refreshFrequency: {
            type: "range",
            title: "Sensor refresh frequency (in ms)",
            defaultValue: 3000,
            min: 50,
            max: 6000,
            step: 2,
            typeofValue: "integer"
        },
        backgroundColour: {
            type: "colour",
            title: "Background Colour",
            defaultValue: "#1862d9"
        },
        textColour: {
            type: "colour",
            title: "Text Colour",
            defaultValue: "#d318d9"
        }
    }
}};