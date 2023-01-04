// const {usb, findByIds, Transfer, transfer} = require("usb");
var HID = require('node-hid');

class LCD {
    constructor() {
        // usb.setDebugLevel(4);
        let vid = 0x1b1c;
        let pid = 0x0c39;
        // this.device = findByIds(vid, pid);
        // this.device.open(true);
        // this.device.interface(0).claim();
        // this.endpoint = this.device.interfaces[0].endpoints[1];
        this.device = new HID.HID(vid, pid);
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
            this.device.write(finalPacket);
        }
    }
    reconstructUSB(){
        this.device = findByIds(vid, pid);
        this.device.open(true);
        this.device.interface(0).claim();
        this.endpoint = this.device.interfaces[0].endpoints[1];
    }
}

module.exports = {LCD};