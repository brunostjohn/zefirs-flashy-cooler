const { contextBridge, ipcRenderer } = require("electron");

contextBridge.exposeInMainWorld("electronAPI", {
  startRendering: () => ipcRenderer.invoke("renderer:startRendering"),
  stopRendering: () => ipcRenderer.invoke("renderer:stopRendering"),
  renderStatus: () => ipcRenderer.invoke("renderer:renderStatus"),
  updateRenderStatus: (callback) => ipcRenderer.on("rendering", callback),
  acceptFps: (callback) => ipcRenderer.on("fps", callback),
  getThemeList: () => ipcRenderer.invoke("themes:getThemeList"),
  receiveThemeList: (callback) => ipcRenderer.on("theme", callback),
  themeSelected: (themeId) => ipcRenderer.send("themes:themeSelected", themeId),
  parametersSendback: (parameters) =>
    ipcRenderer.send("renderer:parameterTransfer", parameters),
  openFile: () => ipcRenderer.invoke("global:openFile"),
  requestSensorInfo: () => ipcRenderer.invoke("renderer:sensorInfo"),
  requestConfig: () => ipcRenderer.invoke("settings:requestConfig"),
  requestVersion: () => ipcRenderer.invoke("settings:requestVersion"),
  requestHealth: () => ipcRenderer.invoke("settings:requestHealth"),
  receiveVersion: (callback) =>
    ipcRenderer.on("settings:receiveVersion", callback),
  receiveConfig: (callback) =>
    ipcRenderer.on("settings:receiveConfig", callback),
  receiveHealth: (callback) =>
    ipcRenderer.on("settings:receiveHealth", callback),
  configSendback: (config) =>
    ipcRenderer.send("settings:configSendback", config),
  requestThemeFolder: () => ipcRenderer.invoke("settings:requestThemeFolder"),
  receiveThemeFolder: (themeFolder) =>
    ipcRenderer.on("settings:receiveThemeFolder", themeFolder),
  openThemeFolder: () => ipcRenderer.invoke("settings:openThemeFolder"),
  receiveSensorInfo: (callback) =>
    ipcRenderer.on("renderer:receiveSensorInfo", callback),
  requestDeviceInfo: () => ipcRenderer.invoke("device:requestDeviceInfo"),
  receiveDeviceInfo: (callback) =>
    ipcRenderer.on("device:receiveDeviceInfo", callback),
  updatePreview: () => ipcRenderer.invoke("renderer:updatePreview"),
  receiveFreshPreview: (callback) =>
    ipcRenderer.on("renderer:receiveFreshPreview", callback),
  closeWindow: () => ipcRenderer.invoke("main:closeWindow"),
  minimiseWindow: () => ipcRenderer.invoke("main:minimiseWindow"),
  closeApp: () => ipcRenderer.invoke("main:killApp"),
  threeDivs: (callback) => ipcRenderer.on("themes:addAdjust", callback),
});
