const {contextBridge, ipcRenderer} = require("electron");

contextBridge.exposeInMainWorld("electronAPI", {
    startRendering: () => ipcRenderer.invoke("renderer:startRendering"),
    stopRendering: () => ipcRenderer.invoke("renderer:stopRendering"),
    renderStatus: () => ipcRenderer.invoke("renderer:renderStatus"),
    updateRenderStatus: (callback) => ipcRenderer.on("rendering", callback),
    acceptFps: (callback) => ipcRenderer.on("fps", callback),
    getThemeList: () => ipcRenderer.invoke("themes:getThemeList"),
    receiveThemeList: (callback) => ipcRenderer.on("theme", callback),
    themeSelected: (themeId) => ipcRenderer.send("themes:themeSelected", themeId),
    parametersSendback: (parameters) => ipcRenderer.send("renderer:parameterTransfer", parameters),
    openFile: () => ipcRenderer.invoke("global:openFile")
})