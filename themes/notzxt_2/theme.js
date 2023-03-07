const Sensors = require("../../libraries/sensors.js");
const { createCanvas, GlobalFonts, Path2D, Image } = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

const width = 480;
const height = 480;
let sensorValue = "45";
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

  context.clearRect(0, 0, 480, 480);

  context.lineWidth = 50;
  context.lineCap = "round";
  context.strokeStyle = "white";

  // min - 2.25, max - 0.89;
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
  // min - 0.75, max - 2.11?;
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
  // min - 0.82, max - 2.18;
  context.beginPath();
  const dot = scale(sensorValueCalc, 20, 90, 0.84, 2.17);
  context.arc(480 / 2, 480 / 2, 215, dot * Math.PI, (dot + 0.001) * Math.PI);
  context.stroke();
  context.closePath();

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

  if (config.showEmblem) context.drawImage(emblem, 220, 110, 50, 50);

  return canvas.toBuffer("image/jpeg", 80).toString("base64");
}

function renderPreview() {
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
    },
  },
};
