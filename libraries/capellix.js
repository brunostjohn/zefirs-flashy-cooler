var HID = require("node-hid");
const native = require("./native-backend.node");

class LCD {
  constructor() {
    this.framebufResetter = setInterval(this.framebufReset.bind(this), 360000);
  }

  framebufReset() {
    native.reset_fb();
    this.sleep(50);
  }

  exit() {
    // this.device.sendFeatureReport([0x03, 0x13, 0x40, 0x01, 0x57, 0x00, 0x84, 0x03, 0xc1, 0x57, 0xef, 0x03, 0x0c, 0x31, 0x80, 0x2a, 0x71, 0xec, 0x32, 0x31, 0x55, 0x35, 0x3f, 0xd9, 0xef, 0xf6, 0x73, 0x78, 0xbc, 0xfb, 0xff, 0x00]);
    clearInterval(this.framebufResetter);
  }

  sleep(milliseconds) {
    const date = Date.now();
    let currentDate = null;
    do {
      currentDate = Date.now();
    } while (currentDate - date < milliseconds);
  }

  sendFrame(image_as_base64) {
    let result;
    try {
      result = native.capellix_send_frame(image_as_base64);
    } catch {
      result = 1024;
      this.reconstructUSB();
    }
  }
}

module.exports = { LCD };
