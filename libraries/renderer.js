const {workerData, parentPort} = require("worker_threads");
const lcd =  require("./capellix.js");
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

    stopRendering(bul){
        if (bul) this.LCD.exit();
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

    wrapUp(){
        if(typeof themeScript.wrapUp === "function"){
            themeScript.wrapUp();
        }
    }
}

renderer = new Renderer(workerData, LCD)

parentPort.on("message", message => {
    if(message=="stop"){
        renderer.stopRendering(false);
        // usbDetect.stopMonitoring();
    } else if(message=="start"){
        renderer.startRendering();
        // usbDetect.startMonitoring();
    } else if(message=="exit"){
        renderer.stopRendering(true);
        renderer.wrapUp();
        parentPort.close();
    } else if(Number.isInteger(message)) {
        renderer.stopRendering(false);
        renderer.setFramerate(message);
    } else {
        renderer.stopRendering(false);
        renderer.changeTheme(message);
    }
});