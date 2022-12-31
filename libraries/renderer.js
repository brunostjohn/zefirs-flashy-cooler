const {workerData, parentPort} = require("worker_threads");
const lcd =  require("./capellix.js");
const themeScript = require(workerData.renderPath);

LCD = new lcd.LCD();

class Renderer{
    constructor(workerData){
        this.timer;
        this.frametime = 1000/workerData.fps;
    }

    startRendering(){
        this.timer = setInterval(this.render, this.frametime);
    }

    stopRendering(){
        clearTimeout(this.timer);
    }

    render(){
        LCD.sendFrame(themeScript.renderFrame());
    }
}

renderer = new Renderer(workerData)

parentPort.on("message", message => {
    if(message=="stop"){
        renderer.stopRendering();
    } else if(message=="start"){
        renderer.startRendering();
    } else if(message=="exit"){
        renderer.stopRendering();
        parentPort.close();
    }
});