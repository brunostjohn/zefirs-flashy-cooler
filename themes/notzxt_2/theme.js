const Sensors = require("../../libraries/sensors.js");
const { createCanvas, GlobalFonts, Path2D, Image } = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

const width = 480;
const height = 480;
let sensorValue = "20";
let sensors = new Sensors();

GlobalFonts.registerFromPath(
  path.join(__dirname, "gothamssm.ttf"),
  "Gotham-SSM"
);

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

const emblem_file = fs.readFileSync(path.join(__dirname, "emblem.png"));
const emblem = new Image();
emblem.src = emblem_file;

if (!sensors.checkIfSensorExists(config.sensorPath)) {
  config.sensorPath = sensors.provideExistingSensor();
}

fs.writeFileSync(path.join(__dirname, "config.json"), JSON.stringify(config));

function wrapUp() {
  sensors.disableSensors();
}

function reloadConfig() {
  config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));
}

function scale(value, inMin, inMax, outMin, outMax) {
  const result =
    ((value - inMin) * (outMax - outMin)) / (inMax - inMin) + outMin;
  return result;
}

const canvas = createCanvas(width, height);
const context = canvas.getContext("2d");
let gradient = context.createLinearGradient(480, 480, 0, 0);
gradient.addColorStop(0, config.colour1);
gradient.addColorStop(1, config.colour2);

let oldval = 20;
let transitionState = false;
const framesUntilDone = Math.round((config.refreshFrequency / 2000) * 25);
let currentCounter = 0;
let arcIncrements = { arc1: 0, arc2: 0, dot: 0 };
let arcVals = { arc1: 0, arc2: 0, dot: 0 };

