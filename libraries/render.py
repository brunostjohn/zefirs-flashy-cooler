from datetime import datetime, timedelta
from threading import Thread
from lcd import LCD
from theme import Theme
from time import sleep

class Renderer:
    def __init__(self, LCD: LCD, theme, smoother: bool = False):
        self.__LCD = LCD
        self.__theme = theme
        self.__is_thread = True
        self.__last_time = datetime.now()
        self.__smoother = smoother
        if self.__theme.get_framerate() is None:
            self.__internal_fps = self.__LCD.get_framerate()
            self.__internal_frametime = self.__LCD.get_frametime()
        else: # theme developer can now specify max fps if needed
            if type(self.__theme.get_framerate()) is not float:
                raise TypeError("Framerate needs to be a float.")
            if self.__theme.get_framerate() > 30 or self.__theme.get_framerate() < 1:
                raise ValueError("Framerate needs to be between 30 and 1.")
            self.__internal_fps = self.__theme.get_framerate()
            self.__internal_frametime = 1000/self.__internal_fps
    
    def __requestFrames(self):
        while self.__is_thread:
            loop_start=datetime.now()
            self.__last_time = datetime.now()
            image = self.__theme.get_frame()
            self.__LCD.send_frame(image)
            timediff = datetime.now() - loop_start
            if timediff < timedelta(milliseconds=self.__internal_frametime):
                sleep((timedelta(milliseconds=self.__internal_frametime) - timediff).total_seconds()) # waiting for next frame moment in case we rendered prematurely
        return
    
    def start_rendering(self):
        self.__thread = Thread(target=self.__requestFrames)
        self.__is_thread = True
        self.__thread.start()
    
    def stop_rendering(self):
        self.__is_thread = False
        
    def set_theme(self, theme: Theme):
        self.stop_rendering()
        self.__theme = theme
        if self.__theme.get_frametime() is None:
            self.__frametime = self.__LCD.get_frametime
        else:
            if type(self.__theme.get_frametime()) is not float:
                raise TypeError("Framerate needs to be a float.")
            if framerate > 30 or framerate < 1:
                raise ValueError("Framerate needs to be between 30 and 1.")
            self.__frametime = self.__theme.get_frametime
            self.__internal_fps = framerate
            self.__internal_frametime = 1000/self.__internal_fps
        self.start_rendering()
        
    def get_theme(self):
        return self.__theme

if __name__ == "__main__":
    import lcd
    lcd = LCD()
    theme = Theme()
    renderer = Renderer(lcd, theme)