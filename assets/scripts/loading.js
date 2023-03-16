window.addEventListener("load", (event) => {
  window.electronAPI.requestVersion();
});

const version = document.getElementById("version");

window.electronAPI.receiveVersion((_event, value) => {
  version.textContent = "Version: " + value;
});

let exT;

window.electronAPI.triggerError((_event, value) => {
  const cont = document.getElementById("error");
  cont.innerText = value[0];
  const button = document.getElementById("modalButton");
  button.click();
  exT = value[1];
});

const consoleCode = document.getElementById("output");

window.electronAPI.receiveConsole((_event, value) => {
  consoleCode.textContent += value + "\n";
});

function showTraceback() {
  const houdini = document.getElementById("houdini");
  const ex = document.getElementById("ex");
  const btn = document.getElementById("trace");
  if (houdini.style.display == "none") {
    houdini.style.display = "block";
    ex.innerText = exT;
    btn.textContent = "Hide traceback";
  } else {
    houdini.style.display = "none";
    ex.innerText = exT;
    btn.innerText = "Show traceback";
  }
}
