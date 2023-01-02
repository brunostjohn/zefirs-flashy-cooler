const {contextBridge, ipcRenderer} = require("electron");

contextBridge.exposeInMainWorld("electronAPI", {
    startRendering: () => ipcRenderer.invoke("renderer:startRendering"),
    stopRendering: () => ipcRenderer.invoke("renderer:stopRendering"),
    updateRenderStatus: (callback) => ipcRenderer.on("rendering", callback),
    getThemeList: () => ipcRenderer.invoke("themes:getThemeList"),
    receiveThemeList: (callback) => ipcRenderer.on("theme", callback)
})