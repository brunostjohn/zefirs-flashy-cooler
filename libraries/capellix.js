var HID = require('node-hid');

class LCD {
    constructor() {
        this.vid = 0x1b1c;
        this.pid = 0x0c39;
        this.device = new HID.HID(this.vid, this.pid);
        this.device.on('data', function(data) {console.log(data)} )
        this.device.on("error", function(error) { console.log(error); });
        this.framebufReset(); 
        this.framebufResetter = setInterval(this.framebufReset.bind(this), 360000);
    }

    framebufReset() {
        // this somehow works, i have literally no idea how or why but it is the best solution for now
        this.device.sendFeatureReport([0x03, 0x0D, 0x01, 0x01, 0x78, 0x00, 0xC0, 0x03, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF]); // or this one?
        this.device.sendFeatureReport([0x03, 0x01, 0x64, 0x01, 0x78, 0x00, 0xC0, 0x03, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF, 0x2F, 0x2F, 0x2F, 0xFF]); // maybe this packet resets the fb?
        this.sleep(100);
    }

    exit() {
        this.device.close();
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
                try {this.device.sendFeatureReport(unfuckPacket);} catch {failedPacket = false; this.reconstructUSB();}
                console.log("unfucking!");
                this.sleep(6000);
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