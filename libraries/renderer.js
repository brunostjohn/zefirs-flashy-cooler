const {workerData, parentPort} = require("worker_threads");
const lcd =  require("./capellix.js");
// const usbDetect = require("usb-detection");
let themeScript = require(workerData.renderPath);

LCD = new lcd.LCD();

class Renderer{
    constructor(workerData, LCD){
        this.timer;
        this.LCD = LCD;
        this.frametime = 1000/workerData.fps;
        this.lasttime = Date.now();
    }

    startRendering(){
        this.timer = setInterval(this.render, this.frametime);
    }

    stopRendering(){
        this.LCD.exit();
        clearInterval(this.timer);
    }

    render(){
        LCD.sendFrame(themeScript.renderFrame());
    }
    setFramerate(framerate){
        this.frametime=1000/framerate;
    }
    changeTheme(path) {
        themeScript = require(path);
    }
}

renderer = new Renderer(workerData, LCD)

// usbDetect.on('add:6940:3129', LCD.reconstructUSB );

parentPort.on("message", message => {
    if(message=="stop"){
        renderer.stopRendering();
        // usbDetect.stopMonitoring();
    } else if(message=="start"){
        renderer.startRendering();
        // usbDetect.startMonitoring();
    } else if(message=="exit"){
        renderer.stopRendering();
        parentPort.close();
        process.exit();
    } else if(Number.isInteger(message)) {
        renderer.stopRendering();
        renderer.setFramerate(message);
    } else {
        renderer.stopRendering();
        renderer.changeTheme(message);
    }
});