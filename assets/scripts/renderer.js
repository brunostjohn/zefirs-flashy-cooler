const btn = document.getElementById("btn");

window.addEventListener("load", (event) => {
    window.electronAPI.renderStatus();
    window.electronAPI.getThemeList();
})

    
window.electronAPI.updateRenderStatus((_event, value) => {
    if (value == 1){
        btn.textContent = "Stop Rendering";
        btn.classList.remove("btn-success");
        btn.classList.add("btn-danger");
    } else if (value==0) {
        btn.textContent = "Start Rendering";
        btn.classList.remove("btn-danger");
        btn.classList.add("btn-success");    
    }
});

btn.addEventListener("click", () => {
    if (btn.textContent.toLowerCase().includes("Start Rendering".toLowerCase())) {
        window.electronAPI.startRendering()
        btn.textContent = "Stop Rendering";
      } else {
        window.electronAPI.stopRendering()
        btn.textContent = "Start Rendering";
      }
})

window.electronAPI.receiveThemeList((_event, theme) => {
    if(theme.isActive) {
        document.getElementById("preview").src=theme.preview;
        document.getElementById("title").innerText=theme.title;
    }
});

window.electronAPI.acceptFps((_event, fps) => {
    document.getElementById("limit").innerText = "Actual FPS: " + Math.round(fps) + " fps";
});