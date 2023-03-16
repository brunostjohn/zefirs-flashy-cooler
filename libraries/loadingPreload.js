const { contextBridge, ipcRenderer } = require("electron");
contextBridge.exposeInMainWorld("electronAPI", {
  requestVersion: () => ipcRenderer.invoke("loading:requestVersion"),
  receiveVersion: (callback) =>
    ipcRenderer.on("loading:receiveVersion", callback),
  receiveConsole: (callback) =>
    ipcRenderer.on("loading:receiveConsole", callback),
  triggerError: (callback) => ipcRenderer.on("loading:error", callback),
  closeApp: () => ipcRenderer.invoke("loading:closeApp"),
});
