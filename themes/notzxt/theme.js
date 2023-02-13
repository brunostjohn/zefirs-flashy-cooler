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
    let sensorValueCalc1, sensorValueCalc2;
    if(sensorValue1 > 85){
        sensorValueCalc1 = 85;
    } else if (sensorValue1<30){
        sensorValueCalc1 = 30;
    } else {
        sensorValueCalc1 = sensorValue1;
    }
    sensorValue2 = sensors.rateLimitedGetSensorValueByPath2(config.sensorPath2, "current", config.refreshFrequency);
    if(sensorValue2 > 85){
        sensorValueCalc2 = 85;
    } else if (sensorValue2<30){
        sensorValueCalc2 = 30;
    } else {
        sensorValueCalc2 = sensorValue2;
    }
    
    context.clearRect(0,0,480,480);

    context.fillStyle = config.backgroundColour;
    context.fillRect(0,0,width,height);
    
    context.beginPath();
    context.arc(width/2, height/2, 200, scale(sensorValueCalc1, 30, 85, 0.9, 0.6) * Math.PI, scale(sensorValueCalc1, 30, 85, 1.1, 1.4) * Math.PI);
    context.strokeStyle = config.colour1;
    context.fillStyle = config.colour1;
    context.font = "40px Gotham-SSM";
    context.fillText("CPU", 150, 300);
    context.font = "70px Gotham-SSM";
    context.fillStyle = config.textColour;
    context.fillText(sensorValue1 + "°", 168, 250);
    context.lineWidth = 40;
    context.lineCap = "round";
    context.stroke();

    context.beginPath();
    context.arc(width/2, height/2, 200, scale(sensorValueCalc2, 30, 85, 1.9, 1.6) * Math.PI, scale(sensorValueCalc2, 30, 85, 2.1, 2.4) * Math.PI);
    context.strokeStyle = config.colour2;
    context.fillStyle = config.colour2;
    context.font = "40px Gotham-SSM";
    context.fillText("GPU", 330, 300);
    context.font = "70px Gotham-SSM";
    context.fillStyle = config.textColour;
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
        },
        backgroundColour: {
            type: "colour",
            title: "Background Colour",
            defaultValue: "#d318d9"
        },
        textColour: {
            type: "colour",
            title: "Text Colour",
            defaultValue: "#d318d9"
        }
    }
}};