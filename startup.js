const ActiveX = require("winax");
const fs = require("fs");
const path = require("path");
const { parentPart, workerData, parentPort } = require("worker_threads");
const colors = require("colors");
const { exec, execSync } = require("child_process");
const HID = require("node-hid");

let iCueRunning = true;
let libreRunning = true;

let config = workerData.configuration;

let errorContent;
let problemTheme;
let exceptionText;

function makeId(length) {
  var result = "";
  var characters =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  var charactersLength = characters.length;
  for (var i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

if (config.defaultThemePath == "") {
  config.defaultThemePath = path.join(
    __dirname,
    "themes",
    "static_image",
    "theme.js"
  );
}

if (!fs.existsSync(config.defaultThemePath)) {
  config.defaultThemePath = path.join(
    __dirname,
    "themes",
    "static_image",
    "theme.js"
  );
}

const themeFolder = path.join(__dirname, "themes");

try {
  execSync('tasklist | findstr "iCUE.exe"');
} catch {
  iCueRunning = false;
}

parentPort.postMessage({
  type: "console",
  content: iCueRunning ? "iCUE.exe found!" : "iCUE.exe not found.",
});

try {
  execSync('tasklist | findstr "LibreHardwareMonitor.exe"');
} catch (err) {
  libreRunning = false;
}
parentPort.postMessage({
  type: "console",
  content: libreRunning
    ? "LibreHardwareMonitor.exe found."
    : "LibreHardwareMonitor.exe not found!",
});

const createHwTrees = () => {
  try {
    if (libreRunning) {
      let hwTrees = [];
      parentPort.postMessage({
        type: "console",
        content: "Generating hardware tree...",
      });
      const conn = new ActiveX.Object("WbemScripting.SWbemLocator");
      const svr = conn.ConnectServer(".", "root\\LibreHardwareMonitor");
      let queryString = "Select Name, Identifier From Hardware";
      const results = [];
      let queryResponse = svr.ExecQuery(queryString);
      for (let i = 0; i < queryResponse.Count; i += 1) {
        const properties = queryResponse.ItemIndex(i).Properties_;
        let count = properties.Count;
        const propEnum = properties._NewEnum;
        const obj = {};
        while (count) {
          count -= 1;
          const prop = propEnum.Next(1);
          obj[prop.Name] = prop.Value;
        }
        results.push(obj);
      }
      const finalArray = [];
      results.forEach((result) => {
        const hardwarePath = result.Identifier;
        const hardwareName = result.Name;
        const finalObject = {
          name: hardwareName,
          path: hardwarePath,
          sensorTree: [],
        };
        queryString =
          'Select SensorType From Sensor Where Parent = "' + hardwarePath + '"';
        const results2 = [];
        queryResponse = svr.ExecQuery(queryString);
        for (let i = 0; i < queryResponse.Count; i += 1) {
          const properties = queryResponse.ItemIndex(i).Properties_;
          let count = properties.Count;
          const propEnum = properties._NewEnum;
          const obj = {};
          while (count) {
            count -= 1;
            const prop = propEnum.Next(1);
            obj[prop.Name] = prop.Value;
          }
          results2.push(obj);
        }
        const sensorCats = [];
        results2.forEach((result) => {
          if (!sensorCats.includes(result.SensorType)) {
            sensorCats.push(result.SensorType);
          }
        });
        sensorCats.forEach((sensorCat) => {
          const treeByCat = { category: sensorCat, sensors: [] };
          queryString =
            'Select Identifier, Name From Sensor Where Parent = "' +
            hardwarePath +
            '" And SensorType = "' +
            sensorCat +
            '"';
          const results3 = [];
          queryResponse = svr.ExecQuery(queryString);
          for (let i = 0; i < queryResponse.Count; i += 1) {
            const properties = queryResponse.ItemIndex(i).Properties_;
            let count = properties.Count;
            const propEnum = properties._NewEnum;
            const obj = {};
            while (count) {
              count -= 1;
              const prop = propEnum.Next(1);
              obj[prop.Name] = prop.Value;
            }
            results3.push(obj);
          }
          results3.forEach((result) => {
            const sensorObject = { name: result.Name, path: result.Identifier };
            treeByCat.sensors.push(sensorObject);
          });
          finalObject.sensorTree.push(treeByCat);
        });
        hwTrees.push(finalObject);
      });
      ActiveX.release(conn);
      parentPort.postMessage({
        type: "console",
        content: "Done!",
      });
      return hwTrees;
    }
  } catch (err) {
    errorless = false;
    errorContent =
      "There was an error while reading system sensors. Please check your configuration.";
    exceptionText = err.message;
  }
};

const loadThemes = () => {
  try {
    let themeLst = [];
    parentPort.postMessage({
      type: "console",
      content: "Reading themes...",
    });

    fs.readdirSync(themeFolder).forEach((file) => {
      const theme = require(path.join(__dirname, "themes", file, "theme.js"));
      let activeFlag = false;
      let requiresSensors = false;
      const themepath = path.join(__dirname, "themes", file, "theme.js");
      problemTheme = themepath;
      const configpath = path.join(__dirname, "themes", file, "config.json");
      if (config.defaultThemePath == themepath) {
        activeFlag = true;
      }
      if (theme.info.requiresSensors != undefined) {
        if (theme.info.requiresSensors) {
          requiresSensors = true;
          activeThemeNeedsSensorsFlag =
            requiresSensors && activeFlag ? true : false;
        }
      }
      if ((requiresSensors && libreRunning) || !requiresSensors) {
        let entry = {
          path: themepath,
          id: makeId(32),
          title: theme.info.title,
          description: theme.info.description,
          preview: "data:image/jpeg;base64," + theme.info.preview,
          isActive: activeFlag,
          hasConfig: theme.info.hasConfig,
          configPath: configpath,
          controllableParameters: theme.info.controllableParameters,
          requiresSensors: requiresSensors,
        };
        if (theme.info.hasConfig) {
          const configTheme = JSON.parse(fs.readFileSync(entry.configPath));
          Object.keys(configTheme).forEach((key) => {
            entry.controllableParameters[key]["value"] = configTheme[key];
            entry.controllableParameters[key]["varName"] = key;
            entry.controllableParameters[key]["id"] = makeId(32);
          });
        }
        themeLst.push(entry);
      }
    });

    themeLst.sort((a, b) => {
      const item1 = a.title;
      const item2 = b.title;

      return item1.localeCompare(item2, undefined, { numeric: true });
    });

    if (activeThemeNeedsSensorsFlag && !libreRunning) {
      themeLst[0].isActive = true;
      config.defaultThemePath = themeLst[0].path;
    }

    parentPort.postMessage({
      type: "console",
      content: "Done!",
    });
    return themeLst;
  } catch (err) {
    errorless = false;
    errorContent = "Failed to load themes! Problematic theme: " + problemTheme;
    exceptionText = err.message;
  }
};

function findDevice() {
  parentPort.postMessage({
    type: "console",
    content: "Enumerating HID devices...",
  });
  const devices = HID.devices();
  parentPort.postMessage({
    type: "console",
    content: "Done.",
  });
  parentPort.postMessage({
    type: "console",
    content: "Searching for supported device...",
  });
  let persistent;
  fs.readdirSync(path.join(__dirname, "devices")).forEach((file) => {
    let manifest = require(path.join(
      __dirname,
      "devices",
      file,
      "resources",
      "device.manifest.js"
    )).manifest;
    manifest["devicePlugin"] = path.join(
      __dirname,
      "devices",
      file,
      "deviceplugin.node"
    );
    manifest["deviceImage"] =
      "data:image/png;base64," +
      fs
        .readFileSync(
          path.join(__dirname, "devices", file, "resources", "device.image.png")
        )
        .toString("base64");
    const found = devices.find(function (device) {
      return (
        device.vendorId == manifest.vendorId &&
        device.productId == manifest.productId
      );
    });
    if (found != undefined) {
      persistent = manifest;
    }
  });
  return persistent;
}

let errorless;

const hardwareList = createHwTrees();
const themeList = loadThemes();
const availableDevice = findDevice();

if (availableDevice != undefined) {
  parentPort.postMessage({
    type: "console",
    content: "Found " + availableDevice.deviceName,
  });
} else {
  errorContent =
    "No devices supported by this app have been found. Please check whether your device is supported.";
  exceptionText =
    "No devices supported found by VID/PID. If you're a plugin dev, make sure your values are correct and that the device is connected.";
}

if (!errorless) {
  parentPort.postMessage({
    type: "error",
    content: errorContent,
    exText: exceptionText,
  });
} else {
  parentPort.postMessage({
    type: "done",
    hardwareList: hardwareList,
    themeList: themeList,
    libreRunning: libreRunning,
    iCueRunning: iCueRunning,
    availableDevice: availableDevice,
  });
}

parentPort.close();
