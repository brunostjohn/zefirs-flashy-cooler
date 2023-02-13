const Sensors = require("../../libraries/sensors.js");
const {createCanvas, GlobalFonts} = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

const width = 480;
const height = 480;
let sensorValue1 = "45";
let sensorValue2 = "45";
let cpuvalue = "";

let sensors = new Sensors();

GlobalFonts.registerFromPath(path.join(__dirname, "gothamssm.ttf"), "Gotham-SSM");

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

function wrapUp() {
    sensors.disableSensors();
}

function reloadConfig() {
    config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));
}

function scale(value, inMin, inMax, outMin, outMax) {
    const result = (value - inMin) * (outMax - outMin) / (inMax - inMin) + outMin;
    return result;
}

const canvas = createCanvas(width, height);
const context = canvas.getContext("2d");
context.textAlign = "center";

function renderFrame() {
    sensorValue1 = sensors.rateLimitedGetSensorValueByPath(config.sensorPath1, "current", config.refreshFrequency);
    if(sensorValue1 > 85){
        sensorValue1 = 85;
    } else if (sensorValue1<30){
        sensorValue1 = 30;
    }
    sensorValue2 = sensors.rateLimitedGetSensorValueByPath2(config.sensorPath2, "current", config.refreshFrequency);
    if(sensorValue2 > 85){
        sensorValue2 = 85;
    } else if (sensorValue2<30){
        sensorValue2 = 30;
    }
    
    context.clearRect(0,0,480,480);
    
    context.beginPath();
    context.arc(width/2, height/2, 200, scale(sensorValue1, 30, 85, 0.9, 0.6) * Math.PI, scale(sensorValue1, 30, 85, 1.1, 1.4) * Math.PI);
    context.strokeStyle = config.colour1;
    context.fillStyle = config.colour1;
    context.font = "40px Gotham-SSM";
    context.fillText("CPU", 150, 300);
    context.font = "70px Gotham-SSM";
    context.fillStyle = "#ffffff";
    context.fillText(sensorValue1 + "°", 168, 250);
    context.lineWidth = 40;
    context.lineCap = "round";
    context.stroke();

    context.beginPath();
    context.arc(width/2, height/2, 200, scale(sensorValue2, 30, 85, 1.9, 1.6) * Math.PI, scale(sensorValue2, 30, 85, 2.1, 2.4) * Math.PI);
    context.strokeStyle = config.colour2;
    context.fillStyle = config.colour2;
    context.font = "40px Gotham-SSM";
    context.fillText("GPU", 330, 300);
    context.font = "70px Gotham-SSM";
    context.fillStyle = "#ffffff";
    context.fillText(sensorValue2 + "°", 348, 250);
    context.lineWidth = 40;
    context.lineCap = "round";
    context.stroke();

    return canvas.toBuffer("image/jpeg").toString("base64");
}

function renderPreview(){
    sensors = new Sensors();
    reloadConfig();
    return renderFrame();
}

module.exports = {renderFrame, renderPreview, info: {
    title: "NotZXT",
    description: "Displays data from system sensors without violating copyrights.",
    preview: renderPreview(),
    requiresSensors: true,
    hasConfig: true,
    controllableParameters: {
        sensorPath1: {
            type: "sensor",
            title: "Sensor 1"
        },
        sensorPath2: {
            type: "sensor",
            title: "Sensor 2"
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
        colour1: {
            type: "colour",
            title: "Left Sensor Colour",
            defaultValue: "#1862d9"
        },
        colour2: {
            type: "colour",
            title: "Right Sensor Colour",
            defaultValue: "#d318d9"
        }
    }
}};