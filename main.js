const appVersion = "0.0.1";
const releaseType = "alpha";

const {app, Tray, Menu, nativeImage, BrowserWindow, nativeTheme, systemPreferences, ipcMain, dialog} = require("electron");
const path = require("path");
const { Worker } = require("worker_threads");
const fs = require("fs");
const { Z_FIXED } = require("zlib");
const Sensors = require("./libraries/sensors.js");
const log = require("why-is-node-running");
let config = JSON.parse(fs.readFileSync(path.join(__dirname, "app.config.json")));
const {exec, execSync} = require("child_process");

let iCueRunning = true;
let libreRunning = true;

if (require("electron-squirrel-startup")) app.quit();
try{execSync("tasklist | findstr \"iCUE.exe\"")} catch {iCueRunning = false}
try{execSync("tasklist | findstr \"LibreHardwareMonitor.exe\"")} catch {libreRunning = false};


nativeTheme.themeSource = "dark";

let fps = config.fps;
let mainWindow;
let themeList = [];
let rendering = config.renderAtStartup;

if (config.defaultThemePath == "") {
    config.defaultThemePath = path.join(__dirname, "themes", "static_image", "theme.js");
}

if (!fs.existsSync(config.defaultThemePath)) {
    config.defaultThemePath = path.join(__dirname, "themes", "static_image", "theme.js");
}

const themeFolder = path.join(__dirname, "themes");

function makeId(length) {
    var result = "";
    var characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    var charactersLength = characters.length;
    for ( var i = 0; i < length; i++ ) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
}

fs.readdirSync(themeFolder).forEach(file => {
    const theme = require(path.join(__dirname, "themes", file, "theme.js"));
    let activeFlag = false;
    let requiresSensors = false;
    const themepath = path.join(__dirname, "themes", file, "theme.js");
    const configpath = path.join(__dirname, "themes", file, "config.json");
    if (config.defaultThemePath == themepath) {
        activeFlag = true;
    }
    if (theme.info.requiresSensors != undefined){
        if(theme.info.requiresSensors) {
            requiresSensors = true;
        }
    }
    if (((requiresSensors && libreRunning) || !requiresSensors)){
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
            requiresSensors: requiresSensors
        };
        if (theme.info.hasConfig) {
            const configTheme = JSON.parse(fs.readFileSync(entry.configPath));
            Object.keys(configTheme).forEach(key => {
                entry.controllableParameters[key]["value"] = configTheme[key];
                entry.controllableParameters[key]["varName"] = key;
                entry.controllableParameters[key]["id"] = makeId(32);
            });
        }
        themeList.push(entry);
    }
});

themeList.sort((a,b) => {
    const item1 = a.title;
    const item2 = b.title;

    return item1.localeCompare(item2, undefined, {numeric: true})
});

const createWindow = () => {
    mainWindow = new BrowserWindow({
        width: 1130,
        height: 800,
        webPreferences: {
            preload: path.join(__dirname, "libraries/preload.js")
        },
        icon: path.join(__dirname, "assets", "images", "favicon.ico")
    })
    ipcMain.handle("renderer:startRendering", startRendering);
    ipcMain.handle("renderer:stopRendering", stopRendering);
    ipcMain.handle("themes:getThemeList", getThemeList);
    ipcMain.on("themes:themeSelected", selectTheme);
    ipcMain.on("renderer:parameterTransfer", applyParameters);
    ipcMain.handle("renderer:renderStatus", renderStatus);
    ipcMain.handle("global:openFile", handleFileOpen);
    ipcMain.handle("settings:requestConfig", requestConfig);
    ipcMain.handle("settings:requestVersion", requestVersion);
    ipcMain.handle("settings:requestHealth", requestHealth);
    ipcMain.on("settings:configSendback", configSendback);
    ipcMain.handle("settings:requestThemeFolder", requestThemeFolder);
    ipcMain.handle("settings:openThemeFolder", openThemeFolder);
    mainWindow.loadFile("assets/ui/themes.html");
    mainWindow.removeMenu();
}

let worker = new Worker("./libraries/renderer.js", { workerData: {renderPath: config.defaultThemePath, fps: fps} });

worker.on("error", err => {
    console.log(err);
});

worker.on('message', (msg) => { 
    // mainWindow.webContents.send("fps", msg);
    console.log(msg);
});

worker.on("unhandledRejection", error => {throw error});

app.whenReady().then(() => {
    createWindow();
    createTray();
    if(rendering) {
        startRendering();
    }
})

let tray;

