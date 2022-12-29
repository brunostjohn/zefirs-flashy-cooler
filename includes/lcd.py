import usb
import usb.backend.libusb1
from datetime import datetime, timedelta

class LCD():
    def __init__(self):
        self.__vendor_id = 0x1b1c
        self.__product_id = 0x0c39
        self.__device = usb.core.find(idVendor = self.__vendor_id, idProduct = self.__product_id)
        
        if self.__device is None:
            raise ValueError("LCD not found!")
        
        self.__device.set_configuration()

        self.__config = self.__device.get_active_configuration()
        self.__interface = self.__config[(0, 0)]
        self.__endpoint = usb.util.find_descriptor(self.__interface, custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_OUT)
        
        if self.__endpoint is None: 
            raise ValueError("LCD out endpoint not found!")
        
        self.__last_time = datetime.now()
        self.__max_fps = 25
        self.__frametime_ms = 1000/self.__max_fps
        
        print("Config:\n", self.__endpoint, "\nFrametime: ",self.__frametime_ms)
    
    def send_packet(self, data):
        packet = [1024 * "\x00"]
        self.__endpoint.write("".join(packet))
        
    def send_packet_raw(self, data):
        self.__endpoint.write(data)
    
    def send_static_image(self, imagepath):
        if (datetime.now() - self.__last_time < timedelta(milliseconds=self.__frametime_ms)):
            # making sure we dont get weird glitches by bombarding the display with jpegs
            return
        self.__last_time = datetime.now()
        image = []
        with open(imagepath, "rb") as f:
            while(byte := f.read(1)):
                image.append(byte)
        bytes_left = len(image)
        packets_sent = 0
        
        while(bytes_left>0):
            bytes_awaiting = min(1016, bytes_left)
            if(bytes_awaiting<1016):
                finaldata = [bytes_left]
                finaldata.extend()
                self.send_packet()
        #lcd.send_packet(image)
        

if __name__ == "__main__":
    lcd = LCD()
    while True:
        lcd.send_static_image("image.jpeg")