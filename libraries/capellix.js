// const {usb, findByIds, Transfer, transfer} = require("usb");
var HID = require('node-hid');

class LCD {
    constructor() {
        // usb.setDebugLevel(3);
        this.vid = 0x1b1c;
        this.pid = 0x0c39;
        // this.device = findByIds(vid, pid);
        // this.device.open(true);
        // this.device.interface(0).claim();
        // this.endpoint = this.device.interfaces[0].endpoints[1];
        this.device = new HID.HID(this.vid, this.pid);
        this.device.on('data', function(data) {console.log(data)} )
        this.device.on("error", function(error) { console.log(error); });
        // this.device.sendFeatureReport([0x03, 0x0D, 0x01, 0x01, 0x78, 0x00, 0xC0, 0x03, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF]);
        this.framebufReset(); 
        this.framebufResetter = setTimeout(this.framebufReset.bind(this), 60000);
    }

    framebufReset() {
        this.device.sendFeatureReport([0x03, 0x0D, 0x01, 0x01, 0x78, 0x00, 0xC0, 0x03, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF]); // or this one?
        this.device.sendFeatureReport([0x03, 0x01, 0x64, 0x01, 0x78, 0x00, 0xC0, 0x03, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF]); // maybe this packet resets the fb?
        // this.device.pause(30);
    }

    exit() {
        this.device.close();
        clearTimeout(this.framebufResetter);
    }

    sleep(milliseconds) {
        const date = Date.now();
        let currentDate = null;
        do {
          currentDate = Date.now();
        } while (currentDate - date < milliseconds);
      }

    sendFrame(image_as_base64) {
        const image = Buffer.from(image_as_base64, "base64");
        let chunks = [];
        if(image.length > 1016) {
            for (let i = 0; i < image.length; i += 1016) {
                const chunk = image.subarray(i, i + 1016).toJSON().data;
                chunks.push(chunk);
            }
        } else {
            chunks.push(image);
        }
        // console.log("jpeg size: " + image.length +" chunk length: " + chunks.length);
        let packetsSent = 0;
        let packet;
        let signature = 0x00;
        let zeropad;
        for (const chunk of chunks) {
            if (chunk.length<1016) {
                zeropad = new Array(1016-chunk.length).fill(0x00);
                zeropad.fill(0x00);
                signature = 0x01;
            }
            if (chunk.length == 1016) {
                signature = 0x00;
            }
            // packet = Buffer([0x02, 0x05, 0x40, signature, packetsSent, 0x00, (chunk.length >> 8 & 0xFF), (chunk.length & 0xFF)]);
            packet = [0x02, 0x05, 0x40, signature, packetsSent, 0x00, (chunk.length >> 8 & 0xFF), (chunk.length & 0xFF)];
            let finalPacket = packet.concat(chunk)
            if (chunk.length<1016) {
                finalPacket = finalPacket.concat(finalPacket, zeropad);
            }
            packetsSent++;
            // this.endpoint.transfer(finalPacket);
            let result;
            try { result = this.device.write(finalPacket); } catch {
                result = 1024;
                this.reconstructUSB();
            }
            let failedPacket;
            if (result != 1024) {
                failedPacket = true;
                console.log("fuckery is afoot");
            }

            if (failedPacket && signature) {
                let unfuckPacket = [0x03, 0x19, 0x40, signature, packetsSent, 0x00, (chunk.length >> 8 & 0xFF), (chunk.length & 0xFF)];
                unfuckPacket = unfuckPacket.concat(chunk.splice(0, 24));
                this.device.sendFeatureReport(unfuckPacket);
                console.log("unfucking!");
                this.device.pause(6000);
                this.device.resume();
                failedPacket = false;
            }
        }
    }
    reconstructUSB(){
        this.sleep(7000);
        this.device = new HID.HID(this.vid, this.pid);
    }
}

module.exports = {LCD};