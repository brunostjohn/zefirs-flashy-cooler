const { workerData, parentPort } = require("worker_threads");
const lcd = require("./display.js");
let themeScript = require(workerData.renderPath);
const { createCanvas, Image } = require("@napi-rs/canvas");

LCD = new lcd.LCD(workerData.availableDevice.devicePlugin);

class Renderer {
  constructor(workerData, LCD) {
    this.timer;
    this.LCD = LCD;
    this.frametime = 1000 / workerData.fps;
    this.lasttime = Date.now();
    this.deviceState = false;
    this.rotation = (workerData.rotation * Math.PI) / 180;
    this.width = workerData.availableDevice.width;
    this.height = workerData.availableDevice.height;
    this.canvas =
      this.rotation != 0 || this.width != 480 || this.height != 480
        ? createCanvas(this.width, this.height)
        : null;
    this.context =
      this.rotation != 0 || this.width != 480 || this.height != 480
        ? this.canvas.getContext("2d")
        : null;
    this.toChange =
      this.rotation != 0 || this.width != 480 || this.height != 480
        ? new Image()
        : null;
  }

  startRendering() {
    if (!this.deviceState) {
      this.LCD.openDevice();
      this.deviceState = true;
    }
    if (this.rotation != 0) {
      this.context.translate(this.canvas.width / 2, this.canvas.height / 2);
      this.context.rotate(this.rotation);
    }
    this.timer =
      this.canvas == null
        ? setInterval(this.render.bind(this), this.frametime)
        : setInterval(this.renderWithChanges.bind(this), this.frametime);
  }

  stopRendering(shouldBeClosed) {
    if (shouldBeClosed) {
      this.LCD.closeDevice();
      this.deviceState = false;
    }
    clearInterval(this.timer);
  }

  render() {
    this.LCD.sendFrame(themeScript.renderFrame().toString("base64"));
  }

  renderWithChanges() {
    this.context.clearRect(0, 0, this.width, this.height);
    const frame = themeScript.renderFrame();
    this.toChange.src = frame;
    if (this.rotation != 0) {
      this.context.drawImage(
        this.toChange,
        -this.toChange.width / 2,
        -this.toChange.width / 2,
        this.width,
        this.height
      );
    } else {
      this.context.drawImage(this.toChange, 0, 0, this.width, this.height);
    }
    const toRender = this.canvas.toBuffer("image/jpeg").toString("base64");
    this.LCD.sendFrame(toRender);
  }

  setFramerate(framerate) {
    this.frametime = 1000 / framerate;
  }

  changeTheme(path) {
    themeScript = require(path);
  }

  wrapUp() {
    if (typeof themeScript.wrapUp === "function") {
      themeScript.wrapUp();
    }
  }
}

renderer = new Renderer(workerData, LCD);

parentPort.on("message", (message) => {
  if (message == "stop") {
    renderer.stopRendering(false);
  } else if (message == "start") {
    renderer.startRendering();
  } else if (message == "exit") {
    renderer.stopRendering(true);
    renderer.wrapUp();
    parentPort.close();
  } else if (Number.isInteger(message)) {
    renderer.stopRendering(false);
    renderer.setFramerate(message);
  } else {
    renderer.stopRendering(false);
    renderer.changeTheme(message);
  }
});
