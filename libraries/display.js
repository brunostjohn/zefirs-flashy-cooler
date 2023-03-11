class LCD {
  constructor(devicePluginPath) {
    this.native = require(devicePluginPath);
  }

  openDevice() {
    this.native.open_device();
  }

  closeDevice() {
    this.native.close_device();
  }

  sendFrame(image_as_base64) {
    this.native.send_frame(image_as_base64);
  }
}

module.exports = { LCD };
