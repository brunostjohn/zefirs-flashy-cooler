import usb
import usb.backend.libusb1
import codecs
from datetime import datetime, timedelta
from math import ceil
from time import sleep

class LCD:
    def __init__(self):
        self.__vendor_id = 0x1b1c
        self.__product_id = 0x0c39
        self.__device = usb.core.find(idVendor=self.__vendor_id, idProduct=self.__product_id)

        if self.__device is None:
            raise ValueError("LCD not found!")

        self.__device.set_configuration()

        self.__config = self.__device.get_active_configuration()
        self.__interface = self.__config[(0, 0)]
        self.__endpoint = usb.util.find_descriptor(self.__interface, custom_match=lambda e: usb.util.endpoint_direction(
            e.bEndpointAddress) == usb.util.ENDPOINT_OUT)

        if self.__endpoint is None:
            raise ValueError("LCD out endpoint not found!")

        self.__last_time = datetime.now()
        self.__max_fps = 25
        self.__frametime_ms = 1000 / self.__max_fps

        print("Config:\n", self.__endpoint, "\nFrametime: ", self.__frametime_ms)

    def send_packet(self, data_length, data, packets_sent, signature):
        packet = b"".join([b"\x02", b"\x05", b"\x40", signature, packets_sent.to_bytes(1, byteorder="big"), b"\x00", (data_length >> 8 & 0xFF).to_bytes(1, byteorder="big"), (data_length & 0xFF).to_bytes(1, byteorder="big"), data])
        if len(packet)<1024:
            packet += b"\x00"*(1024-len(packet))
        self.__endpoint.write(packet)

    def send_packet_raw(self, data):
        self.__endpoint.write(data)

    def send_static_image(self, image_path):
        loop_start=datetime.now()
        if datetime.now() - self.__last_time < timedelta(milliseconds=self.__frametime_ms):
            # making sure we dont get weird glitches by bombarding the display with jpegs
            return
        self.__last_time = datetime.now()
        image = ""
        with open(image_path, "rb") as f:
            image = f.read().hex()
        packets_sent = 0
        packets_to_be_sent = [image[i:i+1016*2] for i in range(0, len(image), 1016*2)]
        for i in packets_to_be_sent:
            if len(i) < 1016:
                self.send_packet(len(i), bytes.fromhex(i), packets_sent, b"\x01")
            else:
                self.send_packet(len(i), bytes.fromhex(i), packets_sent, b"\x00")
            packets_sent += 1
        timediff = datetime.now() - loop_start
        if timediff < timedelta(milliseconds=self.__frametime_ms):
            sleep((timedelta(milliseconds=self.__frametime_ms) - timediff).total_seconds()) # waiting for next frame moment in case we rendered prematurely
            


if __name__ == "__main__":
    lcd = LCD()
    while True:
        lcd.send_static_image("1.jpeg")
