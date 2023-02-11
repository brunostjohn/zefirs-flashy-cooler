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
    var result           = '';
    var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
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
    { label: 'Open CapellixLCD', click() { createWindow(); }},
    { label: 'Start Rendering', click() {startRendering()} },
    { label: 'Quit CapellixLCD', click() { exit(); } }
])

const contextMenuActive = Menu.buildFromTemplate([
    { label: 'Open CapellixLCD', click() { createWindow(); }},
    { label: 'Stop Rendering', click() {stopRendering();} },
    { label: 'Quit CapellixLCD', click() { exit(); } }
])

const createTray = () => {
    const icon = nativeImage.createFromDataURL('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACQAAAAkCAYAAADhAJiYAAAAAXNSR0IArs4c6QAAAAlwSFlzAAALEwAACxMBAJqcGAAAAVlpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IlhNUCBDb3JlIDUuNC4wIj4KICAgPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4KICAgICAgPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIKICAgICAgICAgICAgeG1sbnM6dGlmZj0iaHR0cDovL25zLmFkb2JlLmNvbS90aWZmLzEuMC8iPgogICAgICAgICA8dGlmZjpPcmllbnRhdGlvbj4xPC90aWZmOk9yaWVudGF0aW9uPgogICAgICA8L3JkZjpEZXNjcmlwdGlvbj4KICAgPC9yZGY6UkRGPgo8L3g6eG1wbWV0YT4KTMInWQAACsZJREFUWAmtWFlsXFcZ/u82++Jt7IyT2Em6ZFHTpAtWIzspEgjEUhA8VNAiIYEQUvuABBIUwUMkQIVKPCIoEiABLShISEBbhFJwIGRpIKRpbNeJ7bh2HHvssR3PPnPnLnzfmRlju6EQqUc+c++c8y/fv54z1uQOh+/7Glh0TD59TE/TND7lnfa4/64OKsM071QoeZpA/y9WWvk/B4XCC06TUC+Xyw8HTXNQ1+Ww6PpOrMebewXxvBueJ6/XHOdMJBL5J9Y97m2R0SS/wweE6JxkGx5dilWr1S/7dXsEa2o4+LyFmcFcaL5zbX3Y9gh5hpeWYpSB9XV5/H678V89BGYDXnHJlCsWn4gHrGc1K9CXxferOdvPOOKUfF8cH7nUyCtklQZXih/VNNlmirk3GdBSoIcRswW7/vVkLPYi5W2Uze8bh7J+4wLfh4dViFx5/nmrUi7/MhGNvrCkBfpeWqnW/7BUdadqntQ8zwr6vhUV34xpYnDynWvcmwQNaclDXsqgLMqkocPDw7fNx7d5qIX+/PmJxKGD6VdDkeh7ztyqOFfrokGCEWiiZ1mp0uITnuKAosaT7+pNxMYTyefutcQfbA+b1XLpH5fnF97/yD335Fu6mqTqsclDINBVmI4fDxw80KPAvJSt1MZtMcLiGxYUu83p4UkgnJZlqcl3LAj3WnTkIS9lUBYNPJjueVWgg7qocyOgliFqjZsg8gq5tRdiieQTf1gq15Y8CUbRZtyWOzZwc8lEqS3PTCtgqd13ieO68BQ2uNl64tXAewktrFuX2mPdkWAxn3sxnmx7sqUTJGqso8MGS9tbXFz8DMH8bblUX3T9QARVi8RV8qljfcJy0zRlaf6mzHEuzEtmekqCoZB4rqp0OmudHtUnlEWZlE0d1EWd1N3EozourcO65pw4eTIZQTW9VazJtbqvw9XwKVFQMsKDBuNhtp4uvGGFI+IDgKnpMjYyIis3ZsQMBIR7pONsIaMsyqRs6ohY1rPUSd3EQFDqo+kdZ3Fh4aupbdu+99uFQr2A1CBs4uEAjZjIFUMHi4dVxMXzCdCXQj4vBrwVCofl0ulTcv/DAxJJJBUPc8mpoyI2JDw7bFyT+ifTcSubyXytJ51+roWBxwG9Q73WWjZ7eSUU3//nXM0NI+x0PBGrTSgsLS9JFuFxHFrvSqIrJV279gi6tjiVspTza3JjZhY+0CQZj0mlWJSeHTslCro6eFqymCcVVN77kkGjs1p4sy2VOoSlOrFwT+XR+PjkgGaZ+ycKVbRTYUdVrmaImCvzk1dlFCEJdHRJ284+ie/ol0h7p7jFvExcvCCXzp2Rqem3pAMAiqWS6JGYhFI9Mjo6KjevXVUyKEuFHrKpY6JQ8TXT3D8+OTkAHBw6o6LCFo9ag3o4JtlCyTHEt5AxKvS6YUi5kJeZG3Py0NAxlLcJ9xti+K7Mjo/JfGZRuvv6Ze+9+yWEhDZAvzg3JyhX2d6/S7q6e+TimdOS7ElLKBZDwqvmj6rztayr1fVI1IoXi4PAcYZY1tPEEO1wEVlXgRFBDcmIXTqJsS+XyhKLJ5A/OpIVXXptWUYv/UvaenfIocEhMQ2EzHHErlXFCgQl3paU1eVl6QAY8sQTCSmVihKJx1V/ogvgIYF/pACdcMBhqONoHhF88/2d+bojyA6cRvje2IdFjoSjUSnBS8hgyS9lZOzKFdmPxO3o6gQIGzwuDn1dVSCtCKPy1pZXlATXqUsVYMLRmKo87vP4Y1ioqwCdCegmMYx3W/VPn8RrSDwwIMMbcEjkYo29JZVOy+ybI7K4eksODx1VSqvligpReSVLgySM/FI5h2q062jNyL3s7FtoAyGJIlx1225UmwJF6aJRJ3XzHXO9bWvsJa3jQFlBJkz6iuXdu32HzM7MyP0PPNgAU6ko4Qzp6b+flr8MD9OYJg9CwtzL5+T65ITs2bsP3mGxN/ZbBcOn0sk20gAkLQ+huXpFi8vkoY9AoyDjxTR1mbo6Ltt275HpN0dlNxQE40mVM8Ajjxx9VAGhAvQR1akZFCq799ADysMuQqOxh2FNmamEaz51ItGLfFD9+oUJoZkLowHoFA2mljUacqOMflKuVmHpfmnfvlMuvXZeStmMBIMhcWEdjgFJtrUjXI0KchAuAg0ilxLJNoRVBxhIBm0TjjKAuqjTqTs3CQZ6QUUMGFW7eiWMUg6w+yo8YMW7DqtqlZLkUDV2ISfd29KyDwk9MjYmMyOXxQIIKuShqo4VGFNBEgeDQYqVam5N5tEePFQgURIUBCsd1EWd1XrtDUUMLARD9bKaK5ytQ2Gb75g8WMiEP6VkfnZGevv6UF1vSBW5E0PFDAweFRvlfun8WVmamhDNrkmweQ0pwaPt6M4m8mgKTTFXqcrV0ZH1FKBg6qAu6qTuJiCV1Cp2Q0NDr9Uq5Ym+oMEDlSewsoRwrVBEaij7AJ4s7zrOpumxEdm15y6558GHJVe1Zezy6zJx6aJkpq5JFB4z6zVZmBiX1VWUP0IY4CFMYcpQdZ3xqIs6oftCE5DHKwd0q/tzOV8svdDb3nk8VnG9qmgQC0ZURz8Ur91alXgSByZ6ES9kZZTr/PR16UOCh+7dq0CWyyXJ4xqCQ0nKt9YQSlPue2gAeYZzD7yNLk0wmqAreb2WYSxAJ8Dget64wxtEBlDaqVOn/K5dB67t6+t5MhoMJuc8w8UPKiQ9CQR9JK5czhZAQxPt7TKF3OiAIisUViAD2Lg5d0P2HDgoKeRaW0enyqVwBJcO5fFG5dqa7h406qaeX8384uTZL5w9+UqxhYHFp0YLIYA9ddfu3T+4UJF6Rg+YAc9D0+RoIGP1ULhpWspr10evyK7+ftWTrk9PS/++A9KZSm26cih2mMOErem6n/ZsZwA2TM/MPHXs2LEftnSTbh0Q36mIIbx44cLvOnu3f+xUwbWLmoHTCUlF6g2jBQo/GnFrnGNqSHdvr+rIKGMW1KahwEBdzHft98aNwMr8zd8/NDDwccihc0hLi3GubRjY0Bm6H19fPvnZI4c/fHd7PJ2peXYZ+WQ26JufZELjQ6lbAQtnWre0d3apY8TFIdtAo+Qri6mupsB49lBMC+QXF0YefObZT8j0eKWlswVjEyCCOXHihPGb575VCvVuf3lvetsH9rXF0rla3cnhpoIGjgsUPhR3I4TMKYJQV1Z6WO02aEjHa5mNe3OPW3OPRHVrbXFh9Ocvv/KR1372owx1Pf3005uc35Ddgtd8rsf06IdS5777zZ+mUqmPzjm6TPpmvayZOq4LyATeCzkanmiy4qEuC/yXiO8CSMRzvLs1x9phepLNZl868sy3Pyen/5hd1/EfRvWmuvSWNeaRS/RkPDI4+NjE1NSXEoXlpaNB1zqo20abi59/vu/UfM2pie7WUDVq8l3wTwnskeZ+zTbIQ17KoCzKpGzq2KqX32/roRbh8ePHdUzl0s9/5Rv9n/7go19MxCKfCkZiu3V06wrO5gocxL7Dgd/IEobEMH6rejg+auXidL5Y/vWv/vTX53/y/e/MkGajTH7fOt4RUJOY1df4RdtY6ICFRzqTySOhUOA+3Ai3o31H1ZbnlXBruFmt2iMrudy5xx9//BzWV7nXDBGN2xpjbt/5oGUEdhtO3iD47xZOvm8a5CHvpsV38wsUaMwBWsz3rbK5xr0mzdv2t9Jv/f5vhsF4J+Q63IUAAAAASUVORK5CYII=')
    tray = new Tray(icon)

    tray.setToolTip('CapellixLCD');
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