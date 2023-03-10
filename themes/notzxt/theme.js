const Sensors = require("../../libraries/sensors.js");
const { createCanvas, GlobalFonts, Image } = require("@napi-rs/canvas");
const fs = require("fs");
const path = require("path");

const width = 480;
const height = 480;
let sensorValue1 = "45";
let sensorValue2 = "45";
let cpuvalue = "";

let sensors = new Sensors();

GlobalFonts.registerFromPath(
  path.join(__dirname, "gothamssm.ttf"),
  "Gotham-SSM"
);

const emblem_file = fs.readFileSync(path.join(__dirname, "emblem.png"));
const emblem = new Image();
emblem.src = emblem_file;

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

if (!sensors.checkIfSensorExists(config.sensorPath1)) {
  config.sensorPath1 = sensors.provideExistingSensor();
}

if (!sensors.checkIfSensorExists(config.sensorPath2)) {
  config.sensorPath2 = sensors.provideExistingSensor();
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
context.textAlign = "center";

let oldVal1 = 30;
let oldVal2 = 30;
let transitionState1 = false;
let transitionState2 = false;
const framesUntilDone = Math.round((config.refreshFrequency / 2000) * 25);
let currentCounter1 = 0;
let currentCounter2 = 0;
let arcIncrements1 = { part1: 0, part2: 0 };
let arcVals1 = { part1: 0, part2: 0 };
let arcIncrements2 = { part1: 0, part2: 0 };
let arcVals2 = { part1: 0, part2: 0 };

function renderFrame() {
  sensorValue1 = sensors.rateLimitedGetSensorValueByPath(
    config.sensorPath1,
    "current",
    config.refreshFrequency
  );
  let sensorValueCalc1, sensorValueCalc2;
  if (sensorValue1 > 85) {
    sensorValueCalc1 = 85;
  } else if (sensorValue1 < 30) {
    sensorValueCalc1 = 30;
  } else {
    sensorValueCalc1 = sensorValue1;
  }
  sensorValue2 = sensors.rateLimitedGetSensorValueByPath2(
    config.sensorPath2,
    "current",
    config.refreshFrequency
  );
  if (sensorValue2 > 85) {
    sensorValueCalc2 = 85;
  } else if (sensorValue2 < 30) {
    sensorValueCalc2 = 30;
  } else {
    sensorValueCalc2 = sensorValue2;
  }

  if (
    sensorValueCalc1 != oldVal1 &&
    !transitionState1 &&
    config.smoothAnimation
  ) {
    currentCounter1 = framesUntilDone;
    transitionState1 = true;
    arcVals1.part1 = scale(oldVal1, 30, 85, 0.9, 0.6);
    arcVals1.part2 = scale(oldVal1, 30, 85, 1.1, 1.4);

    arcIncrements1.part1 =
      (scale(sensorValueCalc1, 30, 85, 0.9, 0.6) -
        scale(oldVal1, 30, 85, 0.9, 0.6)) /
      framesUntilDone;
    arcIncrements1.part2 =
      (scale(sensorValueCalc1, 30, 85, 1.1, 1.4) -
        scale(oldVal1, 30, 85, 1.1, 1.4)) /
      framesUntilDone;
    oldVal1 = sensorValueCalc1;
  }

  if (
    sensorValueCalc2 != oldVal2 &&
    !transitionState2 &&
    config.smoothAnimation
  ) {
    currentCounter2 = framesUntilDone;
    transitionState2 = true;

    arcVals2.part1 = scale(oldVal2, 30, 85, 1.9, 1.6);
    arcVals2.part2 = scale(oldVal2, 30, 85, 2.1, 2.4);

    arcIncrements2.part1 =
      (scale(sensorValueCalc2, 30, 85, 1.9, 1.6) -
        scale(oldVal2, 30, 85, 1.9, 1.6)) /
      framesUntilDone;
    arcIncrements2.part2 =
      (scale(sensorValueCalc2, 30, 85, 2.1, 2.4) -
        scale(oldVal2, 30, 85, 2.1, 2.4)) /
      framesUntilDone;
    oldVal2 = sensorValueCalc2;
  }

  context.clearRect(0, 0, 480, 480);

  if (transitionState1 && config.smoothAnimation) {
    arcVals1.part1 += arcIncrements1.part1;
    arcVals1.part2 += arcIncrements1.part2;

    context.fillStyle = config.backgroundColour;
    context.fillRect(0, 0, width, height);
    context.beginPath();
    context.arc(
      width / 2,
      height / 2,
      200,
      arcVals1.part1 * Math.PI,
      arcVals1.part2 * Math.PI
    );
    context.strokeStyle = config.colour1;
    context.fillStyle = config.colour1;
    context.font = "40px Gotham-SSM";
    context.fillText(config.sensorText1, 150, 300);
    context.font = "70px Gotham-SSM";
    context.fillStyle = config.textColour;
    context.fillText(Math.round(sensorValue1) + "??", 168, 250);
    context.lineWidth = 40;
    context.lineCap = "round";
    context.stroke();

    currentCounter1--;
    if (currentCounter1 == 0) {
      transitionState1 = false;
      arcIncrements1 = { part1: 0, part2: 0 };
    }
  } else {
    context.fillStyle = config.backgroundColour;
    context.fillRect(0, 0, width, height);
    context.beginPath();
    context.arc(
      width / 2,
      height / 2,
      200,
      scale(sensorValueCalc1, 30, 85, 0.9, 0.6) * Math.PI,
      scale(sensorValueCalc1, 30, 85, 1.1, 1.4) * Math.PI
    );
    context.strokeStyle = config.colour1;
    context.fillStyle = config.colour1;
    context.font = "40px Gotham-SSM";
    context.fillText(config.sensorText1, 150, 300);
    context.font = "70px Gotham-SSM";
    context.fillStyle = config.textColour;
    context.fillText(Math.round(sensorValue1) + "??", 168, 250);
    context.lineWidth = 40;
    context.lineCap = "round";
    context.stroke();
  }

  if (transitionState2 && config.smoothAnimation) {
    arcVals2.part1 += arcIncrements2.part1;
    arcVals2.part2 += arcIncrements2.part2;

    context.beginPath();
    context.arc(
      width / 2,
      height / 2,
      200,
      arcVals2.part1 * Math.PI,
      arcVals2.part2 * Math.PI
    );
    context.strokeStyle = config.colour2;
    context.fillStyle = config.colour2;
    context.font = "40px Gotham-SSM";
    context.fillText(config.sensorText2, 330, 300);
    context.font = "70px Gotham-SSM";
    context.fillStyle = config.textColour;
    context.fillText(Math.round(sensorValue2) + "??", 348, 250);
    context.lineWidth = 40;
    context.lineCap = "round";
    context.stroke();

    currentCounter2--;
    if (currentCounter2 == 0) {
      transitionState2 = false;
      arcIncrements2 = { part1: 0, part2: 0 };
    }
  } else {
    context.beginPath();
    context.arc(
      width / 2,
      height / 2,
      200,
      scale(sensorValueCalc2, 30, 85, 1.9, 1.6) * Math.PI,
      scale(sensorValueCalc2, 30, 85, 2.1, 2.4) * Math.PI
    );
    context.strokeStyle = config.colour2;
    context.fillStyle = config.colour2;
    context.font = "40px Gotham-SSM";
    context.fillText(config.sensorText2, 330, 300);
    context.font = "70px Gotham-SSM";
    context.fillStyle = config.textColour;
    context.fillText(Math.round(sensorValue2) + "??", 348, 250);
    context.lineWidth = 40;
    context.lineCap = "round";
    context.stroke();
  }

  if (config.showEmblem) context.drawImage(emblem, 220, 110);

  return canvas.toBuffer("image/jpeg").toString("base64");
}

function renderPreview() {
  currentCounter1 = 0;
  currentCounter2 = 0;
  sensors = new Sensors();
  reloadConfig();
  return renderFrame();
}

module.exports = {
  renderFrame,
  renderPreview,
  info: {
    title: "NotZXT",
    description:
      "Displays data from system sensors without violating copyrights. Featuring a guest to the show - Carlotta. I think she also loves treats.",
    preview: renderPreview(),
    requiresSensors: true,
    hasConfig: true,
    controllableParameters: {
      sensorPath1: {
        type: "sensor",
        title: "Sensor 1",
      },
      sensorPath2: {
        type: "sensor",
        title: "Sensor 2",
      },
      sensorText1: {
        type: "text",
        title: "Display first sensor as...",
      },
      sensorText2: {
        type: "text",
        title: "Display second sensor as...",
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
        title: "Left Sensor Colour",
        defaultValue: "#1862d9",
      },
      colour2: {
        type: "colour",
        title: "Right Sensor Colour",
        defaultValue: "#d318d9",
      },
      backgroundColour: {
        type: "colour",
        title: "Background Colour",
        defaultValue: "#d318d9",
      },
      textColour: {
        type: "colour",
        title: "Text Colour",
        defaultValue: "#d318d9",
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
