window.addEventListener("load", (event) => {
  window.electronAPI.requestVersion();
});

const version = document.getElementById("version");

window.electronAPI.receiveVersion((_event, value) => {
  version.textContent = "Version: " + value;
});

window.electronAPI.triggerNoDeviceError((_event, value) => {
  const button = document.getElementById("modalButton");
  button.click();
});

const consoleCode = document.getElementById("output");

window.electronAPI.receiveConsole((_event, value) => {
  consoleCode.textContent += value + "\n";
});
