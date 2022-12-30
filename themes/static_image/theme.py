from libraries.theme import Theme

class Static_Image(Theme):
    def __init__(self, image_path):
        super().__init__
        self.__image_path = image_path
        self.__framerate = 1.0
    def get_frame(self):
        image = ""
        with open(self.__image_path, "rb") as f:
            image = f.read().hex()
        return image
    def get_framerate(self):
        return self.__framerate