const contextMenuInactive = Menu.buildFromTemplate([
    { label: "Open Zefir's Flashy Cooler", click() { createWindow(); }},
    { label: "Start Rendering", click() {startRendering()} },
    { label: "Quit Zefir's Flashy Cooler", click() { exit(); } }
])

const contextMenuActive = Menu.buildFromTemplate([
    { label: "Open Zefir's Flashy Cooler", click() { createWindow(); }},
    { label: "Stop Rendering", click() {stopRendering();} },
    { label: "Quit Zefir's Flashy Cooler", click() { exit(); } }
])

const createTray = () => {
    tray = new Tray(path.join(__dirname, "assets", "images", "favicon.ico"));
    tray.setToolTip("Zefir's Flashy Cooler");
    tray.setContextMenu(contextMenuInactive);
}

app.on("window-all-closed", () => {
    exit();
    app.exit(0);
    // log();
});

function requestConfig() {
    mainWindow.webContents.send("settings:receiveConfig", config);
}

function requestVersion() {
    mainWindow.webContents.send("settings:receiveVersion", appVersion + "@" + releaseType);
}

function requestHealth() {
    mainWindow.webContents.send("settings:receiveHealth", [iCueRunning, libreRunning]);
}

function requestThemeFolder() {
    mainWindow.webContents.send("settings:receiveThemeFolder", path.join(__dirname, "themes"))
}

function openThemeFolder() {
    exec("start " + path.join(__dirname, "themes"));
}

function exit() {
    themeList.forEach(theme => {
        if (theme.isActive) {
            config.defaultThemePath = theme.path;
        }
    });
    const finalConfig = JSON.stringify(config);
    // fs.writeFileSync(path.join(__dirname, "app.config.json"), config);
    // console.log(config);
    worker.postMessage("exit");
}

function startRendering() {
    tray.setContextMenu(contextMenuActive);
    mainWindow.webContents.send("rendering", 1);
    worker.postMessage("start"); 
    rendering = true;
}

function stopRendering() {
    tray.setContextMenu(contextMenuInactive);
    mainWindow.webContents.send("rendering", 0);
    worker.postMessage("stop");
    rendering = false;
}

function getThemeList(){
    themeList.forEach(theme => {
        mainWindow.webContents.send("theme", theme);
    });
}

async function handleFileOpen() {
    const { canceled, filePaths } = await dialog.showOpenDialog()
    if (canceled) {
        return
    } else {
        return filePaths[0]
    }
}


function selectTheme(_event, themeId) {
    const found = themeList.find(element => element.id == themeId);
    themeList.forEach(item => {
        if(item.id == themeId){
            item.isActive=true;
            config.defaultThemePath = item.path;
            fs.writeFileSync(path.join(__dirname, "app.config.json"), JSON.stringify(config));
            config = JSON.parse(fs.readFileSync(path.join(__dirname, "app.config.json")));
        } else {
            item.isActive=false;
        }
    });
    worker.postMessage(found.path);
    if(rendering) {
        startRendering()
    }
}

function configSendback(_event, config){
    const toWrite = JSON.stringify(config);
    fs.writeFileSync(path.join(__dirname, "app.config.json"), toWrite);
}

function renderStatus() {
    if(rendering) {
        mainWindow.webContents.send("rendering", 1);
    } else {
        mainWindow.webContents.send("rendering", 0);
    }
}

function applyParameters(_event, parameters) {
    themeList.forEach(item => {
        if(item.isActive){
            const configTheme = JSON.parse(fs.readFileSync(item.configPath));
            Object.keys(configTheme).forEach(key => {
                parameters.forEach(parameter => {
                    if(parameter.varName == key) {
                        configTheme[key] = parameter.value;
                    }
                });
                Object.keys(item.controllableParameters).forEach(localParameter => {
                    parameters.forEach(parameter => {
                        if(item.controllableParameters[localParameter]["varName"] == parameter.varName) {
                            item.controllableParameters[localParameter]["value"] = parameter.value;
                        }
                });
            })
            });
            const finalConfig = JSON.stringify(configTheme);
            fs.writeFileSync(item.configPath, finalConfig);
            const theme = require(item.path);
            item.preview = "data:image/jpeg;base64," + theme.renderPreview();
            worker.postMessage("exit");
            sleep(300);
            worker = new Worker("./libraries/renderer.js", { workerData: {renderPath: item.path, fps: fps} }); // worker needs to be destroyed for on the fly editing to work
            if(rendering) {
                startRendering();
            }
        }
    });
}

function sleep(milliseconds) {
    const date = Date.now();
    let currentDate = null;
    do {
      currentDate = Date.now();
    } while (currentDate - date < milliseconds);
  }