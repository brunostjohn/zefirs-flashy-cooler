const btn = document.getElementById("btn");

window.electronAPI.updateRenderStatus((_event, value) => {
    if (value == 1){
        btn.textContent = "Stop Rendering";
    } else if (value==0) {
        btn.textContent = "Start Rendering";    
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