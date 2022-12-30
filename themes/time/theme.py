import io
from libraries.theme import Theme
from PIL import Image, ImageDraw, ImageFont
from datetime import datetime

class Time(Theme):
    def __init__(self, image_path):
        super().__init__
        self.__font = ImageFont.truetype(font="./themes/time/font.otf", size=50)
    def get_frame(self):
        image = ""
        size = (width, height) = 480, 480
        img = Image.new("RGB", size, (255, 255, 255))
        draw = ImageDraw.Draw(img)
        draw.text((130,200), str(datetime.now().strftime("%H:%M:%S")), (0,0,0), font=self.__font)
        image = self.image_to_byte_array(img)
        return image
    def image_to_byte_array(self, image: Image) -> bytes:
        imgByteArr = io.BytesIO()
        image.save(imgByteArr, format="JPEG")
        imgByteArr = imgByteArr.getvalue()
        return imgByteArr.hex()
    def get_framerate(self):
        return 10.0
        