function renderFrame() {
  sensorValue = sensors.rateLimitedGetSensorValueByPath(
    config.sensorPath,
    "current",
    config.refreshFrequency
  );
  let sensorValueCalc;
  if (sensorValue > 90) {
    sensorValueCalc = 90;
  } else if (sensorValue < 20) {
    sensorValueCalc = 20;
  } else {
    sensorValueCalc = sensorValue;
  }

  if (sensorValueCalc != oldval && !transitionState && config.smoothAnimation) {
    currentCounter = framesUntilDone;
    transitionState = true;
    arcVals.arc1 = scale(oldval, 20, 90, 0.93, 2.25);
    arcVals.arc2 = scale(oldval, 20, 90, 0.75, 2.09);
    arcVals.dot = scale(oldval, 20, 90, 0.84, 2.17);

    arcIncrements.arc1 =
      (scale(sensorValueCalc, 20, 90, 0.93, 2.25) -
        scale(oldval, 20, 90, 0.93, 2.25)) /
      framesUntilDone;
    arcIncrements.arc2 =
      (scale(sensorValueCalc, 20, 90, 0.75, 2.09) -
        scale(oldval, 20, 90, 0.75, 2.09)) /
      framesUntilDone;
    arcIncrements.dot =
      (scale(sensorValueCalc, 20, 90, 0.84, 2.17) -
        scale(oldval, 20, 90, 0.84, 2.17)) /
      framesUntilDone;
    oldval = sensorValueCalc;
  }
  context.clearRect(0, 0, 480, 480);

  context.lineWidth = 50;
  context.lineCap = "round";
  context.strokeStyle = "white";

  if (transitionState && config.smoothAnimation) {
    arcVals.arc1 += arcIncrements.arc1;
    arcVals.arc2 += arcIncrements.arc2;
    arcVals.dot += arcIncrements.dot;
    // min - 0.93, max - 2.25;
    context.beginPath();
    context.arc(480 / 2, 480 / 2, 215, arcVals.arc1 * Math.PI, 2.3 * Math.PI);
    context.stroke();
    context.closePath();
    // min - 0.75, max - 2.09
    context.beginPath();
    context.arc(480 / 2, 480 / 2, 215, 0.7 * Math.PI, arcVals.arc2 * Math.PI);
    context.stroke();
    context.closePath();
    // min - 0.84, max - 2.17
    context.beginPath();
    context.arc(
      480 / 2,
      480 / 2,
      215,
      arcVals.dot * Math.PI,
      (arcVals.dot + 0.001) * Math.PI
    );
    context.stroke();
    context.closePath();
    currentCounter--;
    if (currentCounter == 0) {
      transitionState = false;
      arcIncrements = { arc1: 0, arc2: 0, dot: 0 };
    }
  } else {
    // min - 0.93, max - 2.25;
    context.beginPath();
    context.arc(
      480 / 2,
      480 / 2,
      215,
      scale(sensorValueCalc, 20, 90, 0.93, 2.25) * Math.PI,
      2.3 * Math.PI
    );
    context.stroke();
    context.closePath();
    // min - 0.75, max - 2.09
    context.beginPath();
    context.arc(
      480 / 2,
      480 / 2,
      215,
      0.7 * Math.PI,
      scale(sensorValueCalc, 20, 90, 0.75, 2.09) * Math.PI
    );
    context.stroke();
    context.closePath();
    // min - 0.84, max - 2.17
    context.beginPath();
    const dot = scale(sensorValueCalc, 20, 90, 0.84, 2.17);
    context.arc(480 / 2, 480 / 2, 215, dot * Math.PI, (dot + 0.001) * Math.PI);
    context.stroke();
    context.closePath();
  }

  context.globalCompositeOperation = "source-in";

  context.fillStyle = gradient;
  context.fillRect(0, 0, canvas.width, canvas.height);

  context.globalCompositeOperation = "source-over";

  context.textAlign = "center";
  context.textBaseline = "middle";
  context.font = "170px Gotham-SSM";
  context.fillStyle = config.textColour;
  context.fillText(Math.round(sensorValue).toString(), 240, 240);
  const textWidth = Math.round(
    context.measureText(Math.round(sensorValue).toString()).width / 2
  );

  context.font = "30px Gotham-SSM";
  context.fillStyle = config.textColour;
  context.fillText(config.sensorText, 240, 330);
  context.font = "60px Gotham-SSM";
  context.fillText("°", 245 + textWidth, 200);

  if (config.showEmblem) context.drawImage(emblem, 220, 110);

  return canvas.toBuffer("image/jpeg", 80);
}

function renderPreview() {
  currentCounter = 0;
  sensors = new Sensors();
  reloadConfig();
  gradient = context.createLinearGradient(480, 480, 0, 0);
  gradient.addColorStop(0, config.colour1);
  gradient.addColorStop(1, config.colour2);
  return renderFrame();
}

module.exports = {
  renderFrame,
  renderPreview,
  info: {
    title: "NotZXT 2",
    description:
      "Displays data from system sensors without violating copyrights (again).",
    preview: renderPreview(),
    requiresSensors: true,
    hasConfig: true,
    controllableParameters: {
      sensorPath: {
        type: "sensor",
        title: "Sensor",
      },
      sensorText: {
        type: "text",
        title: "Display sensor as...",
      },
      refreshFrequency: {
        type: "range",
        title: "Sensor refresh frequency (in ms)",
        defaultValue: 3000,
        min: 50,
        max: 6000,
        step: 2,
        typeofValue: "integer",
      },
      colour1: {
        type: "colour",
        title: "Gradient Colour 1",
        defaultValue: "#1862d9",
      },
      colour2: {
        type: "colour",
        title: "Gradient Colour 2",
        defaultValue: "#d318d9",
      },
      textColour: {
        type: "colour",
        title: "Text Colour",
        defaultValue: "#ffffff",
      },
      showEmblem: {
        type: "bool",
        title: "Show emblem",
        defaultValue: true,
      },
      smoothAnimation: {
        type: "bool",
        title:
          "Smooth transitions in value indicator (warning: increases CPU usage)",
        defaultValue: true,
      },
    },
  },
};
