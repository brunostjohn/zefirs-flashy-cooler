window.addEventListener("load", (event) => {
  window.electronAPI.requestDeviceInfo();
  changeFPSThing();
});

window.electronAPI.receiveDeviceInfo((_event, value) => {
  const vidField = document.getElementById("vid");
  vidField.textContent += value.vendorId;
  const pidField = document.getElementById("pid");
  pidField.textContent += value.productId;
  const image = document.getElementById("preview");
  image.src = value.deviceImage;
  const name = document.getElementById("title");
  name.textContent = value.deviceName;
});

function changeFPSThing() {
  const slideyThing = document.getElementById("fpsInput");
  const labelThing = document.getElementById("fpsLabel");
  labelThing.textContent = "Current value: " + slideyThing.value;
}
